use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RequestJoinDto {
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "serverID")]
    pub server_id: String,
}

#[derive(Deserialize)]
pub struct RequestHasJoinedDto {
    pub username: String,
    #[serde(rename = "serverID")]
    pub server_id: String,
}

#[derive(Deserialize)]
pub struct RequestProfileDTO {
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
}

#[derive(Deserialize)]
pub struct RequestProfilesDto {
    pub usernames: Vec<String>,
}

#[derive(Serialize)]
pub struct AuroraResponse<T> {
    pub success: bool,
    pub result: T,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseDto {
    pub username: String,
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
    pub access_token: String,
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HasJoinResponseDto {
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponseDto {
    pub username: String,
    pub is_alex: Option<bool>,
    pub skin_url: Option<String>,
    pub cape_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ProfilesResponseDto {
    pub id: String,
    pub name: String,
}
