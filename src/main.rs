use axum::{
    Router, routing::get
};
use std::sync::Arc;
use mongodb::Database;

mod handlers;
mod models;
mod config;
use handlers::message_handlers;

use crate::config::database;

#[tokio::main]
async fn main() {
    let db: Database = database::setup().await;

    let app = Router::new()
    .route("/", get(message_handlers::get_message))
    .with_state(Arc::new(db));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
