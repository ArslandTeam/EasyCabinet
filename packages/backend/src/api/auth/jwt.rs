use crate::BackendError;
use axum_extra::extract::cookie::{Cookie, SameSite, SignedCookieJar};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JwtPayload {
    pub uuid: String,
    pub login: String,
    pub iat: u64,
    pub exp: u64,
}

pub fn set_access_token(jar: SignedCookieJar, access_token: String) -> SignedCookieJar {
    let cookie = Cookie::build(("access_token", access_token))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .secure(std::env::var("COOKIE_SECURE").unwrap_or_default() == "true")
        .max_age(time::Duration::seconds(
            std::env::var("JWT_EXPIRES_IN")
                .unwrap()
                .parse::<i64>()
                .expect("JWT_EXPIRES_IN key not set in .env"),
        ))
        .build();

    jar.add(cookie)
}

/// Передаётся ссылка на куки из [`SignedCookieJar`], извлекается и преобразуется в строку. В последствии декодидируется и проверяется валидность
pub fn extract_jwt_token(jar: &SignedCookieJar) -> Result<JwtPayload, BackendError> {
    let token = jar
        .get("access_token")
        .ok_or(BackendError::BadRequest("No access token".into()))?
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
    .map_err(|_| BackendError::BadRequest("Invalid token".into()))?;

    Ok(token.claims)
}
