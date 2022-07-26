use std::{convert::Infallible, net::IpAddr};

use axum::http::{Request, Response};
use hyper::{Body, StatusCode};

use crate::{proxy_call, Client};

pub const PATH_BASE: &str = "/payments";

/// Paths which can only be accessed by other services
const PRIVATE_PATHS: [&str; 1] = ["/create-usage-record"];

lazy_static! {
    pub static ref URI: String =
        std::env::var("PAYMENTS_URI").unwrap_or("http://127.0.0.1:6000".into());
}

pub async fn proxy(
    client_ip: IpAddr,
    _client: Client,
    req: Request<Body>,
    path: String,
) -> Result<Response<Body>, Infallible> {
    if !PRIVATE_PATHS.contains(&path.as_str()) {
		// Allow reqs from outside
        Ok(proxy_call(client_ip, URI.as_str(), req).await)
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
