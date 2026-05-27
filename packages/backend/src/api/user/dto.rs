use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReqwestProfileDTO {
    pub is_alex: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseProfileDTO {
    pub login: String,
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
    pub sessions: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseTexturesDTO {
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}
