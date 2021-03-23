use mongodb::{Client, Database};
use std::error::Error;

pub async fn get_database() -> Result<Database, Box<dyn Error>> {
    let client_uri = match std::env::var("MONGODB_URI") {
        Ok(val) => val,
        _ => String::from("mongodb://127.0.0.1:27017"),
    };

    let client = Client::with_uri_str(client_uri.as_ref()).await?;
    let db = client.database("lingo");

    return Ok(db);
}
