pub mod error;

use models::types::Role;
use serde::{Deserialize, Serialize};

/// User data passed to services through "user" header
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ReqUser {
    pub id: i32,
    pub account_id: String,
    pub role: models::types::Role,
}

/// Takes a value (such as a header) and attempts to deserialize into ReqUser
pub fn extract(value: String) -> Option<ReqUser> {
    serde_json::from_str(&value).ok()
}

/// Make sure user is requesting something within their account
pub fn belongs_to_account(req_user: &Option<ReqUser>, expected: &str) -> bool {
    if let Some(req_user) = req_user {
        // is true if user and expected id match or user is in admin account
        req_user.account_id == expected || req_user.account_id == "admin"
    } else {
        // if no user just let it through
        // means the request came from internally
        true
    }
}

#[cfg(feature = "actix")]
impl actix_web::FromRequest for ReqUser {
    type Error = error::Error;
    type Future = std::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        use actix_web::HttpMessage;
        let value = req.extensions().get::<Option<Self>>().cloned().unwrap();

        // convert to result and return
        std::future::ready(value.ok_or(error::Error {
            status_code: 200,
            message: "".into(),
        }))
    }
}

/// 0 - any user
/// 1 - at least moderator
/// 2 - at least admin
/// 3 - only owner
pub fn role_to_number(role: &Role) -> i32 {
    match role {
        Role::Owner => 3,
        Role::Admin => 2,
        Role::Moderator => 1,
        Role::User => 0,
    }
}

/// Checks if reqUser has a role of at least requirement
pub fn require_role(req_user: &Option<ReqUser>, requirement: Role) -> bool {
    match req_user {
        Some(user) => role_to_number(&user.role) >= role_to_number(&requirement),
        None => true,
    }
}

/// Checks if reqUser has a role higher than requirement, impossible if requirement is Owner
pub fn higher_role(req_user: &Option<ReqUser>, requirement: Role) -> bool {
    match req_user {
        Some(user) => role_to_number(&user.role) > role_to_number(&requirement),
        None => true,
    }
}

#[cfg(feature = "axum")]
pub struct ExtractReqUser(pub Option<ReqUser>);

#[cfg(feature = "axum")]
#[axum::async_trait]
impl<B> axum::extract::FromRequest<B> for ExtractReqUser
where
    B: Send,
{
    type Rejection = (axum::http::StatusCode, &'static str);

    async fn from_request(
        req: &mut axum::extract::RequestParts<B>,
    ) -> Result<Self, Self::Rejection> {
        let header_value: Option<String> = req
            .headers()
            .get("user")
            .map(|v| v.to_str().unwrap().to_string());

        if let Some(header_value) = header_value {
            Ok(ExtractReqUser(extract(header_value)))
        } else {
            Ok(ExtractReqUser(None))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
