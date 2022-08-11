use actix_web::{delete, get, post, put, web, HttpResponse};
use auth::{belongs_to_account, ReqUser};
use diesel::prelude::*;
use models::{Account, Instance, Model, NewAccount, UpdateAccount, User, Validate};
use payments_lib::routes::customer;
use reqwest::Client;

use crate::{api_error::ApiError, db, json::DeleteBody, PAYMENTS_URI};

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

// TODO tests
#[get("/accounts/{id}/instances")]
async fn find_instances(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();
    let for_find_to_be_found = target.clone();
    let to_be_found = web::block(move || Account::find_by_id(for_find_to_be_found)).await??;
    if !belongs_to_account(&req_user, &to_be_found.id) {
        return Err(ApiError::forbidden());
    }

    use models::instances::dsl::*;
    let account = web::block::<_, Result<Vec<Instance>, ApiError>>(move || {
        Ok(models::instances::table
            .filter(account_id.eq(target))
            .get_results(&db::connection()?)?)
    })
    .await??;

    Ok(HttpResponse::Ok().json(account))
}

#[get("/accounts/{id}/usage")]
async fn usage(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let target = target.into_inner();
    if !belongs_to_account(&req_user, &target) {
        return Err(ApiError::forbidden());
    }

    let users_acct_id = target.clone();
    let num_users = web::block::<_, Result<i64, ApiError>>(move || {
        use models::users::dsl::*;
        Ok(users
            .count()
            .filter(account_id.eq(users_acct_id))
            .get_result::<i64>(&db::connection()?)?)
    })
    .await??;

    let instances_acct_id = target.clone();
    let num_instances = web::block::<_, Result<i64, ApiError>>(move || {
        use models::instances::dsl::*;
        Ok(instances
            .count()
            .filter(account_id.eq(instances_acct_id))
            .get_result::<i64>(&db::connection()?)?)
    })
    .await??;

    Ok(HttpResponse::Ok()
        .json(serde_json::json!({ "users": num_users, "instances": num_instances })))
}

#[get("/accounts/{id}/is-subbed")]
async fn is_subbed(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    let account = web::block(move || Account::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&req_user, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().body(account.sub_id.is_some().to_string()))
}

#[get("/accounts/by-sub/{id}")]
async fn find_by_sub(
    target: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    use models::accounts::dsl::*;
    let conn = db::connection()?;
    let account = web::block(move || {
        models::accounts::table
            .filter(sub_id.eq(target.into_inner()))
            .first::<Account>(&conn)
    })
    .await??;

    if !belongs_to_account(&req_user, &account.id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(account))
}

#[post("/accounts")]
async fn create(
    account: web::Json<NewAccount>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&req_user, &account.id) || req_user.is_some() {
        return Err(ApiError::forbidden());
    }

    let account = account.into_inner();
    account.validate()?;

    let account = web::block(move || Account::insert(account)).await??;

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

    let update_set: UpdateAccount = if req_user == None {
        account.into_inner()
    } else {
        // fields which should never be updated from external sources
        UpdateAccount {
            stripe_id: None,
            sub_id: None,
            ..account.into_inner()
        }
    };

    update_set.validate()?;

    if let Some(stripe_id) = to_be_updated.stripe_id {
        let res = Client::new()
            .put(PAYMENTS_URI.to_string() + &customer::update::route(stripe_id.as_str()))
            .json(&update_set)
            .send()
            .await?;
        if res.error_for_status().is_err() {
            return Err(ApiError::new(
                500,
                "Failed to update account with stripe.".into(),
            ));
        }
    };

    let account = web::block(move || Account::update(id, update_set)).await??;

    Ok(HttpResponse::Ok().json(account))
}

#[delete("/accounts/{id}")]
async fn delete(
    id: web::Path<String>,
    req_user: Option<ReqUser>,
) -> Result<HttpResponse, ApiError> {
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
    config.service(find_instances);
    config.service(usage);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(is_subbed);
    config.service(find_by_sub);
}
