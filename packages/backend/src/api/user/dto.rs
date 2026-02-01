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
    pub email: String,
    pub sessions: Vec<sessions::Model>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseTexturesDTO {
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}
