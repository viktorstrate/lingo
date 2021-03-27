use super::{text_messages::TextMessage, ResponseError};
use super::{users::User, Model};

use bson::oid::ObjectId;
use futures::StreamExt;
use mongodb::{
    bson::{self, doc},
    options::{FindOptions, FindOptionsBuilder},
    Cursor, Database,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TextRoom {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
}

impl Model for TextRoom {
    fn collection_name() -> &'static str {
        "text_rooms"
    }
}

// impl TextRoom {
//     pub async fn get_messages(
//         &self,
//         db: &Database,
//     ) -> Result<impl Stream<Item = Result<TextMessage, ResponseError>>, ResponseError> {
//         let room_id = self
//             .id
//             .as_ref()
//             .ok_or_else(|| ResponseError::error(None, "room_id is null"))?;

//         let options = FindOptions::builder()
//             .sort(doc! { "timestamp": -1 })
//             .build();

//         let messages_cursor = TextMessage::collection(&db)
//             .find(doc! { "room_id": room_id }, options)
//             .await?;

//         let iter = messages_cursor.map(f)

//         return Ok(messages);
//     }
// }
