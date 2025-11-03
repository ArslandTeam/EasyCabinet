use serde::Deserialize;
use validator::Validate;

// TODO надо сделать нормальный вывод ошибки валидации
static LOGIN_REGEX: std::sync::LazyLock<regex::Regex> =
    std::sync::LazyLock::new(|| regex::Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

#[derive(Deserialize, Validate)]
pub struct RequestLoginDTO {
    #[validate(length(min = 3, max = 16, message = "Логин должен быть от 3 до 16 символов"))]
    #[validate(regex(path = LOGIN_REGEX, message = "Логин может содержать только латинские буквы, цифры и _"))]
    pub login: String,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct RequestRegisterDTO {
    #[validate(length(min = 3, max = 16, message = "Логин должен быть от 3 до 16 символов"))]
    #[validate(regex(path = LOGIN_REGEX, message = "Логин может содержать только латинские буквы, цифры и _"))]
    pub login: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 20))]
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

#[derive(Deserialize, Validate)]
pub struct RequestChangePasswordDTO {
    pub reset_token: String,
    #[validate(length(min = 8, max = 20))]
    pub password: String,
}
