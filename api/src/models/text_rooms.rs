use super::ResponseError;
use super::{model_from_cursor, Model};

use bson::oid::ObjectId;
use futures::Stream;
use mongodb::{
    bson::{self, doc, DateTime},
    options::FindOptions,
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TextRoom {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub channel_id: ObjectId,
    pub name: String,
}

impl Model for TextRoom {
    fn collection_name() -> &'static str {
        "text_rooms"
    }
}

#[derive(Deserialize, Serialize)]
pub struct TextMessage {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub user_id: ObjectId,
    pub room_id: ObjectId,
    pub timestamp: DateTime,
    pub body: String,
}

impl Model for TextMessage {
    fn collection_name() -> &'static str {
        "text_messages"
    }
}

impl TextRoom {
    pub async fn get_messages(
        &self,
        db: &Database,
    ) -> Result<impl Stream<Item = Result<TextMessage, ResponseError>>, ResponseError> {
        let room_id = self
            .id
            .as_ref()
            .ok_or_else(|| ResponseError::error(None, "room_id is null"))?;

        let options = FindOptions::builder()
            .sort(doc! { "timestamp": -1 })
            .build();

        let messages_cursor = TextMessage::collection(&db)
            .find(doc! { "room_id": room_id }, options)
            .await?;

        return Ok(model_from_cursor(messages_cursor));
    }

    pub async fn send_message(
        &self,
        db: &Database,
        user_id: &ObjectId,
        body: String,
    ) -> Result<TextMessage, ResponseError> {
        let room_id = self
            .id
            .as_ref()
            .ok_or_else(|| ResponseError::error(None, "room_id is null"))?;

        let mut message = TextMessage {
            id: None,
            user_id: user_id.to_owned(),
            room_id: room_id.to_owned(),
            body: body,
            timestamp: DateTime(chrono::Utc::now()),
        };

        let res = TextMessage::collection(db)
            .insert_one(message.to_doc()?, None)
            .await?;

        message.id = res.inserted_id.as_object_id().cloned();

        return Ok(message);
    }
}
