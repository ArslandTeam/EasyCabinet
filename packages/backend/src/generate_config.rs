//! # Panics
use serde::Deserialize;
#[derive(Deserialize)]
pub struct Config {
    pub host: String,
    pub port: String,
    pub frontend_url: String,
    pub backend_url: String,
    #[serde(deserialize_with = "deserialize_jwt_key")]
    pub jwt_secret: jwt_simple::prelude::HS512Key,
    pub jwt_expires_in: u64,
    pub cookie_secure: bool,
    pub cookies_secret: String,
    pub cookie_domain: String,
    pub cookie_expires_in: u64,
    pub redis_url: String,
    pub storage_textures_type: String,
    pub aws_region: String,
    pub aws_endpoint_url: String,
    pub aws_access_key_id: String,
    pub aws_secret_access_key: String,
    pub bucket_name: String,
    pub aws_public_url: String,
    pub database_url: String,
    pub max_connection: u32,
    pub connect_timeout: u64,
    #[serde(deserialize_with = "deserialize_mailbox")]
    pub email_from: lettre::message::Mailbox,
    pub smtp: String,
}

pub static CONFIG: std::sync::LazyLock<Config> = std::sync::LazyLock::new(|| {
    dotenvy::dotenv().ok();

    envy::from_env::<Config>().unwrap_or_else(|e| {
        panic!("Missing or invalid environment variables: {e}");
    })
});

pub fn init() {
    if !std::path::Path::new(".env").exists() {
        std::fs::write(".env", include_str!(".env"))
            .unwrap_or_else(|e| panic!("Error generate file: {e}"));

        println!("Check .env config");
        std::process::exit(1)
    }
}

fn deserialize_jwt_key<'de, D>(deserializer: D) -> Result<jwt_simple::prelude::HS512Key, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    Ok(jwt_simple::prelude::HS512Key::from_bytes(s.as_bytes()))
}

fn deserialize_mailbox<'de, D>(deserializer: D) -> Result<lettre::message::Mailbox, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = <&str>::deserialize(deserializer)?;
    s.parse().map_err(serde::de::Error::custom)
}
