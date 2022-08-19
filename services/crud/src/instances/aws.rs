use aws_sdk_elasticbeanstalk::output::TerminateEnvironmentOutput;
use aws_sdk_route53::model::{ChangeBatch, Change, ChangeAction, ResourceRecordSet, RrType, AliasTarget};

use crate::api_error::ApiError;

pub async fn delete_instance(
    eb_client: &aws_sdk_elasticbeanstalk::Client,
    env_id: &str,
) -> Result<TerminateEnvironmentOutput, ApiError> {
    let res = eb_client
        .terminate_environment()
        .environment_id(env_id)
        .force_terminate(true)
        .send()
        .await;

    if let Err(err) = res {
        error!("{:?}", err);
        return Err(ApiError::new(
            500,
            "An error ocurred while terminating the instance.".into(),
        ));
    }

    Ok(res.unwrap())
}

pub async fn delete_dns(
    r53_client: &aws_sdk_route53::Client,
    url: &str,
    env: &TerminateEnvironmentOutput,
) -> Result<(), ApiError> {
    let res = r53_client
        .change_resource_record_sets()
        .hosted_zone_id("Z0898550109O7ZB98C1FF")
        .change_batch(
            ChangeBatch::builder()
                .changes(
                    Change::builder()
                        .action(ChangeAction::Delete)
                        .resource_record_set(
                            ResourceRecordSet::builder()
                                .name(url)
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
        .await;

    if let Err(err) = res {
        error!("{:?}", err);
        return Err(ApiError::new(
                        500,
                        "An error ocurred while deleting the dns. Your instance has terminated and is no longer available. Please try again.".into(),
                    ));
    }

	Ok(())
}
