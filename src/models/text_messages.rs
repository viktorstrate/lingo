use super::ResponseError;
use super::{users::User, Model};

use bson::oid::ObjectId;
use mongodb::{
    bson::{self, doc, DateTime},
    options::{FindOptions, FindOptionsBuilder},
    Cursor, Database,
};
use serde::{Deserialize, Serialize};

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
