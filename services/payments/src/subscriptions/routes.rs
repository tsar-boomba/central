use axum::{Extension, Json};
use hyper::{Body, Response, StatusCode};
use serde::Deserialize;
use stripe::{
    Address, AttachPaymentMethod, CardDetailsParams, CreateCustomer, CreatePaymentMethod,
    CreatePaymentMethodCardUnion, CreateSubscription, CreateSubscriptionItems, CreateUsageRecord,
    Customer, PaymentMethod, PaymentMethodTypeFilter, Subscription, UsageRecord, UsageRecordAction,
};

use crate::{crud_models, error::ApiError, STRIPE_KEY};

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
                country: Some("USA".to_string()),
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

    let instance_price_id = if STRIPE_KEY.contains("test") {
        "price_1LP8pMAMMTQqCw55f1MxzIjC".to_string()
    } else {
        "".to_string()
    };

    let user_price_id = if STRIPE_KEY.contains("test") {
        "price_1LP8mmAMMTQqCw55U0urmth4".to_string()
    } else {
        "".to_string()
    };

    let subscription = {
        let mut params = CreateSubscription::new(customer.id.clone());
        params.items = Some(vec![
            CreateSubscriptionItems {
                price: Some(instance_price_id),
                ..Default::default()
            },
            CreateSubscriptionItems {
                price: Some(user_price_id.clone()),
                ..Default::default()
            },
        ]);
        params.default_payment_method = Some(&payment_method.id);

        Subscription::create(&stripe, params).await?
    };

    let user_sub_item = subscription.items.data.iter().find(|&item| {
        if let Some(price) = item.price.as_ref() {
            price.id == user_price_id
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
        .body(Body::from(customer.id.to_string()))
        .unwrap())
}

#[derive(Debug, Deserialize)]
pub struct CreateUsage {
	stripe_id: String,
}

pub async fn create_usage(Json(data): Json<CreateUsage>) -> Result<Response<Body>, ApiError> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}
