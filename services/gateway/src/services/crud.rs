use std::{convert::Infallible, net::IpAddr};

use axum::http::{Request, Response};
use hyper::{body, Body, Method, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{authorize_req, error_body, proxy_call, Client};

pub const PATH_BASE: &str = "/";

const PUBLIC_PATHS: [&str; 3] = ["/verify", "/login", "/authenticate"];

lazy_static! {
    pub static ref URI: String =
        std::env::var("CRUD_URI").unwrap_or("http://127.0.0.1:8080".into());
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Resource {
    Load,
    Carrier,
    Shipper,
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub create_perms: Vec<Resource>,
    pub update_perms: Vec<Resource>,
    pub delete_perms: Vec<Resource>,
}

pub async fn proxy(
    client_ip: IpAddr,
    client: Client,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, Infallible> {
    if PUBLIC_PATHS.contains(&path.as_str()) {
        // do not authorize request
        return Ok(proxy_call(client_ip, URI.as_str(), req).await);
    } else {
        return match authorize_req(&client, &req).await {
            // request was authed
            Some(_) => Ok(proxy_call(client_ip, URI.as_str(), req).await),
            // request was not authed
            _ => Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(error_body(
                    "You are not authorized! Login before accessing this resource.",
                )))
                .unwrap()),
        };
    }
}

#[derive(Debug, Deserialize)]
struct RegisterBody {
    account: models::NewAccount,
    user: models::NewUser,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    account: models::Account,
    user: models::User,
}

pub async fn register(client: Client, req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let body =
        serde_json::from_slice::<RegisterBody>(&body::to_bytes(req.into_body()).await.unwrap());

    if let Err(err) = body {
        tracing::error!("{:?}", err);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::empty())
            .unwrap());
    }

    let body = body.unwrap();

    if body.account.id != body.user.account_id {
        tracing::error!("Account and user id do not match.");
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(error_body("Account and user id do not match.")))
            .unwrap());
    }

    let acct_req = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/accounts", URI.as_str()))
        .body(Body::from(serde_json::to_string(&body.account).unwrap()))
        .unwrap();

    let acct_res = client.request(acct_req).await;

    if let Err(err) = acct_res {
        tracing::error!("{:?}", err);
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(error_body(
                "Something went wrong creating the account.",
            )))
            .unwrap());
    }

    let acct_res = acct_res.unwrap();

    if !acct_res.status().is_success() {
        tracing::error!("Something went wrong creating account.");
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from(error_body(
                "Something went wrong creating the account.",
            )))
            .unwrap());
    }

    // removes acct and indicates whether it was a success
    async fn remove_acct(acct_bytes: Body, client: Client) -> bool {
        let acct =
            serde_json::from_slice::<models::Account>(&body::to_bytes(acct_bytes).await.unwrap())
                .unwrap();

        let delete_req = Request::builder()
            .uri(format!("{}/accounts/{}", URI.as_str(), acct.id))
            .body(Body::empty())
            .unwrap();

        let delete_res = client.request(delete_req).await;

        match delete_res {
            Ok(res) => res.status().is_success(),
            Err(err) => {
                tracing::error!("{:?}", err);
                false
            }
        }
    }

    let user_req = Request::builder()
        .method(Method::POST)
        .uri(format!("{}/users", self::URI.as_str()))
        .body(Body::from(serde_json::to_string(&body.user).unwrap()))
        .unwrap();

    let user_res = client.request(user_req).await;

    if let Err(err) = user_res {
        tracing::error!("{:?}", err);
        let success = remove_acct(acct_res.into_body(), client).await;
        return if success {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_body(
                    "Something went wrong creating the user.",
                )))
                .unwrap())
        } else {
            tracing::error!("Registration went horribly wrong.");
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(r#"{fail:true}"#))
                .unwrap())
        };
    }

    let user_res = user_res.unwrap();

    if !user_res.status().is_success() {
        tracing::error!("Failed to create the user.");
        let success = remove_acct(acct_res.into_body(), client).await;
        return if success {
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(error_body(
                    "Something went wrong creating the user.",
                )))
                .unwrap())
        } else {
            tracing::error!("Registration went horribly wrong.");
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(r#"{fail:true}"#))
                .unwrap())
        };
    }

    let account = serde_json::from_slice::<models::Account>(
        &body::to_bytes(acct_res.into_body()).await.unwrap(),
    )
    .unwrap();

    let user = serde_json::from_slice::<models::User>(
        &body::to_bytes(user_res.into_body()).await.unwrap(),
    )
    .unwrap();

    let res_body = RegisterResponse { account, user };

    Ok(Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(serde_json::to_string(&res_body).unwrap()))
        .unwrap())
}
