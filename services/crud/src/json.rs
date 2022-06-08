use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DeleteBody {
    pub affected: i32,
}

impl DeleteBody {
    pub fn new(affected: i32) -> Self {
        DeleteBody { affected }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorBody<'a> {
    message: &'a str,
}

impl<'a> ErrorBody<'a> {
    pub fn new(message: &'a str) -> Self {
        ErrorBody { message }
    }

    pub fn server_err(message: Option<&'a str>) -> Self {
        if let Some(message) = message {
            ErrorBody { message }
        } else {
            ErrorBody {
                message: "Internal server error.",
            }
        }
    }

    pub fn unauthorized(message: Option<&'a str>) -> Self {
        if let Some(message) = message {
            ErrorBody { message }
        } else {
            ErrorBody {
                message: "You are not authorized.",
            }
        }
    }
}
