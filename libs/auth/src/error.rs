#[derive(Debug)]
pub struct Error {
	pub status_code: u16,
	pub message: String,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.message.as_str())
    }
}

#[cfg(feature = "actix")]
impl actix_web::ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status_code = match actix_web::http::StatusCode::from_u16(self.status_code) {
            Ok(status_code) => status_code,
            Err(_) => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        };

        actix_web::HttpResponse::build(status_code).json(serde_json::json!({ "message": self.message.clone() }))
    }
}
