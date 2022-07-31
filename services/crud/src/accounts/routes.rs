use models::{Account, NewAccount, Model};
use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{api_error::ApiError, auth::Claim, belongs_to_account, json::DeleteBody};

#[get("/accounts")]
async fn find_all(jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let accounts = web::block(Account::find_all).await??;

    let accounts = if let Some(jwt) = jwt {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the account is admin of site they will see the account
        accounts
            .into_iter()
            .filter(|x| x.id == jwt.account_id || jwt.account_id == "admin")
            .collect()
    } else {
        accounts
    };

    Ok(HttpResponse::Ok().json(accounts))
}

#[get("/accounts/{id}")]
async fn find(id: web::Path<String>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let account = web::block(move || Account::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&jwt, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(account))
}

#[get("/accounts/{id}/is-subbed")]
async fn is_subbed(id: web::Path<String>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let account = web::block(move || Account::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&jwt, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().body(account.stripe_id.is_some().to_string()))
}

#[post("/accounts")]
async fn create(
    account: web::Json<NewAccount>,
    jwt: Option<Claim>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &account.id) {
        return Err(ApiError::forbidden());
    }

    let account = web::block(move || Account::insert(account.into_inner())).await??;

    Ok(HttpResponse::Ok().json(account))
}

#[put("/accounts/{id}")]
async fn update(
    id: web::Path<String>,
    account: web::Json<NewAccount>,
    jwt: Option<Claim>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &account.id) {
        return Err(ApiError::forbidden());
    }

    let account =
        web::block(move || Account::update(id.into_inner(), account.into_inner())).await??;

    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
async fn delete(id: web::Path<String>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let find_id = id.clone();
    if !belongs_to_account(
        &jwt,
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
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(is_subbed);
}
