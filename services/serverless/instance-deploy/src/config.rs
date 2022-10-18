mod common;
mod error;
mod types;

use std::time::Duration;

use aws_sdk_elasticloadbalancingv2::model::{
    Action, ActionTypeEnum, Certificate, ProtocolEnum, RedirectActionConfig,
    RedirectActionStatusCodeEnum,
};
use aws_sdk_route53::model::{
    AliasTarget, Change, ChangeAction, ChangeBatch, ResourceRecordSet, RrType,
};
use lambda_runtime::{service_fn, LambdaEvent};
use serde::Serialize;

use common::{CRUD_URI, DOMAIN_NAME, ELB_ZONE_ID, HOSTED_ZONE_ID};
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
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    let service = service_fn(func);
    lambda_runtime::run(service).await?;
    Ok(())
}

async fn func(event: LambdaEvent<SqsMessageEvent>) -> Result<Response, Error> {
    tracing::info!("ev received");
    let aws_config = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_config);
    let elb_client = aws_sdk_elasticloadbalancingv2::Client::new(&aws_config);
    let r53_client = aws_sdk_route53::Client::new(&aws_config);
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);

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
        .map(|record| {
            let message: ConfigMessage = serde_json::from_str(&record.body).unwrap();
            (
                tokio::spawn(handle_message(
                    message,
                    eb_client.clone(),
                    elb_client.clone(),
                    r53_client.clone(),
                    sqs_client.clone(),
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
    message: ConfigMessage,
    eb_client: aws_sdk_elasticbeanstalk::Client,
    elb_client: aws_sdk_elasticloadbalancingv2::Client,
    r53_client: aws_sdk_route53::Client,
    sqs_client: aws_sdk_sqs::Client,
    http_client: reqwest::Client,
) -> Result<(), Error> {
    let result: Result<String, Error> = async {let balancer_arn = eb_client
        .describe_environment_resources()
        .environment_id(&message.env_id)
        .send()
        .await?
        .environment_resources()
        .ok_or(Error::new("No resources found."))?
        .load_balancers()
        .ok_or(Error::new("No load balancers yet"))?
        .get(0)
        .ok_or(Error::new("No load balancer?!"))?
        .name()
        .ok_or(Error::new("Load balancer doesn't have a name!?"))?
        .to_string();

    let http_listener_arn = elb_client
        .describe_listeners()
        .load_balancer_arn(&balancer_arn)
        .send()
        .await?
        .listeners()
        .ok_or(Error::new("No listeners found"))?
        .get(0)
        .ok_or(Error::new("No listener?!?!"))?
        .listener_arn()
        .ok_or(Error::new("Listener has no arn?!"))?
        .to_string();

    let target_group_arn = elb_client
        .describe_target_groups()
        .load_balancer_arn(&balancer_arn)
        .send()
        .await?
        .target_groups()
        .ok_or(Error::new("No target groups found"))?
        .get(0)
        .ok_or(Error::new("No target group in arr?!?!"))?
        .target_group_arn()
        .ok_or(Error::new("Target group has no arn!?"))?
        .to_string();

    // adding an https listener to the load balancer
    elb_client
        .create_listener()
        .load_balancer_arn(&balancer_arn)
        .port(443)
        .protocol(ProtocolEnum::Https)
        .ssl_policy("ELBSecurityPolicy-2016-08")
        .default_actions(
            Action::builder()
                .r#type(ActionTypeEnum::Forward)
                .target_group_arn(&target_group_arn)
                .build(),
        )
        .certificates(
            Certificate::builder()
            .certificate_arn("arn:aws:acm:us-east-1:262246349843:certificate/67e2142a-df92-424b-b92c-f5af04d12952")
            .build()
        )
        .send()
        .await?;

    // change http listener to redirect to the https port
    elb_client
        .modify_listener()
        .listener_arn(&http_listener_arn)
        .default_actions(
            Action::builder()
                .r#type(ActionTypeEnum::Redirect)
                .redirect_config(
                    RedirectActionConfig::builder()
                        .status_code(RedirectActionStatusCodeEnum::Http301)
                        .port("443")
                        .protocol("HTTPS")
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;

    elb_client
        .set_security_groups()
        .load_balancer_arn(&balancer_arn)
        .security_groups("sg-0e949ec585c11b34a")
        .send()
        .await?;

    let cname = eb_client
        .describe_environments()
        .application_name(&message.application_name)
        .environment_ids(&message.env_id)
        .send()
        .await?
        .environments()
        .ok_or(Error::new("No envs??"))?
        .get(0)
        .ok_or(Error::new("No env??"))?
        .cname()
        .ok_or(Error::new("No CNAME????"))?
        .to_string();

    let dns_name = format!("{}.{}", message.env_name, DOMAIN_NAME);

    // add dns record & link to the environment
    r53_client
        .change_resource_record_sets()
        .hosted_zone_id(HOSTED_ZONE_ID)
        .change_batch(
            ChangeBatch::builder()
                .changes(
                    Change::builder()
                        .action(aws_sdk_route53::model::ChangeAction::Create)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .r#type(aws_sdk_route53::model::RrType::A)
                                .name(&dns_name)
                                .alias_target(
                                    AliasTarget::builder()
                                        .dns_name(&cname)
                                        // us east 1
                                        .hosted_zone_id(ELB_ZONE_ID)
                                        .evaluate_target_health(false)
                                        .build(),
                                )
                                .build(),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;

        Ok(dns_name)}.await;

    if let Ok(dns_name) = result {
        let callback_res = http_client
            .post(format!(
                "{}instances/{}/callback",
                *CRUD_URI, message.instance_id
            ))
            .header("jwt", &message.jwt)
            .json(&serde_json::json!({
                "envId": &message.env_id,
                "accountId": &message.account_id,
                "url": &dns_name,
            }))
            .send()
            .await?;

        match callback_res.error_for_status() {
            Ok(_) => Ok(()),
            Err(_) => {
                // failed to tell central that instance is deployed
                // delete env and domain records and send to the fail queue

                let env = eb_client
                    .terminate_environment()
                    .environment_id(&message.env_id)
                    .force_terminate(true)
                    .send()
                    .await?;

                r53_client
                    .change_resource_record_sets()
                    .hosted_zone_id("Z0898550109O7ZB98C1FF")
                    .change_batch(
                        ChangeBatch::builder()
                            .changes(
                                Change::builder()
                                    .action(ChangeAction::Delete)
                                    .resource_record_set(
                                        ResourceRecordSet::builder()
                                            .name(&dns_name)
                                            .r#type(RrType::A)
                                            .alias_target(
                                                AliasTarget::builder()
                                                    .dns_name(env.cname().unwrap())
                                                    .evaluate_target_health(false)
                                                    .hosted_zone_id("Z117KPS5GTRQ2G")
                                                    .build(),
                                            )
                                            .build(),
                                    )
                                    .build(),
                            )
                            .build(),
                    )
                    .send()
                    .await?;

                // send to fail queue
                sqs_client
                    .send_message()
                    .queue_url(
                        "https://sqs.us-east-1.amazonaws.com/262246349843/DeadInstanceDeploy",
                    )
                    .message_body(&serde_json::to_string(&message).unwrap())
                    .send()
                    .await?;

                // return ok so that it doesn't go back into queue
                Ok(())
            }
        }
    } else {
        Err(result.err().unwrap())
    }
}
