use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub sender: String,
    pub receiver: String,
    pub message: String
}

impl Message {
    pub fn new(sender: String, receiver: String, message: String) -> Message {
        Message { id: None, sender, receiver, message }
    }
}
