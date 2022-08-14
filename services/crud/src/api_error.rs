use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use bcrypt::BcryptError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use models::ValidationErrors;
use serde::Deserialize;
use serde_json::json;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        ApiError {
            status_code,
            message,
        }
    }

    pub fn server_err() -> ApiError {
        ApiError::new(500, "Internal server error.".into())
    }

    pub fn forbidden() -> ApiError {
        ApiError::new(403, "You do not have access to this resource".into())
    }

    pub fn not_subbed() -> ApiError {
        ApiError::new(403, "You cannot do this while not subscribed.".into())
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
        error!("[Diesel] error: {:?}", error);
        match error {
            DieselError::DatabaseError(kind, _err) => match kind {
                DatabaseErrorKind::UniqueViolation => ApiError::new(
                    409,
                    "One of the provided fields failed unique validation.".into(),
                ),
                DatabaseErrorKind::UnableToSendCommand => {
                    ApiError::new(500, "Unable to send command to database.".into())
                }
                DatabaseErrorKind::ForeignKeyViolation => {
                    ApiError::new(409, "Cannot add to an account that doesn't exist.".into())
                }
                _ => ApiError::new(500, "A database error occurred.".into()),
            },
            DieselError::NotFound => ApiError::new(404, "Record not found".into()),
            _ => ApiError::new(500, "A database error occurred.".into()),
        }
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(err: ValidationErrors) -> Self {
        let mut message = String::from_str("Validation failed for fields: ").unwrap();
        message.push_str(&err.into_errors().into_iter().fold(
            String::new(),
            |mut err_fields, (curr_field, _)| {
                err_fields.reserve(curr_field.len() + 1);
                err_fields.push_str(curr_field);
                err_fields.push_str(",");
                err_fields
            },
        ));
        error!("Validation failed: {}", message);

        ApiError::new(400, message)
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        error!("[Reqwest] error: {:?}", err);
        ApiError::server_err()
    }
}

impl From<BcryptError> for ApiError {
    fn from(err: BcryptError) -> Self {
        error!("[Bcrypt] error: {:?}", err);
        ApiError::new(500, "Error hashing password.".into())
    }
}

impl From<BlockingError> for ApiError {
    fn from(_err: BlockingError) -> Self {
        ApiError::server_err()
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(json!({ "message": self.message.clone() }))
    }
}
