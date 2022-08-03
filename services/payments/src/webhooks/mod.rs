use hyper::{body, Body, Request, Response, StatusCode, Method};
use models::UpdateAccount;
use serde::Deserialize;
use stripe::{
    CreateUsageRecord, EventObject, EventType, Subscription, SubscriptionStatus, UsageRecord,
    UsageRecordAction, Webhook,
};

use crate::{
    error::ApiError, Client, CRUD_URI, INSTANCE_PRICE_ID, STRIPE_WEBHOOK_KEY, USER_PRICE_ID,
};

pub async fn handler(req: Request<Body>) -> Response<Body> {
    let (head, body) = req.into_parts();
    let payload_str = std::str::from_utf8(&body::to_bytes(body).await.unwrap())
        .unwrap()
        .to_string();

    let stripe_signature = head
        .headers
        .get("Stripe-Signature")
        .map(|v| v.to_str().unwrap())
        .unwrap_or_default();

    if let Ok(event) = Webhook::construct_event(&payload_str, stripe_signature, &STRIPE_WEBHOOK_KEY)
    {
        let client = Client::new();
        match event.event_type {
            EventType::CustomerSubscriptionDeleted => {
                if let EventObject::Subscription(sub) = event.data.object {
                    tokio::spawn(handle_sub_delete(client.clone(), sub));
                }
            }
            EventType::CustomerSubscriptionUpdated => {
                if let EventObject::Subscription(sub) = event.data.object {
                    tokio::spawn(handle_sub_update(client.clone(), sub));
                }
            }
            _ => {
                tracing::info!(
                    "Unknown event encountered in webhook: {:?}",
                    event.event_type
                );
            }
        }
    } else {
        tracing::error!(
            "Failed to construct webhook event, ensure your webhook secret is correct."
        );
    }

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::empty())
        .unwrap()
}

async fn handle_sub_delete(client: Client, sub: Subscription) {
    let account = sub_user(&client, &sub).await;
    if let Some(account) = account {
        let req = Request::builder()
            .uri(CRUD_URI.to_string() + "/accounts/" + account.id.as_str())
            .body(Body::from(
                serde_json::to_string(&UpdateAccount {
                    sub_id: Some(None),
                    ..Default::default()
                })
                .unwrap(),
            ))
            .unwrap();

        let res = client.request(req).await;

        if let Ok(res) = res {
            if res.status().is_success() {
                tracing::info!("Successfully canceled sub for {}", account.business_name);
            } else {
                tracing::error!("Failed to update Account and cancel subscription.");
                // TODO some kinda notification for me
            }
        } else {
            tracing::error!("Failed to update Account and cancel subscription.");
            // TODO some kinda notification for me
        }
    }
}

async fn handle_sub_update(client: Client, sub: Subscription) -> Result<(), ApiError> {
    if sub.status == SubscriptionStatus::Canceled || sub.status == SubscriptionStatus::Unpaid {
        // Their subscription is bad so we revoke access 😈
        handle_sub_delete(client, sub).await;
    } else if sub.status == SubscriptionStatus::PastDue {
        // TODO notify account holder of subscription is past due
    } else if sub.status == SubscriptionStatus::Active {
        // Make sure usage records are up to date with current usage
        let account = sub_user(&client, &sub).await;
        if let Some(account) = account {
            let user_sub_item = sub.items.data.iter().find(|&item| {
                if let Some(price) = item.price.as_ref() {
                    price.id == USER_PRICE_ID.as_str()
                } else {
                    false
                }
            });

            let stripe = stripe::Client::new(crate::STRIPE_KEY.as_str());
            let req = Request::builder()
                .method(Method::GET)
                .uri(CRUD_URI.to_string() + "/accounts/" + account.id.as_str() + "/users")
                .body(Body::empty())
                .unwrap();

            let res = client.request(req).await?;
            #[derive(Deserialize)]
            struct MinUser {}
            let users =
                serde_json::from_slice::<Vec<MinUser>>(&body::to_bytes(res.into_body()).await?)?;

            // make sure usage is up to date for user
            UsageRecord::create(
                &stripe,
                &user_sub_item.unwrap().id,
                CreateUsageRecord {
                    action: Some(UsageRecordAction::Set),
                    quantity: users.len().try_into().unwrap(),
                    ..Default::default()
                },
            );

            let instance_sub_item = sub.items.data.iter().find(|&item| {
                if let Some(price) = item.price.as_ref() {
                    price.id == INSTANCE_PRICE_ID.as_str()
                } else {
                    false
                }
            });

            let stripe = stripe::Client::new(crate::STRIPE_KEY.as_str());
            let req = Request::builder()
                .method(Method::GET)
                .uri(CRUD_URI.to_string() + "/accounts/" + account.id.as_str() + "/instances")
                .body(Body::empty())
                .unwrap();

            let res = client.request(req).await?;
            #[derive(Deserialize)]
            struct MinInstance {}
            let users = serde_json::from_slice::<Vec<MinInstance>>(
                &body::to_bytes(res.into_body()).await?,
            )?;

            UsageRecord::create(
                &stripe,
                &instance_sub_item.unwrap().id,
                CreateUsageRecord {
                    action: Some(UsageRecordAction::Set),
                    quantity: users.len().try_into().unwrap(),
                    ..Default::default()
                },
            );
        }
    };

    Ok(())
}

async fn sub_user(client: &Client, sub: &Subscription) -> Option<models::Account> {
    let req = Request::builder()
        .method(Method::GET)
        .uri(CRUD_URI.to_string() + "/accounts/by-sub/" + sub.id.as_str())
        .body(Body::empty())
        .unwrap();
    let res = client.request(req).await.ok();

    if let Some(res) = res {
        serde_json::from_slice::<models::Account>(&body::to_bytes(res.into_body()).await.unwrap())
            .ok()
    } else {
        None
    }
}
