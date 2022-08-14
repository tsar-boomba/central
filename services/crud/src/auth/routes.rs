use actix_web::{
    cookie::{time::Duration, Cookie},
    get, post,
    web::{block, Json, ServiceConfig},
    HttpRequest, HttpResponse,
};
use models::User;
use diesel::prelude::*;

use crate::{api_error::ApiError, auth::Login, db, json::ErrorBody};

#[post("/login")]
async fn login(login: Json<Login>) -> Result<HttpResponse, ApiError> {
    use models::users::dsl::*;
    let conn = db::connection()?;
    let login = login.into_inner();
    let user = block(move || {
        users
            .filter(username.eq(login.username))
            .filter(account_id.eq(login.account_id))
            .first::<User>(&conn)
            .optional()
    })
    .await??;

    // check if found a matching user
    if let Some(user) = user {
        // check password hashes
        match block(move || bcrypt::verify(login.password, &user.password)).await? {
            Ok(result) => {
                if result {
                    // passwords match
                    let token = super::sign(user.id, user.account_id);
                    match token {
                        Ok(token) => Ok(HttpResponse::Ok()
                            .cookie(
                                Cookie::build("at", token)
                                    .path("/")
                                    .http_only(true)
                                    .secure(true)
                                    .max_age(Duration::milliseconds(super::TOKEN_EXPIRY))
                                    .finish(),
                            )
                            .finish()),
                        _ => {
                            Ok(HttpResponse::InternalServerError()
                                .json(ErrorBody::server_err(None)))
                        }
                    }
                } else {
                    // password do not match
                    Ok(HttpResponse::Unauthorized()
                        .json(ErrorBody::new("Incorrect username or password.")))
                }
            }
            // error comparing passwords
            _ => Ok(HttpResponse::InternalServerError().json(ErrorBody::server_err(None))),
        }
    } else {
        // handle found no user
        Ok(HttpResponse::Unauthorized().json(ErrorBody::new("Incorrect username or password.")))
    }
}

/// Returns whether the request is authenticated (has valid jwt)
#[get("/authenticate")]
async fn authenticate(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    match req.cookie("at") {
        // cookie exists try to verify
        Some(token) => match super::verify(&token.value().into()) {
            // valid token, return ok
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            // invalid token, fail
            _ => Ok(HttpResponse::Unauthorized().json(ErrorBody::unauthorized(None))),
        },
        // cookie doesn't exist, automatic fail
        _ => Ok(HttpResponse::Unauthorized().json(ErrorBody::unauthorized(None))),
    }
}

/// Returns full user from jwt
#[get("/verify")]
async fn verify(req: HttpRequest) -> Result<HttpResponse, ApiError> {
    use models::users::dsl::*;
    match req.cookie("at") {
        // cookie exists try to verify
        Some(token) => match super::verify(&token.value().into()) {
            // valid token, get user from db
            Ok(claim) => {
                let conn = db::connection()?;
                let user = block(move || {
                    users
                        .filter(id.eq(claim.id))
                        .filter(account_id.eq(claim.account_id))
                        .get_result::<User>(&conn)
                        .optional()
                })
                .await?;

                // check if db operation failed
                if let Ok(user) = user {
                    // check if found a matching user
                    if let Some(user) = user {
                        Ok(HttpResponse::Ok().json(user))
                    } else {
                        // handle found no user
                        Ok(HttpResponse::Unauthorized()
                            .json(ErrorBody::new("Incorrect username or password.")))
                    }
                } else {
                    // handle db error
                    Ok(HttpResponse::InternalServerError().json(ErrorBody::new("Database error.")))
                }
            }
            // invalid token, fail
            _ => Ok(HttpResponse::Unauthorized().json(ErrorBody::new("Invalid authentication"))),
        },
        // doesn't exists, automatic fail
        _ => Ok(HttpResponse::Unauthorized().json(ErrorBody::new("No authentication."))),
    }
}

pub fn init_routes(config: &mut ServiceConfig) {
    config.service(login);
    config.service(verify);
    config.service(authenticate);
}
