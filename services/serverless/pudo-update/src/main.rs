use lambda_runtime::{run, service_fn, Error, LambdaEvent};

use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    exit_code: i32
}

async fn function_handler(event: LambdaEvent<Value>) -> Result<Response, Error> {
    let resp = Response {
        exit_code: 0
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
