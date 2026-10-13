#[allow(dead_code)]
#[derive(serde::Deserialize)]
pub struct CallbackParams {
    pub code: String,
    pub state: Option<String>,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize, Debug)]
pub struct DiscordUserResponse {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub verified: Option<bool>,
}
