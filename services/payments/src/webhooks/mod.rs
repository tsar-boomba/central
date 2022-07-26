use axum::Json;
use hyper::{body, Body, Request};
use stripe::Event;

pub async fn handler(Json(event): Json<Event>, req: Request<Body>) {
    let (head, body) = req.into_parts();
    let payload_str = std::str::from_utf8(&body::to_bytes(body).await.unwrap()).unwrap();

    println!("received event '{}' ({}) ", event.type_, event.id);
}
