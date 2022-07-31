#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use(error, info)]
extern crate log;
#[macro_use(nanoid)]
extern crate nanoid;

const ID_SIZE: usize = 12;

mod api_error;
mod auth;
mod db;
mod json;
mod sql_types;

mod accounts;
mod instances;
mod users;

use actix_cors::Cors;
use actix_web::{middleware, post, web::Json, App, HttpResponse, HttpServer};
use api_error::ApiError;
use auth::Claim;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

use crate::json::ErrorBody;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // for loading in admin username and password
    dotenv::from_filename(".env.local").ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    db::init();

    info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .supports_credentials();
        App::new()
            .wrap(auth::middleware::Authorize)
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(register)
            .configure(auth::routes::init_routes)
            .configure(accounts::routes::init_routes)
            .configure(users::routes::init_routes)
            .configure(instances::routes::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

/// Make sure user is requesting something within their account
pub fn belongs_to_account(jwt: &Option<Claim>, expected: &str) -> bool {
    if let Some(jwt) = jwt {
        // is true if user and expected id match or user is in admin account
        jwt.account_id == expected || jwt.account_id == "admin"
    } else {
        // if no jwt just let it through
        // means the request came from internally
        true
    }
}

lazy_static! {
    pub static ref PAYMENTS_URI: String =
        std::env::var("PAYMENTS_URI").unwrap_or("http://127.0.0.1:6000".into());
}

#[derive(Debug, Deserialize)]
struct RegisterBody {
    account: models::NewAccount,
    user: models::NewUser,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    account: models::Account,
    user: models::User,
}

#[post("/register")]
async fn register(data: Json<RegisterBody>) -> Result<HttpResponse, ApiError> {
    use diesel::prelude::*;
    let data = data.into_inner();
    let conn = db::connection()?;

    if data.account.id != data.user.account_id {
        return Ok(
            HttpResponse::BadRequest().json(ErrorBody::new("Account and user id do not match."))
        );
    }

    conn.transaction::<HttpResponse, ApiError, _>(|| {
        let with_id = models::NewAccount {
            id: nanoid!(ID_SIZE),
            ..data.account
        };
        let account: models::Account = diesel::insert_into(models::accounts::table)
            .values(&with_id)
            .get_result::<models::Account>(&conn)?;

        let with_hash = models::NewUser {
            password: bcrypt::hash(data.user.password, bcrypt::DEFAULT_COST)?,
            role: models::types::Role::Owner,
            account_id: account.id.clone(),
            ..data.user
        };
        let user: models::User = diesel::insert_into(models::users::table)
            .values(&with_hash)
            .get_result::<models::User>(&conn)?;

        Ok(HttpResponse::Ok().json(RegisterResponse { account, user }))
    })
}

#[cfg(test)]
mod tests;
