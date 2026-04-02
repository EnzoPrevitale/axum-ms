use std::sync::Arc;

use axum::{Json, extract::State};
use mongodb::{Cursor, Database, bson::doc};
use serde_json::{Value, json};
use futures::stream::TryStreamExt;

use crate::models::message::Message;

pub async fn get_message(State(db): State<Arc<Database>>) -> Json<Vec<Message>> {
    let collection = db.collection::<Message>("messages");

    let messages: Vec<Message> = collection
    .find(doc! {}).await.unwrap()
    .try_collect().await.unwrap();

    Json(messages)
}
