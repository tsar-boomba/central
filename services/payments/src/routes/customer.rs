use std::str::FromStr;

use auth::{belongs_to_account, ExtractReqUser};
use axum::{
    extract::Path,
    routing::{post, put},
    Extension, Json, Router,
};
use hyper::{Body, Method, Request, Response, StatusCode};
use payments_lib::routes::customer;
use stripe::{Address, CreateCustomer, Customer, CustomerId, UpdateCustomer};

use crate::{error::ApiError, Client, CRUD_URI};

async fn create_customer(
    Json(account): Json<customer::CustomerParams>,
    Extension(stripe): Extension<stripe::Client>,
    Extension(client): Extension<Client>,
    ExtractReqUser(req_user): ExtractReqUser,
) -> Result<Response<Body>, ApiError> {
    if !belongs_to_account(&req_user, &account.id) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(
                r#"{"message":"You cannot access this resource."}"#,
            ))
            .unwrap());
    }

    if account.stripe_id.is_some() {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Cannot create a customer that already exists.",
        ));
    }

    let customer = Customer::create(
        &stripe,
        CreateCustomer {
            name: Some(&account.business_name),
            email: Some(&account.email),
            address: Some(Address {
                city: Some(account.city.clone()),
                country: Some("US".to_string()),
                line1: Some(account.address.clone()),
                postal_code: Some(account.zip_code.clone()),
                state: Some(account.state.clone()),
                ..Default::default()
            }),
            phone: Some(&account.phone_number),
            ..Default::default()
        },
    )
    .await?;

    let req = Request::builder()
        .uri(format!("{}/accounts/{}", CRUD_URI.as_str(), account.id))
        .method(Method::PUT)
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&models::UpdateAccount {
            stripe_id: Some(Some(customer.id.to_string())),
            ..Default::default()
        })?))
        .unwrap();

    let res = client.request(req).await?;

    if !res.status().is_success() {
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(
                r#"{"message": "Failed to update account with customer id."}"#,
            ))
            .unwrap());
    }

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(customer.id.to_string()))
        .unwrap())
}

async fn update_customer(
    Json(account): Json<customer::update::UpdateCustomerParams>,
    Path(id): Path<String>,
    Extension(stripe): Extension<stripe::Client>,
) -> Result<Response<Body>, ApiError> {
    let parsed_id = CustomerId::from_str(id.as_str())?;
    Customer::update(
        &stripe,
        &parsed_id,
        UpdateCustomer {
            name: account.business_name.as_deref(),
            email: account.email.as_deref(),
            address: Some(Address {
                city: account.city,
                line1: account.address,
                postal_code: account.zip_code,
                state: account.state,
                ..Default::default()
            }),
            phone: account.phone_number.as_deref(),
            ..Default::default()
        },
    )
    .await?;

    Ok(Response::builder().status(StatusCode::OK).body(Body::empty()).unwrap())
}

pub fn init() -> Router {
    Router::new()
        .route("/customer", post(create_customer))
        .route("/customer/:id", put(update_customer))
}
