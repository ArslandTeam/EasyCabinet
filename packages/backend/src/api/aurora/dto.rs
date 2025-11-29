use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestJoinDto {
    pub access_token: String,
    pub user_uuid: String,
    pub server_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestHasJoinedDto {
    pub username: String,
    pub server_id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestProfileDTO {
    pub user_uuid: String,
}

#[derive(Deserialize)]
pub struct RequestProfilesDto {
    pub usernames: Vec<String>,
}
