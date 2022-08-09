mod error;
mod subscriptions;
mod webhooks;

#[macro_use]
extern crate lazy_static;

use axum::{routing::{post, get}, Extension, Router};
use hyper::{client::HttpConnector, Body};
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

pub type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "error,info");
    tracing_subscriber::fmt::init();

    let http_client = Client::new();
    let stripe = stripe::Client::new(STRIPE_KEY.to_string());

    let app = Router::new()
        .route("/subscribe", post(subscriptions::routes::subscribe))
        .route("/customer", post(subscriptions::routes::customer))
        .route(
            "/create-usage-record",
            post(subscriptions::routes::create_usage_record),
        )
        .route("/is-subbed", get(subscriptions::routes::is_subbed))
        .route("/webhooks", post(webhooks::handler))
        .layer(Extension(http_client))
        .layer(Extension(stripe))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from(([127, 0, 0, 1], 6000));
    tracing::info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

lazy_static! {
    pub static ref STRIPE_KEY: String = std::env::var("STRIPE_KEY").unwrap();
    pub static ref STRIPE_WEBHOOK_KEY: String = std::env::var("STRIPE_WEBHOOK_KEY").unwrap();
    pub static ref CRUD_URI: String =
        std::env::var("CRUD_URI").unwrap_or("http://127.0.0.1:8080".into());
    pub static ref INSTANCE_PRICE_ID: String = if STRIPE_KEY.contains("test") {
        "price_1LP8pMAMMTQqCw55f1MxzIjC".to_string()
    } else {
        // TODO prod ids
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
