use std::str::FromStr;

use actix_web::{dev::HttpServiceFactory, get, http::StatusCode, web, HttpResponse};
use mongodb::bson::{doc, oid::ObjectId};
use serde_json::json;

use crate::models::{
    channels::{self, Channel},
    users::AccessToken,
    Model, ResponseError,
};

#[get("")]
async fn get_channel(
    data: web::Data<crate::WebState>,
    token: AccessToken,
    web::Path((channel_id)): web::Path<(String)>,
) -> Result<HttpResponse, ResponseError> {
    let channel_id = ObjectId::from_str(&channel_id)
        .or_else(|err| Err(ResponseError::error(Some(&err), "convert channel id")))?;

    let channel = Channel::collection(&data.db)
        .find_one(doc! { "_id": channel_id }, None)
        .await?;

    match channel {
        Some(doc) => {
            let channel = Channel::from_doc(doc)?;
            Ok(HttpResponse::Ok().json(json!(channel)))
        }
        None => Err(ResponseError::new(
            "channel not found",
            StatusCode::NOT_FOUND,
        )),
    }
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/channel/{channel_id}").service(get_channel)
}
