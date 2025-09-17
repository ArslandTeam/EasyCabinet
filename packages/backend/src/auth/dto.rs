use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginDTO {
    login: String,
    password: String,
}

#[derive(Deserialize)]
pub struct RegisterDTO {
    login: String,
    email: String,
    password: String,
}
