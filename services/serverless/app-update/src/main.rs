use std::time::Duration;

use itertools::Itertools;
use lambda_runtime::{run, service_fn, LambdaEvent};

use serde::{Deserialize, Serialize};
use serverless_util::Error;
use tracing::{error, info};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Payload {
    app_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    exit_code: i32,
}

async fn function_handler(event: LambdaEvent<Payload>) -> Result<Response, Error> {
    let (message, _context) = event.into_parts();
    let aws_config = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_config);

    let app_version_label = eb_client
        .describe_application_versions()
        .application_name(&message.app_name)
        .send()
        .await?
        .application_versions()
        .map(|x| x.to_owned())
        .ok_or(Error::new(format!(
            "Error getting application versions for application: {}",
            &message.app_name
        )))?
        .get(0)
        .map(|x| x.clone())
        .ok_or(Error::new(format!(
            "No applications versions for application: {}",
            &message.app_name
        )))?
        .version_label()
        .map(|x| x.to_owned())
        .ok_or(Error::new("No version label?!?!?!"))?;

    let environments = eb_client
        .describe_environments()
        .application_name(&message.app_name)
        .send()
        .await?
        .environments()
        .ok_or(Error::new("No environments found."))?
        .to_owned();

    if environments.len() == 0 {
        info!("No environments to update.");
        return Ok(Response { exit_code: 0 })
    }

    info!(
        "Updating {} environments for {}",
        environments.len(),
        message.app_name
    );

    let num_environments = environments.len();
    let env_chunks = environments
        .into_iter()
        // gets chunks of 10
        .chunks((num_environments as f64 / 10.0).ceil() as usize);

    for env_chunk in env_chunks.into_iter() {
        // update all envs in a chunk then wait 200ms so no rate limit
        for env in env_chunk.into_iter() {
            let update_result = eb_client
                .update_environment()
                .environment_id(env.environment_id().unwrap())
                .version_label(&app_version_label)
                .send()
                .await;

            if update_result.is_err() {
                error!(
                    "failed to update environment {} for app {}.",
                    env.environment_name().unwrap(),
                    message.app_name
                );
            }
        }
        // wait so we don't "overwhelm" aws
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    Ok(Response { exit_code: 0 })
}

#[tokio::main]
async fn main() -> Result<(), lambda_runtime::Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
