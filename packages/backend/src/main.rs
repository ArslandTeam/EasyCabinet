use std::env;

use axum::{Json, Router, routing::get};
use serde_json::json;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let app = Router::new().route("/", get(Json(json!({"status": "ok"}))));
    let host = env::var("HOST").expect("HOST key not set in .env");
    let port = env::var("PORT").expect("PORT key not set in .env");
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
