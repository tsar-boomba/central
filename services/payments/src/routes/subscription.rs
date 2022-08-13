use std::str::FromStr;

use auth::{belongs_to_account, require_role, ExtractReqUser};
use axum::{routing::post, Extension, Json, Router};
use hyper::{body, Body, Method, Request, Response, StatusCode};
use models::types::Role;
use payments_lib::routes::subscription::{CreateSubscriptionParams, UpdateSubscriptionParams};
use serde::Deserialize;
use stripe::{
    AttachPaymentMethod, CreateSubscription, CreateSubscriptionItems,
    CreateSubscriptionPaymentSettings, CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod,
    CreateUsageRecord, Customer, CustomerId, CustomerInvoiceSettings, PaymentMethod,
    PaymentMethodId, Subscription, SubscriptionId, SubscriptionStatus, UpdateCustomer, UsageRecord,
    UsageRecordAction,
};

use crate::{error::ApiError, Client, CRUD_URI, INSTANCE_PRICE_ID, USER_PRICE_ID};

async fn subscribe(
    Json(data): Json<CreateSubscriptionParams>,
    Extension(stripe): Extension<stripe::Client>,
    Extension(client): Extension<Client>,
    ExtractReqUser(req_user): ExtractReqUser,
) -> Result<Response<Body>, ApiError> {
    if !belongs_to_account(&req_user, &data.account.id) || !require_role(&req_user, Role::Owner) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(
                r#"{"message":"You cannot access this resource."}"#,
            ))
            .unwrap());
    }

    if data.account.stripe_id == None {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(
                r#"{"message":"Account does not have a customer id."}"#,
            ))
            .unwrap());
    }

    let req = Request::builder()
        .uri(format!(
            "{}/accounts/{}/usage",
            CRUD_URI.as_str(),
            data.account.id
        ))
        .method(Method::GET)
        .body(Body::empty())
        .unwrap();

    let res = client.request(req).await?;
    // TODO define api for crud
    #[derive(Deserialize)]
    struct Usage {
        pub users: u64,
        pub instances: u64,
    }
    let usage = serde_json::from_slice::<Usage>(&body::to_bytes(res.into_body()).await.unwrap())?;

    let parsed_payment_id = PaymentMethodId::from_str(&data.payment_method_id)?;
    let customer_id = data.account.stripe_id.unwrap();
    let parsed_customer_id = CustomerId::from_str(&customer_id)?;
    let customer = Customer::retrieve(&stripe, &parsed_customer_id, &[]).await?;
    let expansions = &["pending_setup_intent", "latest_invoice.payment_intent"];
    PaymentMethod::attach(
        &stripe,
        &parsed_payment_id,
        AttachPaymentMethod {
            customer: parsed_customer_id.clone(),
        },
    )
    .await?;
    Customer::update(
        &stripe,
        &parsed_customer_id,
        UpdateCustomer {
            invoice_settings: Some(CustomerInvoiceSettings {
                default_payment_method: Some(parsed_payment_id.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await?;
    // get or create subscription
    let subscription = if data.account.sub_id == None && customer.subscriptions.data.len() < 1 {
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
        params.payment_settings = Some(CreateSubscriptionPaymentSettings {
            save_default_payment_method: Some(
                CreateSubscriptionPaymentSettingsSaveDefaultPaymentMethod::OnSubscription,
            ),
            ..Default::default()
        });
        params.expand = expansions;

        Subscription::create(&stripe, params).await?
    } else {
        Subscription::retrieve(
            &stripe,
            &SubscriptionId::from_str(&data.account.sub_id.clone().unwrap())?,
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

    let instance_sub_item = subscription.items.data.iter().find(|&item| {
        if let Some(price) = item.price.as_ref() {
            price.id == INSTANCE_PRICE_ID.as_str()
        } else {
            false
        }
    });

    UsageRecord::create(
        &stripe,
        &user_sub_item.unwrap().id,
        CreateUsageRecord {
            action: Some(UsageRecordAction::Set),
            quantity: usage.users,
            ..Default::default()
        },
    )
    .await?;

    UsageRecord::create(
        &stripe,
        &instance_sub_item.unwrap().id,
        CreateUsageRecord {
            action: Some(UsageRecordAction::Set),
            quantity: usage.instances,
            ..Default::default()
        },
    )
    .await?;

    if data.account.sub_id == None {
        let req = Request::builder()
            .uri(format!(
                "{}/accounts/{}",
                CRUD_URI.as_str(),
                data.account.id
            ))
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

    tracing::info!("Created subscription for {}", data.account.business_name);

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&subscription)?))
        .unwrap())
}

async fn update_subscription(
    Json(data): Json<UpdateSubscriptionParams>,
    Extension(stripe): Extension<stripe::Client>,
    ExtractReqUser(req_user): ExtractReqUser,
) -> Result<Response<Body>, ApiError> {
    if !belongs_to_account(&req_user, &data.account.id) || !require_role(&req_user, Role::Owner) {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from(
                r#"{"message":"You cannot access this resource."}"#,
            ))
            .unwrap());
    }

    if data.account.stripe_id == None || data.account.sub_id == None {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(
                r#"{"message":"Account does not have a customer or subscription id."}"#,
            ))
            .unwrap());
    }

    let parsed_payment_id = PaymentMethodId::from_str(&data.payment_method_id)?;
    let customer_id = data.account.stripe_id.unwrap();
    let parsed_customer_id = CustomerId::from_str(&customer_id)?;

    // update payment method
    PaymentMethod::attach(
        &stripe,
        &parsed_payment_id,
        AttachPaymentMethod {
            customer: parsed_customer_id.clone(),
        },
    )
    .await?;
    Customer::update(
        &stripe,
        &parsed_customer_id,
        UpdateCustomer {
            invoice_settings: Some(CustomerInvoiceSettings {
                default_payment_method: Some(parsed_payment_id.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        },
    )
    .await?;

    let sub_id = data.account.sub_id.unwrap();
    let parsed_sub_id = SubscriptionId::from_str(&sub_id)?;
    let expansions = &["pending_setup_intent", "latest_invoice.payment_intent"];
    let sub = Subscription::retrieve(&stripe, &parsed_sub_id, expansions).await?;

    if sub.status == SubscriptionStatus::PastDue || sub.status == SubscriptionStatus::Unpaid {
        // get missed invoice
        let invoice = sub.latest_invoice.unwrap().into_object().unwrap();

        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(
                serde_json::to_string(&serde_json::json!({ "invoice": invoice })).unwrap(),
            ))
            .unwrap())
    } else {
        // just return success!!
        Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("{}"))
            .unwrap())
    }
}

pub fn init() -> Router {
    Router::new().route("/", post(subscribe).put(update_subscription))
}
