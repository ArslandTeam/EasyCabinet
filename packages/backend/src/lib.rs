use axum::{
    Json, Router,
    http::{HeaderValue, StatusCode},
    routing::{get, post},
};
use http::{Method, header};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use serde_json::json;
use std::env;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;
mod auth;
mod entities;
mod user;

#[tokio::main]
pub async fn start_backend() {
    dotenvy::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL key not set in .env");
    let host = env::var("HOST").expect("HOST key not set in .env");
    let port = env::var("PORT").expect("PORT key not set in .env");
    //"RUST_LOG=debug" or "RUST_LOG=info"
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("axum_tracing_example=error,tower_http=warn"))
                .unwrap(),
        )
        .init();

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let state = AppState { conn };

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, init_router(state)).await.unwrap();
}

fn init_router(state: AppState) -> Router {
    let frontend = env::var("FRONTEND_URL").expect("FRONTEND_URL key not set in .env");

    Router::new()
        .route("/", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/auth/login", post(auth::controller::login))
        .route("/auth/register", post(auth::controller::register))
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(frontend.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::GET, Method::PUT, Method::OPTIONS])
                .allow_headers([header::CONTENT_TYPE, header::ACCEPT])
                .allow_credentials(true),
        )
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[derive(Debug)]
pub enum BackendError {
    BadRequest(String),
    InternalError,
}

// TODO возможно надо будет переписать
impl axum::response::IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        match self {
            BackendError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, Json(json!({"error": msg}))).into_response()
            }
            BackendError::InternalError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
