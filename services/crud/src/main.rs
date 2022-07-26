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
#[macro_use]
extern crate models;

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
use actix_web::{middleware, App, HttpServer};
use auth::Claim;
use dotenv::dotenv;

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

#[cfg(test)]
mod tests;
