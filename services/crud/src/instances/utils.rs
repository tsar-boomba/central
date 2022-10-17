use models::{types::InstanceStatus, Instance, Model, UpdateInstance};

use crate::{api_error::ApiError, auth};

pub async fn deploy(instance: &Instance, sns_client: &aws_sdk_sns::Client) -> Result<(), ApiError> {
    let result = sns_client
        .publish()
        .topic_arn("arn:aws:sns:us-east-1:262246349843:InstanceDeploy")
        .message(
            serde_json::to_string(&serde_json::json!({
                "instanceId": instance.id,
                "accountId": instance.account_id,
                "name": instance.name,
                "key": auth::sign_instance_key().unwrap(),
            }))
            .unwrap(),
        )
        .send()
        .await;

    if let Err(_) = result {
        return Err(ApiError::new(
            500,
            "An error ocurred during initial instance deployment. Please try again later.".into(),
        ));
    };

    Ok(())
}

/// Queues a task that runs after 15mins to ensure that deployment completed successfully
/// If the instance status
pub fn ensure_deployment(id: String) {
    actix_web::rt::spawn((|| async {
        actix_web::rt::time::sleep(core::time::Duration::from_secs(15 * 60)).await;
        let instance = Instance::find_by_id(id.clone());

        if let Ok(instance) = instance {
            if instance.status == InstanceStatus::Deploying {
                // timeout while deploying
                // TODO notify user
                info!("Instance {} timed out while deploying.", instance.id);

                match Instance::update(
                    id,
                    UpdateInstance {
                        status: Some(InstanceStatus::Failed),
                        ..Default::default()
                    },
                ) {
                    Err(_) => {
                        error!("Error updating failed deployment.")
                    }
                    _ => {}
                };
            }
        } else {
        }
    })());
}
