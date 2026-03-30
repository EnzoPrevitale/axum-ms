use axum::Json;
use serde_json::{Value, json};

pub async fn get_message() -> Json<Value> {
    Json(json!({"hello": "world"})) 
}
