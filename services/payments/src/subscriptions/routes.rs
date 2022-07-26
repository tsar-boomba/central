use axum::{Extension, Json};
use hyper::{Body, Response};
use serde::Deserialize;

use crate::crud_models;

#[derive(Debug, Deserialize)]
pub struct CreateAccount {
    account: crud_models::Account,
    /// Card number
    number: String,
    exp_year: i32,
    exp_month: i32,
    /// 3 numbers on the back
    cvc: String,
}

pub async fn subscribe(
    Json(data): Json<CreateAccount>,
    Extension(stripe): Extension<stripe::Client>,
) -> Response<Body> {
    todo!()
}
