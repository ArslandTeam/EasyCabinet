/// # Panics
pub fn init() {
    if !std::path::Path::new(".env").exists() {
        std::fs::write(
            ".env",
r#"HOST="0.0.0.0"
PORT="4000"
BACKEND_URL="http://backend.example.com"
FRONTEND_URL="http://example.com"

JWT_SECRET="secret"
JWT_EXPIRES_IN=1800

COOKIE_SECURE=true
COOKIES_SECRET="secret"
COOKIE_EXPIRES_IN=2592000

CACHE="local"
REDIS_URL="redis://localhost:6379"

DATABASE_URL="mysql://username:password@host:port/database"

EMAIL_FROM="no-replay@example.ru"
SMTP="smtps://username:password@host:port""#,
        )
        .expect("Error generate file");

        println!("Check .env config");
        std::process::exit(1)
    }
}
