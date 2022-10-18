mod common;
mod error;
mod types;

use std::time::Duration;

use aws_sdk_route53::model::{
    AliasTarget, Change, ChangeAction, ChangeBatch, ResourceRecordSet, RrType,
};
use common::{CRUD_URI, DOMAIN_NAME, ELB_ZONE_ID, HOSTED_ZONE_ID};
use lambda_runtime::{service_fn, LambdaEvent};
use serde::Serialize;

use error::Error;
use types::{ConfigMessage, SqsMessageEvent};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ItemFailure {
    item_identifier: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    batch_item_failures: Vec<ItemFailure>,
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    lambda_runtime::run(service_fn(|event| async {
        dotenvy::dotenv().ok();
        let aws_config = aws_config::load_from_env().await;
        let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_config);
        let r53_client = aws_sdk_route53::Client::new(&aws_config);

        let http_client = reqwest::Client::builder()
            .use_rustls_tls()
            .connect_timeout(Duration::from_secs(2))
            .timeout(Duration::from_secs(2))
            .build()
            .unwrap();

        func(event, eb_client, r53_client, http_client).await
    }))
    .await?;
    Ok(())
}

async fn func(
    event: LambdaEvent<SqsMessageEvent>,
    eb_client: aws_sdk_elasticbeanstalk::Client,
    r53_client: aws_sdk_route53::Client,
    http_client: reqwest::Client,
) -> Result<Response, Error> {
    let (event, _context) = event.into_parts();

    let tasks = event
        .records
        .into_iter()
        .map(|record| {
            let message: ConfigMessage = serde_json::from_str(&record.body).unwrap();
            (
                tokio::spawn(handle_message(
                    message,
                    eb_client.clone(),
                    r53_client.clone(),
                    http_client.clone(),
                )),
                record.message_id,
            )
        })
        .collect::<Vec<_>>();

    let mut failures: Vec<ItemFailure> = Vec::new();

    // add failed tasks to failures so they can return to queue
    for task in tasks {
        match task.0.await {
            Ok(result) => match result {
                Err(err) => {
                    log::error!("Error ocurred processing message {}: {}", task.1, err);
                    failures.push(ItemFailure {
                        item_identifier: task.1,
                    })
                }
                _ => {}
            },
            Err(err) => {
                log::error!(
                    "Join error ocurred while processing message {}: {}",
                    task.1,
                    err
                );
                failures.push(ItemFailure {
                    item_identifier: task.1,
                })
            }
        }
    }

    Ok(Response {
        batch_item_failures: failures,
    })
}

async fn handle_message(
    message: ConfigMessage,
    eb_client: aws_sdk_elasticbeanstalk::Client,
    r53_client: aws_sdk_route53::Client,
    http_client: reqwest::Client,
) -> Result<(), Error> {
    let term_result = eb_client
        .terminate_environment()
        .environment_id(&message.env_id)
        .force_terminate(true)
        .send()
        .await;

    if let Ok(env) = term_result {
        r53_client
            .change_resource_record_sets()
            .hosted_zone_id(HOSTED_ZONE_ID)
            .change_batch(
                ChangeBatch::builder()
                    .changes(
                        Change::builder()
                            .action(ChangeAction::Delete)
                            .resource_record_set(
                                ResourceRecordSet::builder()
                                    .name(&format!("{}.{}", message.env_name, DOMAIN_NAME))
                                    .r#type(RrType::A)
                                    .alias_target(
                                        AliasTarget::builder()
                                            .dns_name(env.cname().unwrap())
                                            .evaluate_target_health(false)
                                            .hosted_zone_id(ELB_ZONE_ID)
                                            .build(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .send()
            .await
            .ok();
    }

    http_client
        .post(format!(
            "{}instances/{}/fail-callback",
            *CRUD_URI, message.instance_id
        ))
        .header("jwt", message.key)
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}
