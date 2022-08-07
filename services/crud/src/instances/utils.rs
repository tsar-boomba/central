use models::{types::InstanceStatus, Instance, Model, UpdateInstance};
use reqwest::Client;

use crate::{api_error::ApiError, auth::sign_instance, INSTANCES_URI};

pub async fn deploy(instance: &Instance) -> Result<(), ApiError> {
    let res = Client::builder()
        .build()
        .unwrap()
        .post(INSTANCES_URI.as_str())
        .header("jwt", sign_instance().unwrap())
        .json(&serde_json::json!({
            "instanceId": instance.id,
            "accountId": instance.account_id,
            "name": instance.name
        }))
        .send()
        .await?;

    if let Err(_) = res.error_for_status() {
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

                match Instance::update(
                    id,
                    UpdateInstance {
                        status: Some(InstanceStatus::Failed),
                        ..Default::default()
                    },
                ) {
                    Err(_) => {
                        error!("Error updating failed deployment.")
                    },
					_ => {}
                };
            }
        } else {
        }
    })());
}
