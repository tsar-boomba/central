mod common;
mod error;
mod types;

use std::time::Duration;

use aws_lambda_events::sqs::SqsEvent;
use aws_sdk_route53::model::{
    AliasTarget, Change, ChangeAction, ChangeBatch, ResourceRecordSet, RrType,
};
use common::{CRUD_URI, DOMAIN_NAME, ELB_ZONE_ID, HOSTED_ZONE_ID};
use lambda_runtime::{service_fn, LambdaEvent};
use serde::Serialize;

use error::Error;
use types::FailMessage;

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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let service = service_fn(func);
    lambda_runtime::run(service).await?;
    Ok(())
}

async fn func(event: LambdaEvent<SqsEvent>) -> Result<Response, Error> {
    let aws_config = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_config);
    let r53_client = aws_sdk_route53::Client::new(&aws_config);

    let http_client = reqwest::Client::builder()
        .use_rustls_tls()
        .connect_timeout(Duration::from_secs(2))
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    let (event, _context) = event.into_parts();

    let tasks = event
        .records
        .into_iter()
        .filter(|record| record.body.is_some() && record.message_id.is_some())
        .map(|record| {
            let message: FailMessage = serde_json::from_str(&record.body.unwrap()).unwrap();
            (
                tokio::spawn(handle_message(
                    message,
                    eb_client.clone(),
                    r53_client.clone(),
                    http_client.clone(),
                )),
                record.message_id.unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut failures: Vec<ItemFailure> = Vec::new();

    // add failed tasks to failures so they can return to queue
    for task in tasks {
        match task.0.await {
            Ok(result) => match result {
                Err(err) => {
                    tracing::error!("Error ocurred processing message {}: {}", task.1, err);
                    failures.push(ItemFailure {
                        item_identifier: task.1,
                    })
                }
                _ => {}
            },
            Err(err) => {
                tracing::error!(
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
    message: FailMessage,
    eb_client: aws_sdk_elasticbeanstalk::Client,
    r53_client: aws_sdk_route53::Client,
    http_client: reqwest::Client,
) -> Result<(), Error> {
    if let Some(env_id) = message.env_id {
        if let Some(env_name) = message.env_name {
            let term_result = eb_client
                .terminate_environment()
                .environment_id(&env_id)
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
                                            .name(&format!("{}.{}", env_name, DOMAIN_NAME))
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
        }
    }

    http_client
        .post(format!(
            "{}instances/{}/fail-callback",
            *CRUD_URI, message.instance_id
        ))
        .header("jwt", message.jwt)
        .send()
        .await?
        .error_for_status()
        .map(|_| ())
        .map_err(|e| e.into())
}
