use serde::Deserialize;
use validator::Validate;

// TODO надо сделать нормальный вывод ошибки валидации
static LOGIN_REGEX: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

#[derive(Deserialize)]
pub struct RequestLoginDTO {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RequestRegisterDTO {
    #[validate(length(min = 3, max = 16))]
    #[validate(regex(path = LOGIN_REGEX))]
    pub login: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub code: u32,
}

#[derive(Deserialize, Validate)]
pub struct RequestVerifyEmailDTO {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize, Validate)]
pub struct RequestResetPasswordDTO {
    #[validate(email)]
    pub email: String,
}

#[derive(Deserialize)]
pub struct RequestChangePasswordDTO {
    pub reset_token: String,
    pub password: String,
}
