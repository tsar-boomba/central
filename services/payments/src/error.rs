use axum::response::IntoResponse;
use hyper::StatusCode;
use stripe::{ErrorType, ParseIdError, StripeError};

pub struct ApiError {
    status: StatusCode,
    message: String,
}

impl ApiError {
    pub fn new(status: StatusCode, message: impl Into<String>) -> Self {
        ApiError {
            status,
            message: message.into(),
        }
    }
}

impl From<StripeError> for ApiError {
    fn from(err: StripeError) -> Self {
        tracing::error!("[Stripe] An error ocurred: {:?}", err);
        match err {
            StripeError::ClientError(message) => ApiError::new(StatusCode::BAD_REQUEST, message),
            StripeError::Stripe(request_err) => {
                tracing::error!("[Stripe] Error ocurred: {:?}", request_err);
                let status = StatusCode::from_u16(request_err.http_status).unwrap();
                let message: String = match request_err.error_type {
                    ErrorType::Card => request_err
                        .message
                        .unwrap_or("A card error ocurred.".into()),
                    ErrorType::InvalidRequest => "Bad details provided to stripe.".into(),
                    _ => "A Stripe error ocurred.".into(),
                };

                ApiError::new(status, message)
            }
            StripeError::QueryStringSerialize(qs_err) => {
                tracing::error!("[Stripe] A querystring error ocurred: {:?}", qs_err);
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error ocurred.",
                )
            }
            StripeError::JSONSerialize(json_err) => {
                tracing::error!(
                    "[Stripe] A json serialization error ocurred: {:?}",
                    json_err
                );
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error ocurred.",
                )
            }
            StripeError::UnsupportedVersion => {
                tracing::error!("[Stripe] An unsupported version error ocurred!");
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error ocurred.",
                )
            }
            StripeError::Timeout => {
                tracing::error!("[Stripe] A Stripe timeout ocurred!");
                ApiError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "An internal server error ocurred.",
                )
            }
        }
    }
}

impl From<ParseIdError> for ApiError {
    fn from(_: ParseIdError) -> Self {
        tracing::error!("[Stripe] Invalid id received.");
        ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid id.")
    }
}

impl From<hyper::Error> for ApiError {
    fn from(err: hyper::Error) -> Self {
        tracing::error!("[hyper] An error ocurred: {}", err.message());
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "An error ocurred with internal communication.",
        )
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        let message = err.to_string();
        tracing::error!("[serde_json] An error ocurred: {}", message);
        match err.classify() {
            serde_json::error::Category::Data => ApiError::new(StatusCode::BAD_REQUEST, message),
            _ => ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "An internal server error ocurred.",
            ),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (self.status, format!(r#"{{"message":"{}"}}"#, self.message)).into_response()
    }
}
