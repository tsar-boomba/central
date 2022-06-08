use axum::Json;
use hyper::{Response, Body};

pub async fn subscribe(Json(data): Json<()>) -> Response<Body> {
	todo!()
}