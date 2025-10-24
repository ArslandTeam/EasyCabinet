use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDTO {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterDTO {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordDTO {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ChangePasswordDTO {
    pub reset_token: String,
    pub password: String,
}
