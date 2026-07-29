use crate::{
    AppState, BackendError, ValidatedJson,
    api::auth::{
        dto,
        jwt::{self, JwtPayload},
        service::AuthService,
    },
};
use axum::{
    Extension,
    extract::State,
    http::{HeaderMap, StatusCode, header::USER_AGENT},
    response::IntoResponse,
};
use axum_extra::extract::{SignedCookieJar, cookie::Cookie};

pub struct AuthController;

impl AuthController {
    pub async fn authentication(
        State(state): State<AppState>,
        headers: HeaderMap,
        jar: SignedCookieJar,
        ValidatedJson(payload): ValidatedJson<dto::RequestLoginDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        let user_agent = headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("Unknown");

        let (access_token, refresh_token) = AuthService::authentication(
            &state.db,
            &state.cache,
            payload.login,
            &payload.password,
            user_agent,
        )
        .await?;
        let jar = AuthService::set_refresh_token_cookie(jar, refresh_token).await;
        let jar = jwt::set_access_token(jar, access_token).await;
        Ok((StatusCode::OK, jar))
    }

    pub async fn register(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<dto::RequestRegisterDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuthService::register(&state.db, &state.cache, payload).await?;
        Ok(StatusCode::CREATED)
    }

    pub async fn verify_email(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<dto::RequestVerifyEmailDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuthService::verify_email(&state.db, &state.cache, &payload.email).await?;
        Ok(StatusCode::OK)
    }

    pub async fn refresh(
        State(state): State<AppState>,
        headers: HeaderMap,
        jar: SignedCookieJar,
    ) -> Result<impl IntoResponse, BackendError> {
        let old_refresh_token = jar
            .get("refresh_token")
            .ok_or(BackendError::Unauthorized("No refresh token".into()))?
            .value()
            .to_owned();

        let user_agent = headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("Unknown");

        let (access_token, refresh_token) =
            AuthService::refresh(&state.cache, old_refresh_token, user_agent).await?;

        let jar = AuthService::set_refresh_token_cookie(jar, refresh_token).await;
        let jar = jwt::set_access_token(jar, access_token).await;
        Ok((StatusCode::OK, jar))
    }

    pub async fn logout(
        State(state): State<AppState>,
        jar: SignedCookieJar,
    ) -> Result<impl IntoResponse, BackendError> {
        let refresh_token = jar
            .get("refresh_token")
            .map(|cookie| cookie.value().to_owned());

        AuthService::logout(&state.cache, refresh_token).await?;

        let jar = jar
            .remove(Cookie::from("refresh_token"))
            .remove(Cookie::build("access_token").path("/").build());

        Ok((StatusCode::OK, jar))
    }

    pub async fn logout_all(
        State(state): State<AppState>,
        jar: SignedCookieJar,
        Extension(payload): Extension<JwtPayload>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuthService::logout_all(&state.cache, payload.uuid).await?;

        let jar = jar
            .remove(Cookie::from("refresh_token"))
            .remove(Cookie::build(("access_token", "")).path("/").build());

        Ok((StatusCode::OK, jar))
    }

    pub async fn reset_password(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<dto::RequestResetPasswordDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuthService::reset_password(&state.db, &state.cache, &payload.email).await?;
        Ok(StatusCode::OK)
    }

    pub async fn change_password(
        State(state): State<AppState>,
        ValidatedJson(payload): ValidatedJson<dto::RequestChangePasswordDTO>,
    ) -> Result<impl IntoResponse, BackendError> {
        AuthService::change_password(
            &state.db,
            &state.cache,
            payload.reset_token,
            payload.password,
        )
        .await?;
        Ok(StatusCode::OK)
    }
}
