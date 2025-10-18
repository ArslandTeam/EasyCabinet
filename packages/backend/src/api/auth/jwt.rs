use crate::BackendError;
use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
// В функцию validate нужно будет сделать проверку на валидность токена через assert_eq!
// async fn validate(payload: JwtPayload) {}

#[derive(Deserialize, Serialize)]
pub struct JwtPayload {
    pub uuid: String,
    pub login: String,
    pub iat: u64,
    pub exp: u64,
}

pub fn set_access_token(jar: SignedCookieJar, access_token: String) -> SignedCookieJar {
    let cookie = Cookie::build(("accessToken", access_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(std::env::var("COOKIE_SECURE").unwrap_or_default() == "true")
        .max_age(time::Duration::seconds(900))
        .build();

    jar.add(cookie)
}

// INFO тут мы извлекаем access token из куки
pub fn extract_jwt_token(jar: &SignedCookieJar) -> Result<JwtPayload, BackendError> {
    let token = jar
        .get("accessToken")
        .ok_or(BackendError::BadRequest("No access token".to_string()))?
        .value()
        .to_string();

    let token = decode::<JwtPayload>(
        &token,
        &DecodingKey::from_secret(
            std::env::var("JWT_SECRET")
                .expect("JWT_SECRET key not set in .env")
                .as_bytes(),
        ),
        &Validation::default(),
    )
    .map_err(|_| BackendError::BadRequest("Invalid token".to_string()))?;

    Ok(token.claims)
}
