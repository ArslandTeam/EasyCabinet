use serde::{Deserialize, Serialize};

async fn validate(payload: JwtPayload) {}

#[derive(Deserialize, Serialize)]
pub struct JwtPayload {
    pub uuid: String,
    pub login: String,
}
