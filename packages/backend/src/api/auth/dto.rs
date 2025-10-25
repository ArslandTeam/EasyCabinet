use serde::Deserialize;

#[derive(Deserialize)]
pub struct RequestLoginDTO {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestRegisterDTO {
    pub login: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RequestResetPasswordDTO {
    pub email: String,
}

#[derive(Deserialize)]
pub struct RequestChangePasswordDTO {
    pub reset_token: String,
    pub password: String,
}
