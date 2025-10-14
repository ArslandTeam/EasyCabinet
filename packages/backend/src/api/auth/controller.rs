use crate::{
    AppState, BackendError,
    api::auth::dto::{LoginDTO, RegisterDTO},
    api::auth::service,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{SignedCookieJar, cookie::Cookie};
use serde_json::json;

pub async fn login(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<LoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    let (access_token, refresh_token) = service::login(&state.conn, &state.cache, payload).await?;
    let jar = service::set_refresh_token_cookie(jar, refresh_token);
    Ok((
        StatusCode::OK,
        jar,
        Json(json!({"accessToken": access_token})),
    ))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::register(&state.conn, payload).await?;
    Ok(StatusCode::CREATED)
}

pub async fn refresh(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    let old_refresh_token = jar
        .get("refreshToken")
        .ok_or(BackendError::BadRequest("No refresh token".to_string()))?
        .value()
        .to_string();

    let (access_token, refresh_token) = service::refresh(&state.cache, old_refresh_token).await?;

    let jar = service::set_refresh_token_cookie(jar, refresh_token);
    Ok((
        StatusCode::OK,
        jar,
        Json(json!({"accessToken": access_token})),
    ))
}

// TODO может быть придётся переписать
pub async fn logout(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    if let Some(refresh_token) = jar.get("refreshToken").map(|c| c.value().to_string()) {
        service::logout(&state.cache, refresh_token).await;
    }

    let jar = jar.remove(Cookie::from("refreshToken"));

    Ok((StatusCode::OK, jar))
}
