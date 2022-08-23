use std::{convert::Infallible, net::IpAddr};

use axum::http::{HeaderValue, Request, Response};
use hyper::{Body, StatusCode};
use regex::RegexSet;

use crate::{authorize_req, error_body, proxy_call, Client};

pub const PATH_BASE: &str = "/payments";

lazy_static! {
    /// Paths which don't need to be authenticated
    static ref PUBLIC_PATH_RE: RegexSet = RegexSet::new(&["^/webhooks/?$"]).unwrap();
    /// Paths which can only be accessed by other services
    static ref PRIVATE_PATH_RE: RegexSet =
        RegexSet::new(&["^/create-usage-record/?$", r"^/customer/.*/?$"]).unwrap();
}

lazy_static! {
    pub static ref URI: String =
        std::env::var("PAYMENTS_URI").unwrap_or("http://127.0.0.1:6000".into());
}

pub async fn proxy(
    client_ip: IpAddr,
    client: Client,
    mut req: Request<Body>,
    path: String,
) -> Result<Response<Body>, Infallible> {
    if PUBLIC_PATH_RE.is_match(path.as_str()) {
        Ok(proxy_call(client_ip, URI.as_str(), req).await)
    } else if !PRIVATE_PATH_RE.is_match(path.as_str()) {
        return match authorize_req(&client, &req).await {
            // request was authed
            Some(user) => {
                req.headers_mut().append(
                    "user",
                    HeaderValue::from_str(&serde_json::to_string(&user).unwrap()).unwrap(),
                );
                Ok(proxy_call(client_ip, URI.as_str(), req).await)
            }
            // request was not authed
            _ => Ok(Response::builder()
                .status(StatusCode::UNAUTHORIZED)
                .body(Body::from(error_body(
                    "You are not authorized! Login before accessing this resource.",
                )))
                .unwrap()),
        };
    } else {
        // Only reqs from internal services (crud, instance-deploy, etc.) allowed
        Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from(format!(
                r#"{{"message": "uri: {} is not valid"}}"#,
                path
            )))
            .unwrap())
    }
}
