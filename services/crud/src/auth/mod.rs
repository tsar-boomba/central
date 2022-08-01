pub mod middleware;
pub mod routes;

use chrono::prelude::*;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

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

lazy_static! {
    static ref JWT_SECRET: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("No JWT_SECRET env variable!")
            .as_bytes(),
    )
    .expect("Failed to encode secret");
}

fn sign(id: i32, account_id: String) -> Result<String, jwt::Error> {
    let claim = Claim::new(id, account_id);
    claim.sign_with_key(&*JWT_SECRET)
}

fn verify(token: &String) -> Result<Claim, jwt::Error> {
    let result: Result<Claim, jwt::Error> = token.verify_with_key(&*JWT_SECRET);
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
