use axum::{Json, Extension};
use hyper::{Response, Body};

pub async fn subscribe(Json(data): Json<()>, Extension(stripe): Extension<stripe::Client>) -> Response<Body> {
	todo!()
}