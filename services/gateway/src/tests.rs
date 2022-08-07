use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use axum::{
    routing::{get, post},
    Json, Router,
};
use hyper::{body, Body, Method, Request, StatusCode};
use models::types::Role;
use serde_json::from_slice;

use crate::{app, Client};

lazy_static! {
    static ref INITIATED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

fn init() {
    let mut initiated = INITIATED.lock().unwrap();
    if *initiated == false {
        std::env::set_var("RUST_LOG", "info");
        tracing_subscriber::fmt::init();
        *initiated = true;
    }
}

fn init_e2e() {
    // start mock servers
    tokio::spawn(mock_crud());
    tokio::spawn(mock_payments());

    // start main
    tokio::spawn(app(4001));
}

async fn mock_crud() {
    let test_addr = "127.0.0.1:8081";
    std::env::set_var("CRUD_URI", "http://".to_string() + test_addr);
    let app = Router::new()
        .route(
            "/verify",
            get(|| async {
                Json(auth::ReqUser {
                    id: 10,
                    account_id: "account_id".into(),
                    role: Role::User,
                })
            }),
        )
        .route(
            "/users",
            get(|| async { Json::<Vec<auth::ReqUser>>(vec![]) }),
        );

    let addr = SocketAddr::from(test_addr.parse::<SocketAddr>().unwrap());
    println!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

const WEBHOOK_BODY: &str = "Web hooked";

async fn mock_payments() {
    let test_addr = "127.0.0.1:6001";
    std::env::set_var("PAYMENTS_URI", "http://".to_string() + test_addr);
    let app = Router::new()
        .route(
            "/subscriptions",
            get(|| async { Json(Vec::<String>::new()) }),
        )
        .route("/webhooks", post(|| async { WEBHOOK_BODY }));

    let addr = SocketAddr::from(test_addr.parse::<SocketAddr>().unwrap());
    println!("server listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[tokio::test]
async fn e2e() {
    init();
    init_e2e();
    let client = Client::new();

    // expect unverified
    tracing::info!("Testing crud users.");
    let req = Request::builder()
        .method(Method::GET)
        .uri("http://localhost:4001/users")
        .header("Cookie", "noop=noop")
        .body(Body::empty())
        .unwrap();
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_json: Vec<auth::ReqUser> =
        from_slice(&body::to_bytes(res.into_body()).await.unwrap()).unwrap();

    assert_eq!(body_json, vec![]);

    // test verify
    tracing::info!("Testing crud verify.");
    let res = client
        .get("http://localhost:4001/verify".try_into().unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body_json: auth::ReqUser =
        from_slice(&body::to_bytes(res.into_body()).await.unwrap()).unwrap();

    assert_eq!(
        body_json,
        auth::ReqUser {
            id: 10,
            account_id: "account_id".into(),
            role: Role::User
        }
    );

    // test payments
    tracing::info!("Testing payments webhooks.");
    let req = Request::builder()
        .method(Method::POST)
        .uri("http://localhost:4001/payments/webhooks")
        .header("Cookie", "noop=noop")
        .body(Body::empty())
        .unwrap();
    let res = client.request(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let body = &body::to_bytes(res.into_body()).await.unwrap();

    assert_eq!(body, WEBHOOK_BODY);
}
