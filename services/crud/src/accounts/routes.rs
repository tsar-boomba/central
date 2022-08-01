use actix_web::{delete, get, post, put, web, HttpResponse};
use auth::{ReqUser, belongs_to_account};
use diesel::prelude::*;
use models::{Account, Model, NewAccount, UpdateAccount, User};

use crate::{api_error::ApiError, db, json::DeleteBody};

#[get("/accounts")]
async fn find_all(req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let accounts = web::block(Account::find_all).await??;

    let accounts = if let Some(req_user) = req_user {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the account is admin of site they will see the account
        accounts
            .into_iter()
            .filter(|x| x.id == req_user.account_id || req_user.account_id == "admin")
            .collect()
    } else {
        accounts
    };

    Ok(HttpResponse::Ok().json(accounts))
}

#[get("/accounts/{id}")]
async fn find(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let account = web::block(move || Account::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&req_user, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(account))
}

// TODO tests
#[get("/accounts/{id}/users")]
async fn find_users(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();
    let for_find_to_be_found = target.clone();
    let to_be_found = web::block(move || Account::find_by_id(for_find_to_be_found)).await??;
    if !belongs_to_account(&req_user, &to_be_found.id) {
        return Err(ApiError::forbidden());
    }

    use models::users::dsl::*;
    let account = web::block::<_, Result<Vec<User>, ApiError>>(move || {
        Ok(models::users::table
            .filter(account_id.eq(target))
            .get_results(&db::connection()?)?)
    })
    .await??;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/accounts/{id}/is-subbed")]
async fn is_subbed(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let account = web::block(move || Account::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&req_user, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().body(account.sub_id.is_some().to_string()))
}

#[post("/accounts")]
async fn create(
    account: web::Json<NewAccount>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&req_user, &account.id) {
        return Err(ApiError::forbidden());
    }

    let account = web::block(move || Account::insert(account.into_inner())).await??;

    Ok(HttpResponse::Ok().json(account))
}

#[put("/accounts/{id}")]
async fn update(
    id: web::Path<String>,
    account: web::Json<UpdateAccount>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let id = id.into_inner();
    let for_find_to_be_updated = id.clone();
    let to_be_updated = web::block(move || Account::find_by_id(for_find_to_be_updated)).await??;
    if !belongs_to_account(&req_user, &to_be_updated.id) {
        return Err(ApiError::forbidden());
    }

    let account = web::block(move || Account::update(id, account.into_inner())).await??;

    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
async fn delete(id: web::Path<String>, req_user: Option<ReqUser>) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    if !belongs_to_account(
        &req_user,
        &web::block(move || Account::find_by_id(find_id)).await??.id,
    ) {
        return Err(ApiError::forbidden());
    }

    let affected = web::block(move || Account::delete(id.into_inner())).await??;

    Ok(HttpResponse::Ok().json(DeleteBody::new(affected.try_into().unwrap())))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(find_users);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(is_subbed);
}
