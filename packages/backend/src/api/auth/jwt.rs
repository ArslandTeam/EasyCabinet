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
