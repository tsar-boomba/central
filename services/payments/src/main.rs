mod subscriptions;

#[macro_use]
extern crate lazy_static;

use axum::{Router, routing::{post}};
use hyper::{client::HttpConnector, Body};
use std::{net::SocketAddr, time::Duration};

type Client = hyper::client::Client<HttpConnector, Body>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let main_client = Client::new();

    let app = Router::new()
        .route("/subscriptions", post(subscriptions::routes::subscribe));

    let addr = SocketAddr::from(([127, 0, 0, 1], 6000));
    tracing::debug!("Reverse proxy listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
