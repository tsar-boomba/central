pub mod customer;
pub mod subscription;

use std::str::FromStr;

use axum::{extract::Query, Extension, Json};
use hyper::{Body, Response, StatusCode};
use payments_lib::routes::{create_usage_record, is_subbed::IsSubbedQuery};
use stripe::{
    CreateUsageRecord, Subscription, SubscriptionId, SubscriptionStatus, UsageRecord,
    UsageRecordAction,
};

use crate::{error::ApiError, INSTANCE_PRICE_ID, USER_PRICE_ID};

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
        Subscription::retrieve(&stripe, &SubscriptionId::from_str(&data.sub_id)?, &[]).await?;

    let sub_item = subscription.items.data.iter().find(|&item| {
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
        &sub_item.unwrap().id,
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

pub async fn is_subbed(
    stripe: Extension<stripe::Client>,
    query: Query<IsSubbedQuery>,
) -> Result<Response<Body>, ApiError> {
    let sub_id = SubscriptionId::from_str(&query.sub_id)?;
    let sub = Subscription::retrieve(&stripe, &sub_id, &[]).await?;

    if sub.status != SubscriptionStatus::Active {
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap());
    };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap())
}
