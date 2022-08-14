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

mod accounts;
mod instances;
mod users;

use actix_web::{middleware, post, web::{Json, self}, App, HttpResponse, HttpServer};
use api_error::ApiError;
use dotenv::dotenv;
use models::{Account, Validate};
use payments_lib::routes::create_usage_record;
use serde::{Deserialize, Serialize};

use crate::json::ErrorBody;

#[derive(Debug, Clone)]
struct AppData {
    pub eb_client: aws_sdk_elasticbeanstalk::Client,
    pub r53_client: aws_sdk_route53::Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // for loading in admin username and password
    dotenv::from_filename(".env.local").ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    db::init();

    let aws_creds = aws_config::load_from_env().await;
    let eb_client = aws_sdk_elasticbeanstalk::Client::new(&aws_creds);
    let r53_client = aws_sdk_route53::Client::new(&aws_creds);

    let app_data = AppData { eb_client, r53_client };

    info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(auth::middleware::Authorize)
            .wrap(middleware::Logger::default())
            .service(register)
            .configure(auth::routes::init_routes)
            .configure(accounts::routes::init_routes)
            .configure(users::routes::init_routes)
            .configure(instances::routes::init_routes)
            .app_data(web::Data::new(app_data.clone()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

lazy_static! {
    pub static ref PAYMENTS_URI: String =
        std::env::var("PAYMENTS_URI").unwrap_or("http://127.0.0.1:6000".into());
    pub static ref INSTANCES_URI: String =
        std::env::var("INSTANCES_URI").unwrap_or("http://127.0.0.1:3001".into());
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
        with_id.validate()?;
        let account: models::Account = diesel::insert_into(models::accounts::table)
            .values(&with_id)
            .get_result::<models::Account>(&conn)?;

        let with_hash = models::NewUser {
            password: bcrypt::hash(data.user.password, bcrypt::DEFAULT_COST)?,
            role: models::types::Role::Owner,
            account_id: account.id.clone(),
            ..data.user
        };
        with_hash.validate()?;
        let user: models::User = diesel::insert_into(models::users::table)
            .values(&with_hash)
            .get_result::<models::User>(&conn)?;

        Ok(HttpResponse::Ok().json(RegisterResponse { account, user }))
    })
}

fn update_usage(
    owner: &Account,
    resource: String,
    new_value: i64,
) -> Result<reqwest::blocking::Response, ApiError> {
    let client = reqwest::blocking::Client::new();

    return client
        .post(PAYMENTS_URI.to_string() + create_usage_record::ROUTE)
        .header("Content-Type", "application/json")
        .json(&create_usage_record::CreateUsageRecordParams {
            sub_id: owner.sub_id.clone().unwrap(),
            number: new_value.try_into().unwrap(),
            resource,
        })
        .send()
        .map_err(|e| e.into());
}

#[cfg(test)]
mod tests;
