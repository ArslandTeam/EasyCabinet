use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestJoinDto {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "userUUID")]
    pub user_uuid: String,
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
