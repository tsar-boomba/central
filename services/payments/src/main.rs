mod error;
mod subscriptions;
mod webhooks;


#[macro_use]
extern crate lazy_static;

use axum::{routing::post, Extension, Router};
use hyper::{client::HttpConnector, Body};
use std::net::SocketAddr;

pub type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let http_client = Client::new();
    let stripe = stripe::Client::new(STRIPE_KEY.to_string());

    let app = Router::new()
        .route("/subscribe", post(subscriptions::routes::subscribe))
        .route(
            "/create-usage-record",
            post(subscriptions::routes::create_usage_record),
        )
        .route("/webhooks", post(webhooks::handler))
        .layer(Extension(http_client))
        .layer(Extension(stripe));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6000));
    tracing::debug!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

lazy_static! {
    pub static ref STRIPE_KEY: String = std::env::var("STRIPE_KEY").unwrap();
    pub static ref CRUD_URI: String =
        std::env::var("CRUD_URI").unwrap_or("http://127.0.0.1:8080".into());
    pub static ref INSTANCE_PRICE_ID: String = if STRIPE_KEY.contains("test") {
        "price_1LP8pMAMMTQqCw55f1MxzIjC".to_string()
    } else {
        "".to_string()
    };
    pub static ref USER_PRICE_ID: String = if STRIPE_KEY.contains("test") {
        "price_1LP8mmAMMTQqCw55U0urmth4".to_string()
    } else {
        "".to_string()
    };
}

#[cfg(test)]
mod tests;
