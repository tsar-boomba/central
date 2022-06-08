use super::model::*;
use actix_web::{delete, get, post, put, web, HttpResponse};

use crate::{api_error::ApiError, auth::Claim, belongs_to_account, json::DeleteBody};

#[get("/users")]
async fn find_all(jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let users = web::block(User::find_all).await??;

    let users = if let Some(jwt) = jwt {
        // if filter func returns true item will be allowed into the iterator
        // so if the account ids match or if the user is admin of site they will see the user
        users
            .into_iter()
            .filter(|x| x.account_id == jwt.account_id || jwt.account_id == "admin")
            .collect()
    } else {
        users
    };

    Ok(HttpResponse::Ok().json(users))
}

#[get("/users/{id}")]
async fn find(id: web::Path<i32>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let user = web::block(move || User::find_by_id(id.into_inner())).await??;

    if !belongs_to_account(&jwt, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    Ok(HttpResponse::Ok().json(user))
}

#[post("/users")]
async fn create(user: web::Json<NewUser>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    let user = web::block(move || User::insert(user.into_inner())).await?;

    match user {
        Ok(user) => Ok(HttpResponse::Ok().json(user)),
        Err(err) => {
            if err.status_code == 409 {
                // there was a conflict only possibility is username
                Err(ApiError::new(409, "Username must be unique.".into()))
            } else {
                Err(err)
            }
        }
    }
}

#[put("/users/{id}")]
async fn update(
    id: web::Path<i32>,
    user: web::Json<NewUser>,
    jwt: Option<Claim>,
) -> Result<HttpResponse, ApiError> {
    if !belongs_to_account(&jwt, &user.account_id) {
        return Err(ApiError::forbidden());
    }

    let user = web::block(move || User::update(id.into_inner(), user.into_inner())).await??;

    Ok(HttpResponse::Ok().json(user))
}

#[delete("/users/{id}")]
async fn delete(id: web::Path<i32>, jwt: Option<Claim>) -> Result<HttpResponse, ApiError> {
    let find_id = *id;
    if !belongs_to_account(
        &jwt,
        &web::block(move || User::find_by_id(find_id))
            .await??
            .account_id,
    ) {
        return Err(ApiError::forbidden());
    }

    let affected = web::block(move || User::delete(id.into_inner())).await??;

    Ok(HttpResponse::Ok().json(DeleteBody::new(affected.try_into().unwrap())))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
}
