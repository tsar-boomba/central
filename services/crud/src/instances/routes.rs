use std::time::Duration;

use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use auth::{belongs_to_account, require_role, ReqUser};
use models::{
    types::{InstanceStatus, Role},
    Account, Instance, Model, NewInstance, UpdateInstance, Validate,
};
use reqwest::{redirect::Policy, Client};
use serde::Deserialize;

use crate::{
    accounts, api_error::ApiError, auth::verify_instance_deploy, json::DeleteBody, update_usage,
    AppData,
};

#[get("/instances")]
async fn find_all(req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let instances = web::block(Instance::find_all).await??;

    let instances = if let Some(req_user) = req_user {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the instance is admin of site they will see the instance
        instances
            .into_iter()
            .filter(|x| x.account_id == req_user.account_id || req_user.account_id == "admin")
            .collect()
    } else {
        instances
    };

    Ok(HttpResponse::Ok().json(instances))
}

#[get("/instances/{id}")]
async fn find(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let instance = web::block(move || Instance::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&req_user, &instance.account_id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(instance))
}

#[post("/instances")]
async fn create(
    instance: web::Json<NewInstance>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&req_user, &instance.account_id) || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    // make sure user's account is subbed
    if let Some(req_user) = req_user {
        let owner_id = req_user.account_id.clone();
        let owner = web::block(move || Account::find_by_id(owner_id)).await??;
        if owner.sub_id.is_none() {
            return Err(ApiError::not_subbed());
        }
    }

    let created = NewInstance {
        status: InstanceStatus::Deploying,
        ..instance.into_inner()
    };

    created.validate()?;

    let instance = web::block(move || Instance::insert(created)).await??;

    // just start deployment with aws, lambda will call back later with url and env_id
    let deploy_result = super::utils::deploy(&instance).await;

    if let Err(_) = deploy_result {
        // initial deployment failed
        let update_result = web::block(move || {
            Instance::update(
                instance.id,
                UpdateInstance {
                    status: Some(InstanceStatus::Failed),
                    ..Default::default()
                },
            )
        })
        .await?;

        match update_result {
                    Ok(_) => Err(ApiError::new(
                        500,
                        "Initial deployment failed. Please try again later.".into(),
                    )),
                    Err(_) => Err(ApiError::new(500, "Initial deployment failed, current instance status is 'Failed', but couldn't be updated.".into()))
                }
    } else {
        super::utils::ensure_deployment(instance.id.clone());
        Ok(HttpResponse::Ok().json(instance))
    }
}

#[put("/instances/{id}")]
async fn update(
    id: web::Path<String>,
    instance: web::Json<UpdateInstance>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let for_find_to_be_updated = id.clone();
    let to_be_updated = web::block(move || Instance::find_by_id(for_find_to_be_updated)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.account_id)
        || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    // fields which can't be updated after creation
    let update_set: UpdateInstance = UpdateInstance {
        account_id: None,
        name: None,
        url: None,
        ..instance.into_inner()
    };

    update_set.validate()?;

    let instance = web::block(move || Instance::update(id, update_set)).await??;

    Ok(HttpResponse::Ok().json(instance))
}

#[delete("/instances/{id}")]
async fn delete(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    let instance = web::block(move || Instance::find_by_id(find_id)).await??;
    if !belongs_to_account(&req_user, &instance.account_id) || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    let owner_id = instance.account_id.clone();
    let owner = web::block(move || Account::find_by_id(owner_id)).await??;
    let num_instances = accounts::utils::usage(owner.id.clone()).await?.instances;
    println!("{num_instances}");

    if instance.status == InstanceStatus::Deploying || instance.status == InstanceStatus::Configured
    {
        // cannot delete while these are happening
        return Err(ApiError::new(
            400,
            "Cannot delete an instance that has status 'Deploying' or 'Configured'.".into(),
        ));
    }

    if let Some(env_id) = instance.env_id {
        if let Some(url) = instance.url {
            if instance.status == InstanceStatus::Ok || instance.status == InstanceStatus::Unhealthy
            {
                // only these two statuses require aws termination
                let env = super::aws::delete_instance(&app_data.eb_client, &env_id).await?;

                super::aws::delete_dns(&app_data.r53_client, &url, &env).await?;
            }
        }
    }

    let affected = web::block(move || Instance::delete(id.into_inner())).await??;

    // subtract 1 because it was successfully deleted
    let res = web::block(move || update_usage(&owner, "instances".into(), num_instances - 1)).await??;

    if let Err(_) = res.error_for_status() {
        // TODO notify me
        error!("Failed to update instance usage with Stripe. Your instance is still deleted.");
        return Ok(HttpResponse::InternalServerError().finish());
    };

    Ok(HttpResponse::Ok().json(DeleteBody::new(affected as i32)))
}

#[put("/instances/{id}/deactivate")]
async fn deactivate(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
    app_data: web::Data<AppData>,
) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    let instance = web::block(move || Instance::find_by_id(find_id)).await??;
    let owner_id = instance.account_id.clone();
    let owner = web::block(|| Account::find_by_id(owner_id)).await??;
    if !belongs_to_account(&req_user, &instance.account_id) || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    if instance.status != InstanceStatus::Ok && instance.status != InstanceStatus::Unhealthy {
        // cannot deactivate while these are happening
        return Err(ApiError::new(
            400,
            "Cannot deactivate an instance that has status 'Deploying' or 'Configured'.".into(),
        ));
    }

    let usage = accounts::utils::usage(instance.account_id).await?;
    if let Some(env_id) = instance.env_id {
        if let Some(url) = instance.url {
            // must ba one of these two to be deactivated
            let env = super::aws::delete_instance(&app_data.eb_client, &env_id).await?;
            super::aws::delete_dns(&app_data.r53_client, &url, &env).await?;

            web::block(|| {
                Instance::update(
                    id.into_inner(),
                    UpdateInstance {
                        status: Some(InstanceStatus::Inactive),
                        ..Default::default()
                    },
                )
            })
            .await??;

            let num_instances = usage.instances - 1;
            let res = web::block(move || update_usage(&owner, "instances".into(), num_instances))
                .await??;

            if let Err(_) = res.error_for_status() {
                // TODO notify me
                error!("Failed to update instance usage with Stripe. Your instance will still be deactivated.");
                return Ok(HttpResponse::InternalServerError().finish());
            };

            Ok(HttpResponse::Ok().finish())
        } else {
            Err(ApiError::new(
                400,
                "Instance must have a url property to be deactivated.".into(),
            ))
        }
    } else {
        Err(ApiError::new(
            400,
            "Instance must have a environment id property to be deactivated.".into(),
        ))
    }
}

#[put("/instances/{id}/deploy")]
async fn deploy(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    let instance = web::block(move || Instance::find_by_id(find_id)).await??;
    if !belongs_to_account(&req_user, &instance.account_id) || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    if instance.status != InstanceStatus::Inactive && instance.status != InstanceStatus::Failed {
        // cannot deploy while these are happening
        return Err(ApiError::new(
            400,
            "Can only deactivate an instance that has status 'Inactive' or 'Failed'.".into(),
        ));
    }

    let update_id = instance.id.clone();
    web::block(|| {
        Instance::update(
            update_id,
            UpdateInstance {
                status: Some(InstanceStatus::Deploying),
                ..Default::default()
            },
        )
    })
    .await??;

    // just start deployment with aws, lambda will call back later with url and env_id
    let deploy_result = super::utils::deploy(&instance).await;

    if let Err(_) = deploy_result {
        info!("failed to send req to deploy instance");
        // initial deployment failed
        let update_result = web::block(move || {
            Instance::update(
                instance.id,
                UpdateInstance {
                    status: Some(InstanceStatus::Failed),
                    ..Default::default()
                },
            )
        })
        .await?;

        match update_result {
                    Ok(_) => Err(ApiError::new(
                        500,
                        "Initial deployment failed. Please try again later.".into(),
                    )),
                    Err(_) => Err(ApiError::new(500, "Initial deployment failed, current instance status is 'Failed', but couldn't be updated.".into()))
                }
    } else {
        super::utils::ensure_deployment(instance.id.clone());
        Ok(HttpResponse::Ok().json(instance))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CallbackParams {
    env_id: String,
    url: String,
    account_id: String,
}

#[post("/instances/{id}/callback")]
async fn callback(
    target: web::Path<String>,
    params: web::Json<CallbackParams>,
    req: HttpRequest,
) -> Result<HttpResponse, ApiError> {
    // make sure has token sent to instance deploy invocation
    if verify_instance_deploy(
        &req.headers()
            .get("jwt")
            .map(|v| v.to_str().unwrap())
            .unwrap_or_default()
            .to_string(),
    )
    .is_err()
    {
        return Err(ApiError::forbidden());
    }

    let owner_id = params.account_id.clone();
    let owner = web::block(move || Account::find_by_id(owner_id)).await??;

    let params = params.into_inner();
    web::block(move || {
        Instance::update(
            target.into_inner(),
            UpdateInstance {
                env_id: Some(Some(params.env_id)),
                url: Some(Some(params.url)),
                status: Some(InstanceStatus::Configured),
                ..Default::default()
            },
        )
    })
    .await??;

    let num_instances = accounts::utils::usage(owner.id.clone()).await?.instances;

    let res = web::block(move || update_usage(&owner, "instances".into(), num_instances)).await??;

    if let Err(_) = res.error_for_status() {
        // TODO notify me
        error!("Failed to update instance usage with Stripe. Your instance will still be usable.");
        return Ok(HttpResponse::InternalServerError().finish());
    };

    Ok(HttpResponse::Ok().finish())
}

// Checks instance health
#[get("/instances/{id}/health")]
async fn health(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let for_find_to_be_updated = id.clone();
    let instance = web::block(move || Instance::find_by_id(for_find_to_be_updated)).await??;
    if !belongs_to_account(&req_user, &instance.account_id) {
        return Err(ApiError::forbidden());
    }

    if instance.status != InstanceStatus::Ok
        && instance.status != InstanceStatus::Unhealthy
        && instance.status != InstanceStatus::Configured
    {
        // must be one of the above to be valid for refreshing status
        return Err(ApiError::new(
            400,
            "Instance must be 'Ok', 'Unhealthy', or 'Configured' to have it's status checked."
                .into(),
        ));
    }

    if let Some(url) = instance.url {
        let client = Client::builder()
            .redirect(Policy::none())
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap();
        let res = client.get(format!("https://{url}/health")).send().await;

        match res {
            Ok(res) => {
                if res.status().is_redirection() {
                    // redirect is desireable response for health check
                    web::block(move || {
                        Instance::update(
                            id,
                            UpdateInstance {
                                status: Some(InstanceStatus::Ok),
                                ..Default::default()
                            },
                        )
                    })
                    .await??;
                    Ok(HttpResponse::Ok().finish())
                } else {
                    web::block(move || {
                        Instance::update(
                            id,
                            UpdateInstance {
                                status: Some(InstanceStatus::Unhealthy),
                                ..Default::default()
                            },
                        )
                    })
                    .await??;
                    Err(ApiError::new(
                        500,
                        "Instance returned a bad response, setting to unhealthy.".into(),
                    ))
                }
            }
            Err(err) => {
                match instance.status {
                    InstanceStatus::Configured => {
                        // if its configured and couldn't connect its okay, otherwise there is an error
                        if err.is_connect() {
                            Ok(HttpResponse::Ok().finish())
                        } else {
                            web::block(move || {
                                Instance::update(
                                    id,
                                    UpdateInstance {
                                        status: Some(InstanceStatus::Unhealthy),
                                        ..Default::default()
                                    },
                                )
                            })
                            .await??;
                            Ok(HttpResponse::Ok().finish())
                        }
                    }
                    _ => {
                        // any other status must be updated to unhealthy
                        web::block(move || {
                            Instance::update(
                                id,
                                UpdateInstance {
                                    status: Some(InstanceStatus::Unhealthy),
                                    ..Default::default()
                                },
                            )
                        })
                        .await??;
                        Ok(HttpResponse::Ok().finish())
                    }
                }
            }
        }
    } else {
        Err(ApiError::new(400, "Instance does not have a url.".into()))
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(deactivate);
    config.service(deploy);
    config.service(callback);
    config.service(health);
}
