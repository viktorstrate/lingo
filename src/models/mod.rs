use std::fmt;

use actix_web::{http::StatusCode, HttpResponse};
use mongodb::bson::Document;
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

pub mod users;

pub trait Model: Sized + DeserializeOwned + Serialize {
    fn collection_name() -> &'static str;

    fn collection(db: &mongodb::Database) -> mongodb::Collection {
        db.collection(Self::collection_name())
    }

    fn from_doc(document: mongodb::bson::Document) -> Result<Self, ResponseError> {
        match mongodb::bson::from_document::<Self>(document) {
            Ok(model) => Ok(model),
            Err(err) => Err(ResponseError::error(
                Some(&err),
                &format!(
                    "failed to convert document to struct ({})",
                    Self::collection_name()
                ),
            )),
        }
    }

    fn to_doc(&self) -> Result<Document, ResponseError> {
        match mongodb::bson::to_document(&self) {
            Ok(model) => Ok(model),
            Err(err) => Err(ResponseError::error(
                Some(&err),
                &format!(
                    "failed to convert struct to document ({})",
                    Self::collection_name()
                ),
            )),
        }
    }
}

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

impl From<mongodb::error::Error> for ResponseError {
    fn from(err: mongodb::error::Error) -> Self {
        ResponseError::error(Some(&err), &format!("database error"))
    }
}

impl actix_web::error::ResponseError for ResponseError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        actix_web::HttpResponse::build(self.status_code).json(json!({
          "type": "error",
          "message": self.message
        }))
    }
}
