use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use bcrypt::BcryptError;
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use serde::Deserialize;
use serde_json::json;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct ApiError {
    pub status_code: u16,
    pub message: String,
}

impl ApiError {
    pub fn new(status_code: u16, message: String) -> ApiError {
        error!("{}", message);
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
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl From<DieselError> for ApiError {
    fn from(error: DieselError) -> ApiError {
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

impl From<BcryptError> for ApiError {
    fn from(_err: BcryptError) -> Self {
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
