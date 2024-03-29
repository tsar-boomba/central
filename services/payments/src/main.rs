mod error;
mod routes;
mod webhooks;

#[macro_use]
extern crate lazy_static;

use axum::{
    routing::{get, post},
    Extension, Router,
};
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
        .nest("/subscription", routes::subscription::init())
        .nest("/customer", routes::customer::init())
        .route("/create-usage-record", post(routes::create_usage_record))
        .route("/sub-status", get(routes::sub_status))
        .route("/webhooks", post(webhooks::handler))
        .layer(Extension(http_client))
        .layer(Extension(stripe))
        .layer(TraceLayer::new_for_http());

    let addr = SocketAddr::from((if *PROD { [0, 0, 0, 0] } else { [127, 0, 0, 1] }, 6000));
    tracing::info!("Server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

lazy_static! {
    pub static ref STRIPE_KEY: String = std::env::var("STRIPE_KEY").unwrap();
    pub static ref STRIPE_WEBHOOK_KEY: String = std::env::var("STRIPE_WEBHOOK_KEY").unwrap();
    static ref PROD: bool = std::env::var("RUST_ENV").unwrap_or("dev".into()) == "prod";
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
