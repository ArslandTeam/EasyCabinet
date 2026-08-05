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
    pub sessions: Vec<SessionDTO>,
}

#[derive(Deserialize, Serialize)]
pub struct SessionDTO {
    pub id: String,
    pub current: bool,
    pub user_agent: String,
}

#[derive(Deserialize, Serialize)]
pub struct ResponseTexturesDTO {
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct RequestChangePassword {
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RequestChangeEmail {
    #[validate(email)]
    pub email: String,
    pub code: u32,
}
