use actix_web::{dev::HttpServiceFactory, get, post, web, HttpResponse};
use futures::stream::StreamExt;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::{
    channels::{self, Channel},
    model_from_cursor,
    users::AccessToken,
    Model, ResponseError,
};

#[derive(Deserialize, Serialize)]
struct MakeChannelRequest {
    name: String,
}

#[post("")]
async fn make_channel(
    data: web::Data<crate::WebState>,
    token: AccessToken,
    req: web::Json<MakeChannelRequest>,
) -> Result<HttpResponse, ResponseError> {
    let user = token.get_user(&data.db).await?;
    let channel = channels::Channel::make_channel(
        &data.db,
        req.name.to_owned(),
        user.id.expect("user to have id"),
    )
    .await?;

    Ok(HttpResponse::Ok().json(json!({ "name": &channel.name })))
}

#[get("")]
async fn list_channels(data: web::Data<crate::WebState>) -> Result<HttpResponse, ResponseError> {
    let cursor = Channel::collection(&data.db).find(doc! {}, None).await?;
    let mut iter = model_from_cursor::<Channel>(cursor);

    let mut channels = vec![];

    while let Some(value) = iter.next().await {
        channels.push(value?);
    }

    return Ok(HttpResponse::Ok().json(json!(channels)));
}

pub fn routes() -> impl HttpServiceFactory {
    web::scope("/channels")
        .service(make_channel)
        .service(list_channels)
}
