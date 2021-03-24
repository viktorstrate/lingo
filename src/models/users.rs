use std::{
    future::{ready, Future, Ready},
    pin::Pin,
};

use super::Model;
use super::ResponseError;
use crate::WebState;
use actix_web::{
    dev::ResponseBody,
    http::{header::AUTHORIZATION, StatusCode},
    web::Data,
    FromRequest, HttpMessage,
};
use bson::oid::ObjectId;
use mongodb::bson::{self, doc, DateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub username: String,
    pub password_hash: String,
}

impl Model for User {
    fn collection_name() -> &'static str {
        "users"
    }
}

impl User {
    pub async fn register(
        db: &mongodb::Database,
        username: &str,
        password: &str,
    ) -> Result<User, impl actix_web::ResponseError> {
        let users = db.collection("users");

        let user_exists = match users.find_one(doc! { "username": &username }, None).await {
            Ok(doc) => doc.is_some(),
            Err(_) => return Err(ResponseError::error("get user from database")),
        };

        if user_exists {
            return Err(ResponseError::new(
                "user already exists",
                StatusCode::BAD_REQUEST,
            ));
        }

        let user = User {
            id: None,
            username: username.to_string(),
            password_hash: password.to_string(),
        };

        users.insert_one(user.to_doc()?, None).await?;

        println!("New user created: {}", user.username);

        return Ok(user);
    }
}

#[derive(Deserialize, Serialize)]
pub struct AccessToken {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub expire_at: DateTime,
    pub token: String,
}

impl Model for AccessToken {
    fn collection_name() -> &'static str {
        "access_tokens"
    }
}

impl AccessToken {
    pub async fn generate_token(
        db: &mongodb::Database,
        user_id: ObjectId,
    ) -> Result<AccessToken, ResponseError> {
        let access_token = AccessToken {
            id: None,
            user_id: user_id,
            expire_at: DateTime::from(chrono::Utc::now() + chrono::Duration::days(30)),
            token: Uuid::new_v4().to_string(),
        };

        AccessToken::collection(&db)
            .insert_one(access_token.to_doc()?, None)
            .await?;

        return Ok(access_token);
    }

    pub async fn get_user(db: &mongodb::Database, token: &str) -> Result<User, ResponseError> {
        match AccessToken::collection(&db)
            .find_one(doc! { token: token }, None)
            .await
        {
            Ok(Some(doc)) => User::from_doc(doc),
            _ => Err(ResponseError::error("error")),
        }
    }
}

impl actix_web::FromRequest for AccessToken {
    type Error = ResponseError;
    // type Future = Ready<Result<Self, Self::Error>>;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let header_value = req.headers().get(AUTHORIZATION).map(|val| val.clone());

        let db = match req.app_data::<Data<WebState>>() {
            Some(state) => state.db.clone(),
            _ => {
                return Box::pin(ready(Err(ResponseError::error(
                    "could not get db from WebState",
                ))))
            }
        };

        Box::pin(async move {
            let auth_header = match header_value {
                Some(header) => match header.to_str() {
                    Ok(str) => str.to_owned(),
                    _ => return Err(ResponseError::error("invalid AUTHORIZATION header")),
                },
                None => {
                    return Err(ResponseError::new(
                        "missing access token",
                        StatusCode::UNAUTHORIZED,
                    ))
                }
            };

            if !auth_header.to_lowercase().starts_with("bearer ") {
                return Err(ResponseError::error(
                    "AUTHORIZATION header is invalid bearer",
                ));
            }

            let token_value = match auth_header.split(' ').nth(1) {
                Some(token) => token,
                _ => {
                    return Err(ResponseError::error(
                        "AUTHORIZATION header is invalid bearer",
                    ))
                }
            };

            let token = match AccessToken::collection(&db)
                .find_one(doc! { "token": token_value }, None)
                .await
            {
                Ok(Some(token)) => AccessToken::from_doc(token),
                Ok(None) => Err(ResponseError::new(
                    "invalid token",
                    StatusCode::UNAUTHORIZED,
                )),
                Err(_) => Err(ResponseError::error("failed to convert access token")),
            };

            return token;
        })
    }
}
