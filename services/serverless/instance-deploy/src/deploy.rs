mod common;
mod error;
mod types;

use std::time::Duration;

use aws_lambda_events::sns::SnsEvent;
use aws_sdk_elasticbeanstalk::{
    model::{ConfigurationOptionSetting, EnvironmentTier},
    output::CreateEnvironmentOutput,
};
use lambda_runtime::{service_fn, LambdaEvent};
use nanoid::nanoid;
use serde::Serialize;

use common::CRUD_URI;
use error::Error;
use types::{ConfigMessage, DeployMessage};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct Response {
    message: String,
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

async fn func(event: LambdaEvent<SnsEvent>) -> Result<Response, Error> {
    tracing::info!("ev received");
    let aws_config = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_config);
    let sqs_client = aws_sdk_sqs::Client::new(&aws_config);

    tracing::info!("aws clients made");

    let http_client = reqwest::Client::builder()
        .use_rustls_tls()
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .build()
        .unwrap();

    let (event, _context) = event.into_parts();
    let message: DeployMessage = serde_json::from_str(&event.records[0].sns.message).unwrap();
    // eventually get this from the payload
    let application_name = "test-deploy";

    let result: Result<CreateEnvironmentOutput, Error> = async {
        let app_version_label = eb_client
            .describe_application_versions()
            .application_name(application_name)
            .send()
            .await?
            .application_versions()
            .map(|x| x.to_owned())
            .ok_or(Error::new(format!(
                "Error getting application versions for application: {}",
                application_name
            )))?
            .get(0)
            .map(|x| x.clone())
            .ok_or(Error::new(format!(
                "No applications versions for application: {}",
                application_name
            )))?
            .version_label()
            .map(|x| x.to_owned())
            .ok_or(Error::new("No version label?!?!?!"))?;

        println!("ver label gotten");

        let solutions_stacks = eb_client
            .list_available_solution_stacks()
            .send()
            .await?
            .solution_stacks()
            .map(|x| x.to_owned())
            .ok_or(Error::new("No solution stacks found?????"))?;

        let docker_stack = solutions_stacks
            .into_iter()
            .find(|x| x.contains("running Docker"))
            .ok_or(Error::new("Couldn't find Docker in solution stacks"))?;

        let env_name = format!("{}-{}", message.name, message.account_id);
        let db_pass = nanoid!(36);
        let jwt_secret = nanoid!(36);

        let env_info = eb_client
            .create_environment()
            .application_name(application_name)
            .environment_name(env_name)
            .tier(
                EnvironmentTier::builder()
                    .name("WebServer")
                    .r#type("Standard")
                    .build(),
            )
            .version_label(app_version_label)
            .solution_stack_name(docker_stack)
            .set_option_settings(Some({
                let mut options = no_config_options();

                options.extend(
                    [
                        set_db("DBPassword", &db_pass),
                        set_env("KEY", &message.key),
                        set_env("ID", &message.instance_id),
                        set_env("ACCOUNT_ID", &message.account_id),
                        set_env("NEXT_PUBLIC_NAME", &message.name),
                        set_env("NAME", &message.name),
                        set_env("JWT_SECRET", &jwt_secret),
                    ]
                    .into_iter(),
                );

                options
            }))
            .send()
            .await?;

        Ok(env_info)
    }
    .await;

    if let Ok(env_info) = result {
        let env_id = env_info
            .environment_id()
            .ok_or(Error::new("No env id??"))?
            .to_string();

        let env_name = env_info
            .environment_name()
            .ok_or(Error::new("No env name??"))?
            .to_string();

        sqs_client
            .send_message()
            .queue_url("https://sqs.us-east-1.amazonaws.com/262246349843/CentralInstanceConfig")
            .message_body(
                serde_json::to_string(&ConfigMessage {
                    account_id: message.account_id.clone(),
                    instance_id: message.instance_id.clone(),
                    key: message.key,
                    name: message.name,
                    jwt: message.jwt,
                    application_name: application_name.to_string(),
                    // add necessary data from environment
                    env_id,
                    env_name,
                })
                .unwrap(),
            )
            .send()
            .await?;

        Ok(Response {
            message: "Deployed".into(),
        })
    } else {
        http_client
            .post(format!(
                "{}instances/{}/fail-callback",
                *CRUD_URI, message.instance_id
            ))
            .header("jwt", message.jwt)
            .send()
            .await?;

        Err(result.err().unwrap())
    }
}

type SetterFunction = &'static (dyn Sync + Fn(&str, &str) -> ConfigurationOptionSetting);

#[allow(non_upper_case_globals)]
static set_db: SetterFunction = &create_option_setter("aws:rds:dbinstance");
#[allow(non_upper_case_globals)]
static set_env: SetterFunction =
    &create_option_setter("aws:elasticbeanstalk:application:environment");

fn no_config_options() -> Vec<ConfigurationOptionSetting> {
    let options: Vec<ConfigurationOptionSetting> = vec![
        set_db("DBAllocatedStorage", "10"),
        set_db("DBDeletionPolicy", "Delete"),
        set_db("DBEngine", "postgres"),
        set_db("DBEngineVersion", "14.2"),
        set_db("DBUser", "backend"),
        set_db("HasCoupledDatabase", "true"),
        set_db("DBInstanceClass", "db.t4g.micro"),
        option(
            "aws:elasticbeanstalk:environment",
            "LoadBalancerType",
            "application",
        ),
        option(
            "aws:elasticbeanstalk:command",
            "DeploymentPolicy",
            "RollingWithAdditionalBatch",
        ),
        option("aws:ec2:instances", "InstanceTypes", "t2.micro, t3.micro"),
        option(
            "aws:autoscaling:launchconfiguration",
            "IamInstanceProfile",
            "aws-elasticbeanstalk-ec2-role",
        ),
        option(
            "aws:autoscaling:launchconfiguration",
            "SecurityGroups",
            "central-instances",
        ),
        option(
            "aws:elasticbeanstalk:environment:process:default",
            "HealthCheckPath",
            "/health",
        ),
        option(
            "aws:elasticbeanstalk:environment:process:default",
            "MatcherHTTPCode",
            "308,301,307,302",
        ),
    ];

    options
}

fn option(namespace: &str, key: &str, value: &str) -> ConfigurationOptionSetting {
    ConfigurationOptionSetting::builder()
        .namespace(namespace)
        .option_name(key)
        .value(value)
        .build()
}

const fn create_option_setter(
    namespace: &'static str,
) -> impl Fn(&str, &str) -> ConfigurationOptionSetting {
    {
        move |k, v| option(namespace, k, v)
    }
}
