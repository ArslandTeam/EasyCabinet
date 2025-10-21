use crate::BackendError;
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use std::env;

async fn smtp_build() -> Result<SmtpTransport, BackendError> {
    let url = env::var("SMTP").expect("SMTP key not set in .env");

    let mailer = SmtpTransport::from_url(&url)
        .map_err(|_| BackendError::InternalError)?
        .build();

    Ok(mailer)
}

async fn send_email(to: String, subject: String, html: String) -> Result<(), BackendError> {
    let message = Message::builder()
        .to(to.parse().map_err(|_| BackendError::InternalError)?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html)
        .map_err(|_| BackendError::InternalError)?;

    smtp_build()
        .await?
        .send(&message)
        .map_err(|_| BackendError::InternalError)?;
    Ok(())
}

// async fn send_reset_password_email(email: String) -> Result<(), BackendError> {
//     send_email(email, "Сброс пароля".to_string(), html);
//     Ok(())
// }
