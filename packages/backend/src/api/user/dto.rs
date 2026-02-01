use serde::{Deserialize, Serialize};

use crate::api::database::entities::sessions;

#[derive(Deserialize, Serialize)]
pub struct ReqwestProfileDTO {
    pub is_alex: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseProfileDTO {
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseAccountDTO {
    pub email: String,
    pub sessions: Vec<sessions::Model>,
}
