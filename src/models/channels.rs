use super::ResponseError;
use super::{model_from_cursor, text_rooms::TextRoom, users::User, Model};

use bson::oid::ObjectId;
use futures::Stream;
use mongodb::{
    bson::{self, doc},
    Database,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Channel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub members: Vec<ObjectId>,
}

impl Model for Channel {
    fn collection_name() -> &'static str {
        "channels"
    }
}

impl Channel {
    pub async fn get_members(
        &self,
        db: &Database,
    ) -> Result<impl Stream<Item = Result<User, ResponseError>>, ResponseError> {
        let users = User::collection(&db)
            .find(doc! { "_id": { "$in": &self.members } }, None)
            .await?;

        return Ok(model_from_cursor(users));
    }

    pub async fn make_channel(
        db: &Database,
        name: String,
        user_id: ObjectId,
    ) -> Result<Self, ResponseError> {
        let channel = Channel {
            id: None,
            name: name,
            members: vec![user_id],
        };

        Channel::collection(&db)
            .insert_one(channel.to_doc()?, None)
            .await?;

        return Ok(channel);
    }

    pub async fn make_text_room(
        &self,
        db: &Database,
        name: String,
    ) -> Result<TextRoom, ResponseError> {
        let channel_id = self
            .id
            .as_ref()
            .ok_or_else(|| ResponseError::error(None, "channel_id is null"))?
            .clone();

        let mut room = TextRoom {
            id: None,
            channel_id: channel_id,
            name: name,
        };

        let res = TextRoom::collection(&db)
            .insert_one(room.to_doc()?, None)
            .await?;

        room.id = res.inserted_id.as_object_id().cloned();

        return Ok(room);
    }
}
