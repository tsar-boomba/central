use std::{convert::Infallible, net::IpAddr};

use axum::http::{Request, Response};
use hyper::{Body, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{proxy_call, Client, authorize_req};

pub const PATH_BASE: &str = "/";

const PUBLIC_PATHS: [&str; 3] = ["/verify", "/login", "/authenticate"];

pub fn uri() -> String { std::env::var("CRUD_URI").unwrap_or("http://127.0.0.1:8080".into()) }

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Resource {
    Load,
    Carrier,
    Shipper,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
	create_perms: Vec<Resource>,
    update_perms: Vec<Resource>,
    delete_perms: Vec<Resource>,
}

pub async fn proxy(client_ip: IpAddr, client: Client, req: Request<Body>, path: String) -> Result<Response<Body>, Infallible> {
	let uri = uri();

	if PUBLIC_PATHS.contains(&path.as_str()) {
		// do not authorize request
		return Ok(proxy_call(client_ip, uri.as_str(), req).await);
	} else {
		return match authorize_req(&client, &req).await {
			// request was authed
			Some(_) => {
				Ok(proxy_call(client_ip, uri.as_str(), req).await)
			},
			// request was not authed
			_ => {
				Ok(
					Response::builder()
						.status(StatusCode::UNAUTHORIZED)
						.body(Body::from(r#"{ "message": "You are not authorized! Login before accessing this resource." }"#))
						.unwrap()
				)
			}
		}
	}
}
