use crate::{BackendError, generate_config::CONFIG};
use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JwtPayload {
    pub uuid: String,
    pub login: String,
    pub session_id: i32,
    pub iat: u64,
    pub exp: u64,
}

pub async fn set_access_token(jar: SignedCookieJar, access_token: String) -> SignedCookieJar {
    let cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .domain(&CONFIG.cookie_domain)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(
            CONFIG.jwt_expresion_in.try_into().unwrap(),
        ))
        .secure(CONFIG.cookie_secure)
        .build();

    jar.add(cookie)
}

/// Передаётся ссылка на куки из [`SignedCookieJar`], извлекается и преобразуется в строку. В последствии декодидируется и проверяется валидность
pub async fn extract_jwt_token(jar: &SignedCookieJar) -> Result<JwtPayload, BackendError> {
    let token = jar
        .get("access_token")
        .ok_or(BackendError::BadRequest("No access token".into()))?
        .value()
        .to_string();

    let token = decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| BackendError::BadRequest("Invalid token".into()))?;

    Ok(token.claims)
}
