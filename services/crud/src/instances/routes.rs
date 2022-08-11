use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use auth::{belongs_to_account, require_role, ReqUser};
use diesel::prelude::*;
use models::{
    types::{InstanceStatus, Role},
    Account, Instance, Model, NewInstance, UpdateInstance, Validate,
};
use serde::Deserialize;

use crate::{api_error::ApiError, auth::verify_instance, db, json::DeleteBody, update_usage};

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
) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    if !belongs_to_account(
        &req_user,
        &web::block(move || Instance::find_by_id(find_id))
            .await??
            .account_id,
    ) || !require_role(&req_user, Role::Admin)
    {
        return Err(ApiError::forbidden());
    }

    let affected = web::block(move || Instance::delete(id.into_inner())).await??;

    Ok(HttpResponse::Ok().json(DeleteBody::new(affected.try_into().unwrap())))
}

#[derive(Debug, Deserialize)]
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
    if verify_instance(
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

    use models::instances::dsl::*;

    let conn = db::connection()?;
    let num_user = instances
        .count()
        .filter(account_id.eq(owner.id.clone()))
        .get_result::<i64>(&conn)?;

    let res = web::block(move || update_usage(&owner, "instances".into(), num_user)).await??;

    if let Err(_) = res.error_for_status() {
        // TODO notify me
        error!("Failed to update instance usage with Stripe. Your instance will still be usable.");
        return Ok(HttpResponse::InternalServerError().finish());
    };

    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(callback);
}
