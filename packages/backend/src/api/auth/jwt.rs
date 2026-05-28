use crate::{AppState, BackendError, generate_config::CONFIG};
use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar};
use jwt_simple::prelude::*;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct JwtPayload {
    pub uuid: String,
    pub login: String,
    pub session_id: String,
}

pub async fn set_access_token(jar: SignedCookieJar, access_token: String) -> SignedCookieJar {
    let cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .domain(&CONFIG.cookie_domain)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(
            CONFIG.jwt_expires_in.try_into().unwrap(),
        ))
        .secure(CONFIG.cookie_secure)
        .build();

    jar.add(cookie)
}

pub async fn extract_jwt_token(jar: &SignedCookieJar) -> Result<JwtPayload, BackendError> {
    let cookie = jar
        .get("access_token")
        .ok_or(BackendError::Unauthorized("Not found access token".into()))?;
    let token_data = cookie.value();

    let key = &CONFIG.jwt_secret;

    let token = key
        .verify_token::<JwtPayload>(token_data, None)
        .map_err(|_| BackendError::Unauthorized("Not valid access token".into()))?;

    Ok(token.custom)
}

pub async fn auth_middleware(
    axum::extract::State(state): axum::extract::State<AppState>,
    jar: SignedCookieJar,
    mut req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, BackendError> {
    let payload = extract_jwt_token(&jar).await?;

    state
        .cache
        .get(&format!("session:{}:{}", payload.uuid, payload.session_id))
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::Unauthorized("Not valid session".into()))?;

    req.extensions_mut().insert(payload);

    Ok(next.run(req).await)
}
