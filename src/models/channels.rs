use super::ResponseError;
use super::{model_from_cursor, users::User, Model};

use bson::oid::ObjectId;
use futures::{stream::Map, Stream, StreamExt};
use mongodb::{
    bson::{self, doc},
    options::{FindOptions, FindOptionsBuilder},
    Cursor, Database,
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
    /// Returns a cursor of User
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
        name: &str,
        user_id: &ObjectId,
    ) -> Result<Self, ResponseError> {
        let channel = Channel {
            id: None,
            name: name.to_owned(),
            members: vec![user_id.clone()],
        };

        Channel::collection(&db)
            .insert_one(channel.to_doc()?, None)
            .await?;

        return Ok(channel);
    }
}
