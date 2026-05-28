use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize)]
pub struct RequestProfileDTO {
    pub is_alex: bool,
    pub del_skin: bool,
    pub del_cape: bool,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseProfileDTO {
    pub login: String,
    pub textures: ResponseTexturesDTO,
    pub sessions: Vec<String>,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseTexturesDTO {
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct RequestChangePassword {
    pub password: String,
}
