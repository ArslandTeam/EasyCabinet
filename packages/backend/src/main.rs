use crate::api::{
    aurora::controller::AuroraController, auth::controller::AuthController,
    cache_manager::CacheManager, storage_manager::StorageService, user::controller::UserControler,
};
use axum::http::{Method, StatusCode, header};
use axum_extra::extract::cookie::Key;
use std::{ops::Deref, sync::Arc};
mod api;
pub mod generate_config;
use crate::generate_config::CONFIG;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    generate_config::init();
    //"RUST_LOG=debug" or "RUST_LOG=info"
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().or_else(|_| {
                tracing_subscriber::EnvFilter::try_new("axum_tracing_example=error,tower_http=warn")
            })?,
        )
        .init();

    use migration::MigratorTrait;
    let db = sea_orm::Database::connect(&CONFIG.database_url).await?;
    migration::Migrator::up(&db, None).await?;

    let cache = CacheManager::cache_init().await;
    let storage = StorageService::storage_init().await;
    let key = Key::from(CONFIG.cookies_secret.as_bytes());

    let state = AppState(Arc::new(InnerState {
        db,
        cache,
        storage,
        key,
    }));

    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", CONFIG.host, CONFIG.port)).await?;

    axum::serve(listener, init_router(state)).await?;

    Ok(())
}

fn init_router(state: AppState) -> axum::Router {
    use axum::routing::{get, post, put};

    let private_routes = axum::Router::new()
        .route("/auth/logout_all", post(AuthController::logout_all))
        .route("/users", get(UserControler::get_profile))
        .route("/users", put(UserControler::update_profile))
        .route(
            "/users/change-password",
            put(UserControler::change_password),
        )
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            api::auth::jwt::auth_middleware,
        ));

    let routes = axum::Router::new()
        .route(
            "/",
            get(|| async { axum::Json(serde_json::json!({"status": "ok"})) }),
        )
        .route("/auth/login", post(AuthController::authentication))
        .route("/auth/register", post(AuthController::register))
        .route("/auth/verify-email", post(AuthController::verify_email))
        .route("/auth/refresh", post(AuthController::refresh))
        .route("/auth/logout", post(AuthController::logout))
        .route("/auth/reset-password", post(AuthController::reset_password))
        .route(
            "/auth/change-password",
            post(AuthController::change_password),
        )
        .route("/aurora/auth", post(AuroraController::auth))
        .route("/aurora/join", post(AuroraController::join))
        .route("/aurora/hasJoined", post(AuroraController::has_joined))
        .route("/aurora/profile", post(AuroraController::profile))
        .route("/aurora/profiles", post(AuroraController::profiles));

    axum::Router::new()
        .merge(routes)
        .merge(private_routes)
        .nest_service(
            "/uploads",
            tower_http::services::ServeDir::new(std::path::Path::new("uploads")),
        )
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .layer(
            tower_http::cors::CorsLayer::new()
                .allow_origin(
                    CONFIG
                        .frontend_url
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                )
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
struct AppState(Arc<InnerState>);

impl Deref for AppState {
    type Target = InnerState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct InnerState {
    db: sea_orm::DatabaseConnection,
    cache: CacheManager,
    storage: StorageService,
    key: Key,
}

/// Эта реализация сообщает [`SignedCookieJar`], как получить доступ к ключу из нашего состояния
impl axum::extract::FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.0.key.clone()
    }
}

/// Реализация, выполняющая автоматическую валидацию JSON запроса.
///
/// Используется в контроллерах для проверки входных данных и возвращает
/// [`BackendError::BadRequest`] при ошибке валидации или десериализации.
///
/// - Тип `T` должен реализовывать [`serde::de::DeserializeOwned`] и [`validator::Validate`].
/// - Контекст `S` должен быть безопасен для асинхронного использования (`Send + Sync`).
///
/// Пример взять реализациями из:
/// - <https://github.com/tokio-rs/axum/blob/main/examples/validator/src/main.rs>
/// - <https://github.com/truehazker/axum-validated-extractors/blob/develop/src/lib.rs>
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

impl<T, S> axum::extract::FromRequest<S> for ValidatedJson<T>
where
    T: serde::de::DeserializeOwned + validator::Validate,
    S: Send + Sync,
    axum::extract::Json<T>:
        axum::extract::FromRequest<S, Rejection = axum::extract::rejection::JsonRejection>,
{
    type Rejection = BackendError;

    async fn from_request(
        req: axum::http::Request<axum::body::Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::Json(value) = axum::extract::Json::<T>::from_request(req, state)
            .await
            .map_err(|e| BackendError::BadRequest(e.to_string()))?;

        value
            .validate()
            .map_err(|e| BackendError::BadRequest(e.to_string()))?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Debug)]
pub enum BackendError {
    BadRequest(String),
    BadRequestAurora(String),
    Unauthorized(String),
    InternalError,
}

// TODO возможно надо будет переписать
impl axum::response::IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        match self {
            BackendError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({"message": msg})),
            )
                .into_response(),
            BackendError::BadRequestAurora(msg) => (
                StatusCode::BAD_REQUEST,
                axum::Json(serde_json::json!({"success": false, "error": msg})),
            )
                .into_response(),
            BackendError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({"message": msg})),
            )
                .into_response(),
            BackendError::InternalError => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}
