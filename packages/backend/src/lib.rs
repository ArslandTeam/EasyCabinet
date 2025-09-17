use axum::{
    Json, Router,
    http::StatusCode,
    routing::{get, post},
};
use serde_json::json;
use std::env;

mod auth;

#[tokio::main]
pub async fn start_backend() {
    dotenvy::dotenv().ok();
    let app = Router::new()
        .route("/", get(Json(json!({"status": "ok"}))))
        .route("/auth/login", post(auth::controller::login));
    let host = env::var("HOST").expect("HOST key not set in .env");
    let port = env::var("PORT").expect("PORT key not set in .env");
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
pub enum BackendError {
    BadReqwest(String),
    InternalError,
}

impl axum::response::IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        match self {
            BackendError::BadReqwest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response()
            }
            BackendError::InternalError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
