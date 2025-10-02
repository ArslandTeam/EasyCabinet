use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDTO {
    pub login: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterDTO {
    login: String,
    email: String,
    password: String,
}
