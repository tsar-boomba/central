pub mod middleware;
pub mod routes;

use actix_web::{dev, FromRequest, HttpMessage, HttpRequest};
use chrono::prelude::*;
use futures_util::future::{ready, Ready};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

use crate::api_error::ApiError;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Login {
    account_id: String,
    username: String,
    password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Claim {
    pub id: i32,
    pub account_id: String,
    pub exp: i64,
    pub iat: i64,
}

/// Milliseconds to token expiry
const TOKEN_EXPIRY: i64 = 43_200_000;

impl Claim {
    pub fn new(id: i32, account_id: String) -> Self {
        let now = Utc::now().timestamp_millis();
        // expires in 12 hours
        Self {
            id,
            account_id,
            exp: now + TOKEN_EXPIRY,
            iat: now,
        }
    }
}

impl FromRequest for Claim {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let value = req.extensions().get::<Option<Self>>().cloned().unwrap();

        // convert to result and return
        ready(value.ok_or(ApiError {
            status_code: 200,
            message: "".into(),
        }))
    }
}

fn get_secret() -> Hmac<Sha256> {
    Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("No JWT_SECRET env variable!")
            .as_bytes(),
    )
    .expect("Failed to encode secret")
}

fn sign(id: i32, account_id: String) -> Result<String, jwt::Error> {
    let claim = Claim::new(id, account_id);
    claim.sign_with_key(&get_secret())
}

fn verify(token: &String) -> Result<Claim, jwt::Error> {
    let result: Result<Claim, jwt::Error> = token.verify_with_key(&get_secret());
    match result {
        Ok(claim) => {
            if Utc::now().timestamp_millis() > claim.exp {
                return Err(jwt::Error::Format);
            }
            Ok(claim)
        }
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests;
