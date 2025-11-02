use crate::{
    AppState, BackendError,
    api::auth::{dto, jwt, service},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra::extract::{SignedCookieJar, cookie::Cookie};

pub async fn authentication(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    Json(payload): Json<dto::RequestLoginDTO>,
) -> Result<impl IntoResponse, BackendError> {
    let (access_token, refresh_token) =
        service::authentication(&state.conn, &state.cache, payload.login, payload.password).await?;
    let jar = service::set_refresh_token_cookie(jar, refresh_token);
    let jar = jwt::set_access_token(jar, access_token.clone());
    Ok((StatusCode::OK, jar))
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<dto::RequestRegisterDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::register(&state.conn, &state.cache, payload).await?;
    Ok(StatusCode::CREATED)
}

pub async fn verify_email(
    State(state): State<AppState>,
    Json(payload): Json<dto::RequestVerifyEmailDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::verify_email(&state.conn, &state.cache, payload.email).await?;
    Ok(StatusCode::OK)
}

pub async fn refresh(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    let old_refresh_token = jar
        .get("refresh_token")
        .ok_or(BackendError::BadRequest("No refresh token".to_string()))?
        .value()
        .to_string();

    let (access_token, refresh_token) = service::refresh(&state.cache, old_refresh_token).await?;

    let jar = service::set_refresh_token_cookie(jar, refresh_token);
    let jar = jwt::set_access_token(jar, access_token.clone());
    Ok((StatusCode::OK, jar))
}

// TODO может быть придётся переписать
pub async fn logout(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    if let Some(refresh_token) = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
    {
        service::logout(&state.cache, refresh_token).await;
    }

    let jar = jar
        .remove(Cookie::from("refresh_token"))
        .remove(Cookie::build("access_token").path("/").build());

    Ok((StatusCode::OK, jar))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(payload): Json<dto::RequestResetPasswordDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::reset_password(&state.conn, &state.cache, payload.email).await?;
    Ok(StatusCode::OK)
}

pub async fn change_password(
    State(state): State<AppState>,
    Json(payload): Json<dto::RequestChangePasswordDTO>,
) -> Result<impl IntoResponse, BackendError> {
    service::change_password(
        &state.conn,
        &state.cache,
        payload.reset_token,
        payload.password,
    )
    .await?;
    Ok(StatusCode::OK)
}
