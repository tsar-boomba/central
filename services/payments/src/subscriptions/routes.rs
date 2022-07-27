use std::str::FromStr;

use axum::{Extension, Json};
use hyper::{Body, Response, StatusCode};
use payments_lib::routes::{self, subscribe};
use stripe::{
    Address, AttachPaymentMethod, CardDetailsParams, CreateCustomer, CreatePaymentMethod,
    CreatePaymentMethodCardUnion, CreateSubscription, CreateSubscriptionItems, CreateUsageRecord,
    Customer, PaymentMethod, PaymentMethodTypeFilter, Subscription, SubscriptionId, UsageRecord,
    UsageRecordAction,
};

use crate::{error::ApiError, INSTANCE_PRICE_ID, USER_PRICE_ID};

pub async fn subscribe(
    Json(data): Json<subscribe::SubscribeParams>,
    Extension(stripe): Extension<stripe::Client>,
) -> Result<Response<Body>, ApiError> {
    // Do this first to see if it fails
    let payment_method = PaymentMethod::create(
        &stripe,
        CreatePaymentMethod {
            type_: Some(PaymentMethodTypeFilter::Card),
            card: Some(CreatePaymentMethodCardUnion::CardDetailsParams(
                CardDetailsParams {
                    number: data.number.clone(), // UK visa
                    exp_year: data.exp_year,
                    exp_month: data.exp_month,
                    cvc: Some(data.cvc.clone()),
                    ..Default::default()
                },
            )),
            ..Default::default()
        },
    )
    .await?;

    let customer = Customer::create(
        &stripe,
        CreateCustomer {
            name: Some(&data.account.business_name),
            email: Some(&data.account.email),
            address: Some(Address {
                city: Some(data.account.city.clone()),
                country: Some("US".to_string()),
                line1: Some(data.account.address.clone()),
                postal_code: Some(data.account.zip_code.clone()),
                state: Some(data.account.state.clone()),
                ..Default::default()
            }),
            phone: Some(&data.account.phone_number),
            ..Default::default()
        },
    )
    .await?;

    PaymentMethod::attach(
        &stripe,
        &payment_method.id,
        AttachPaymentMethod {
            customer: customer.id.clone(),
        },
    )
    .await?;

    let subscription = {
        let mut params = CreateSubscription::new(customer.id.clone());
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
        params.default_payment_method = Some(&payment_method.id);

        Subscription::create(&stripe, params).await?
    };

    let user_sub_item = subscription.items.data.iter().find(|&item| {
        if let Some(price) = item.price.as_ref() {
            price.id == USER_PRICE_ID.as_str()
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
            quantity: 1,
            ..Default::default()
        },
    );

    tracing::debug!("{:?}", subscription);

    tracing::info!("Created subscription for {}", data.account.business_name);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(subscription.id.to_string()))
        .unwrap())
}

pub async fn create_usage_record(
    Json(data): Json<routes::create_usage_record::CreateUsageRecordParams>,
    Extension(stripe): Extension<stripe::Client>,
) -> Result<Response<Body>, ApiError> {
    if data.resource != "users" && data.resource != "instances" {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from("'resource' must be one of 'instances' or 'users'."))
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
