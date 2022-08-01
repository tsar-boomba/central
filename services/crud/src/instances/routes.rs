use auth::{ReqUser, belongs_to_account};
use models::{Instance, NewInstance, Model, UpdateInstance};
use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{api_error::ApiError, json::DeleteBody};

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
    if !belongs_to_account(&req_user, &instance.account_id) {
        return Err(ApiError::forbidden());
    }

    let instance = web::block(move || Instance::insert(instance.into_inner())).await??;

    Ok(HttpResponse::Ok().json(instance))
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
    if !belongs_to_account(&req_user, &to_be_updated.account_id) {
        return Err(ApiError::forbidden());
    }

    let instance =
        web::block(move || Instance::update(id, instance.into_inner())).await??;

    Ok(HttpResponse::Ok().json(instance))
}

#[delete("/instances/{id}")]
async fn delete(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    if !belongs_to_account(
        &req_user,
        &web::block(move || Instance::find_by_id(find_id))
            .await??
            .account_id,
    ) {
        return Err(ApiError::forbidden());
    }

    let affected = web::block(move || Instance::delete(id.into_inner())).await??;

    Ok(HttpResponse::Ok().json(DeleteBody::new(affected.try_into().unwrap())))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    //config.route("/instances", web::get().to(find_all));

    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
