use axum::{
    Json, Router,
    extract::FromRef,
    http::{HeaderValue, StatusCode},
    routing::{get, post, put},
};
use axum_extra::extract::cookie::Key;
use http::{Method, header};
use migration::{Migrator, MigratorTrait};
use moka::future::Cache;
use sea_orm::{Database, DatabaseConnection};
use serde_json::json;
use std::env;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::EnvFilter;
mod api;

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

    let cache = Cache::builder()
        .max_capacity(1000)
        .time_to_live(std::time::Duration::from_secs(2592000))
        .build();

    let key = Key::from(
        env::var("COOKIES_SECRET")
            .expect("COOKIES_SECRET key not set in .env")
            .as_bytes(),
    );

    let state = AppState { conn, cache, key };

    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}"))
        .await
        .unwrap();
    axum::serve(listener, init_router(state)).await.unwrap();
}

fn init_router(state: AppState) -> Router {
    let frontend = env::var("FRONTEND_URL").expect("FRONTEND_URL key not set in .env");

    Router::new()
        .route("/", get(|| async { Json(json!({"status": "ok"})) }))
        .route("/auth/login", post(api::auth::controller::login))
        .route("/auth/register", post(api::auth::controller::register))
        .route("/auth/refresh", post(api::auth::controller::refresh))
        .route("/auth/logout", post(api::auth::controller::logout))
        .route(
            "/auth/reset-password",
            post(api::auth::controller::reset_password),
        )
        .route(
            "/auth/change-password",
            post(api::auth::controller::change_password),
        )
        .route("/users", get(api::user::controller::get_profile))
        .route("/users", put(api::user::controller::update_profile))
        .nest_service(
            "/uploads",
            tower_http::services::ServeDir::new(std::path::Path::new("uploads")),
        )
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(frontend.parse::<HeaderValue>().unwrap())
                .allow_methods([Method::POST, Method::GET, Method::PUT, Method::OPTIONS])
                .allow_headers([
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                    header::AUTHORIZATION,
                    header::COOKIE,
                ])
                .allow_credentials(true),
        )
        .with_state(state)
}

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection,
    cache: Cache<String, String>,
    key: Key,
}

// INFO эта реализация сообщает `SignedCookieJar`, как получить доступ к ключу из нашего состояния
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.key.clone()
    }
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
                (StatusCode::BAD_REQUEST, Json(json!({"message": msg}))).into_response()
            }
            BackendError::InternalError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
