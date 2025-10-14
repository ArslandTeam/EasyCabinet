use crate::BackendError;
use axum_extra::extract::cookie::SignedCookieJar;
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
