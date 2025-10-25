use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ReqwestProfileDTO {
    pub is_alex: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseProfileDTO {
    #[serde(rename = "isAlex")]
    pub is_alex: Option<bool>,
    #[serde(rename = "skinUrl")]
    pub skin_url: Option<String>,
    #[serde(rename = "capeUrl")]
    pub cape_url: Option<String>,
}
