//! # Panics
pub struct Config {
    pub host: String,
    pub port: String,
    pub frontend_url: String,
    pub backend_url: String,
    pub jwt_secret: String,
    pub jwt_expresion_in: u64,
    pub cookie_secure: bool,
    pub cookie_secret: String,
    pub cookie_domain: String,
    pub cookie_expresion_in: u64,
    pub cache_type: String,
    pub redis_url: String,
    pub storage_textures_type: String,
    pub aws_region: String,
    pub aws_endepoint_url: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub bucket_name: String,
    pub aws_public_url: String,
    pub db_url: String,
    pub email_from: String,
    pub smpt: String,
}

pub fn get_env(key: &str) -> String {
    dotenvy::dotenv().ok();
    std::env::var(key).unwrap_or_else(|_| {
        panic!("Not set key in env: {key}");
    })
}

#[allow(clippy::expect_used)]
pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| Config {
    host: get_env("HOST"),
    port: get_env("PORT"),
    frontend_url: get_env("FRONTEND_URL"),
    backend_url: get_env("BACKEND_URL"),
    jwt_secret: get_env("JWT_SECRET"),
    jwt_expresion_in: get_env("JWT_EXPIRES_IN")
        .parse::<u64>()
        .expect("JWT_EXPIRES_IN error parcing (64 bit)"),
    cookie_secure: get_env("COOKIE_SECURE").eq_ignore_ascii_case("true"),
    cookie_secret: get_env("COOKIES_SECRET"),
    cookie_domain: get_env("COOKIE_DOMAIN"),
    cookie_expresion_in: get_env("COOKIE_EXPIRES_IN")
        .parse::<u64>()
        .expect("COOKIE_EXPIRES_IN error parcing (64 bit)"),
    cache_type: get_env("CACHE"),
    redis_url: get_env("REDIS_URL"),
    storage_textures_type: get_env("STORAGE_TEXTURES_TYPE"),
    aws_region: get_env("AWS_REGION"),
    aws_endepoint_url: get_env("AWS_ENDPOINT_URL"),
    aws_access_key_id: get_env("AWS_ACCESS_KEY_ID"),
    aws_secret_access_key: get_env("AWS_SECRET_ACCESS_KEY"),
    bucket_name: get_env("BUCKET_NAME"),
    aws_public_url: get_env("AWS_PUBLIC_URL"),
    db_url: get_env("DATABASE_URL"),
    email_from: get_env("EMAIL_FROM"),
    smpt: get_env("SMTP"),
});

pub fn init() {
    if !std::path::Path::new(".env").exists() {
        std::fs::write(".env", include_str!(".env"))
            .unwrap_or_else(|_| panic!("Error generate file (check permission)"));

        println!("Check .env config");
        std::process::exit(1)
    }
}
