use std::str::FromStr;

use auth::{belongs_to_account, ExtractReqUser};
use axum::{Extension, Json};
use hyper::{body, Body, Method, Request, Response, StatusCode};
use payments_lib::routes::{
    create_usage_record, customer,
    subscribe::{SubscribeParams, SubscribeResponse},
};
use serde::Deserialize;
use stripe::{
    Address, CreateCustomer, CreateSubscription, CreateSubscriptionItems,
    CreateSubscriptionPaymentSettings, CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod,
    CreateUsageRecord, Customer, CustomerId, Subscription, SubscriptionId, UsageRecord,
    UsageRecordAction,
};

use crate::{error::ApiError, Client, CRUD_URI, INSTANCE_PRICE_ID, USER_PRICE_ID};

pub async fn customer(
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

pub async fn subscribe(
    Json(account): Json<SubscribeParams>,
    Extension(stripe): Extension<stripe::Client>,
    Extension(client): Extension<Client>,
    ExtractReqUser(req_user): ExtractReqUser,
) -> Result<Response<Body>, ApiError> {
    tracing::info!("subscribe runnign");
    if !belongs_to_account(&req_user, &account.id) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(
                r#"{"message":"You cannot access this resource."}"#,
            ))
            .unwrap());
    }

    if account.stripe_id == None {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(
                r#"{"message":"Account does not have a customer id."}"#,
            ))
            .unwrap());
    }

    let customer_id = account.stripe_id.unwrap();
    let customer = Customer::retrieve(&stripe, &CustomerId::from_str(&customer_id)?, &[]).await?;
    // get or create subscription
    let expansions = &["pending_setup_intent"];
    let subscription = if account.sub_id == None && customer.subscriptions.data.len() < 1 {
        let mut params = CreateSubscription::new(CustomerId::from_str(&customer_id)?);
        params.items = Some(vec![
            CreateSubscriptionItems {
                price: Some(INSTANCE_PRICE_ID.clone()),
                ..Default::default()
            },
            CreateSubscriptionItems {
                price: Some(USER_PRICE_ID.clone()),
                ..Default::default()
            },
        ]);
        params.payment_behavior = Some(stripe::SubscriptionPaymentBehavior::DefaultIncomplete);
        params.payment_settings = Some(CreateSubscriptionPaymentSettings {
            save_default_payment_method: Some(
                CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::OnSubscription,
            ),
            ..Default::default()
        });
        params.expand = expansions;
        //params.default_payment_method = Some(&payment_method.id);

        Subscription::create(&stripe, params).await?
    } else {
        Subscription::retrieve(
            &stripe,
            &SubscriptionId::from_str(&account.sub_id.clone().unwrap())?,
            expansions,
        )
        .await?
    };

    let user_sub_item = subscription.items.data.iter().find(|&item| {
        if let Some(price) = item.price.as_ref() {
            price.id == USER_PRICE_ID.as_str()
        } else {
            false
        }
    });

    let req = Request::builder()
        .uri(format!(
            "{}/accounts/{}/users",
            CRUD_URI.as_str(),
            account.id
        ))
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let res = client.request(req).await?;
    tracing::info!("{}", res.status());
    // just need to count them not actually the data
    #[derive(Deserialize)]
    struct Noop {
        pub id: i32,
    }
    let acct_users =
        serde_json::from_slice::<Vec<Noop>>(&body::to_bytes(res.into_body()).await.unwrap())?;

    // new subs always have at least one user
    UsageRecord::create(
        &stripe,
        &user_sub_item.unwrap().id,
        CreateUsageRecord {
            action: Some(UsageRecordAction::Set),
            quantity: acct_users.len().try_into().unwrap(),
            ..Default::default()
        },
    ).await?;

    if account.sub_id == None {
        let req = Request::builder()
            .uri(format!("{}/accounts/{}", CRUD_URI.as_str(), account.id))
            .method(Method::PUT)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&models::UpdateAccount {
                sub_id: Some(Some(subscription.id.to_string())),
                ..Default::default()
            })?))
            .unwrap();

        let res = client.request(req).await?;

        if !res.status().is_success() {
            return Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(
                    r#"{"message":"Failed to update account subscription id."}"#,
                ))
                .unwrap());
        }
    }

    tracing::debug!("{:?}", subscription);

    tracing::info!("Created subscription for {}", account.business_name);

    let client_secret = {
        subscription
            .pending_setup_intent
            .unwrap()
            .into_object()
            .unwrap()
            .client_secret
    };

    if let Some(client_secret) = client_secret {
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(serde_json::to_string(&SubscribeResponse {
                sub_id: subscription.id.to_string(),
                client_secret,
            })?))
            .unwrap())
    } else {
        Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(
                r#"{"message":"Failed to expand client secret."}"#,
            ))
            .unwrap())
    }
}

pub async fn create_usage_record(
    Json(data): Json<create_usage_record::CreateUsageRecordParams>,
    Extension(stripe): Extension<stripe::Client>,
) -> Result<Response<Body>, ApiError> {
    if data.resource != "users" && data.resource != "instances" {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(
                "'resource' must be one of 'instances' or 'users'.",
            ))
            .unwrap());
    };

    let subscription =
        Subscription::retrieve(&stripe, &SubscriptionId::from_str(&data.stripe_id)?, &[]).await?;

    let user_sub_item = subscription.items.data.iter().find(|&item| {
        if let Some(price) = item.price.as_ref() {
            price.id
                == (if data.resource == "users" {
                    USER_PRICE_ID.as_str()
                } else {
                    INSTANCE_PRICE_ID.as_str()
                })
        } else {
            false
        }
    });

    // new subs always have at least one user
    UsageRecord::create(
        &stripe,
        &user_sub_item.unwrap().id,
        CreateUsageRecord {
            action: Some(UsageRecordAction::Set),
            quantity: data.number,
            ..Default::default()
        },
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}
