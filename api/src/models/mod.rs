use std::fmt;

use actix_web::http::StatusCode;
use serde_json::json;

pub mod access_tokens;
pub mod users;

#[derive(Debug)]
pub struct ResponseError {
    message: String,
    status_code: StatusCode,
}

impl ResponseError {
    pub fn error(err: Option<&dyn std::error::Error>, message: &str) -> ResponseError {
        if let Some(err) = err {
            println!("ERROR: Response error: {}: {}", message, err.to_string(),);
        } else {
            println!("ERROR: Response error: {}", message);
        }

        ResponseError {
            message: message.to_string(),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn new(message: &str, status_code: StatusCode) -> ResponseError {
        ResponseError {
            message: message.to_string(),
            status_code: status_code,
        }
    }
}

impl fmt::Display for ResponseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.message)
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status_code
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        actix_web::HttpResponse::build(self.status_code).json(json!({
          "type": "error",
          "message": self.message
        }))
    }
}

impl<E: std::error::Error> From<E> for ResponseError {
    fn from(err: E) -> Self {
        Self {
            message: format!("error: {}", &err),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
