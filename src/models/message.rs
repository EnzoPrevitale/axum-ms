use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub sender: String,
    pub receiver: String,
    pub message: String
}
