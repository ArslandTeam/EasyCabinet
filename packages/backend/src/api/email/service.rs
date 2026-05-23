use crate::{BackendError, generate_config::CONFIG};
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};

async fn smtp_build() -> Result<SmtpTransport, BackendError> {
    let mailer = SmtpTransport::from_url(&CONFIG.smtp)
        .map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?
        .build();

    Ok(mailer)
}

async fn send_email(email: &str, subject: &str, html: String) -> Result<(), BackendError> {
    let message = Message::builder()
        .from(CONFIG.email_from.clone())
        .to(email.parse().map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html)
        .map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?;

    smtp_build().await?.send(&message).map_err(|e| {
        tracing::error!("{e}");
        BackendError::InternalError
    })?;

    Ok(())
}

pub async fn send_reset_password_email(email: &str, reset_token: &str) -> Result<(), BackendError> {
    let html = render_reset_password_template(reset_token);
    send_email(email, "Сброс пароля", html).await?;
    Ok(())
}

pub async fn send_verify_email(email: &str, code: u32) -> Result<(), BackendError> {
    let html = render_verify_email_template(code);
    send_email(email, "Подтверждение почты", html).await?;
    Ok(())
}

fn render_reset_password_template(reset_token: &str) -> String {
    let mut env = minijinja::Environment::new();
    env.add_template(
        "reset_password.html",
        include_str!("./templates/reset_password.html"),
    )
    .unwrap();
    let template = env.get_template("reset_password.html").unwrap();
    template
        .render(minijinja::context! {
            frontend_url => &CONFIG.frontend_url,
            reset_token => reset_token
        })
        .unwrap()
}

fn render_verify_email_template(code: u32) -> String {
    let mut env = minijinja::Environment::new();
    env.add_template(
        "verify_email.html",
        include_str!("./templates/verify_email.html"),
    )
    .unwrap();
    let template = env.get_template("verify_email.html").unwrap();
    template
        .render(minijinja::context! {
            frontend_url => &CONFIG.frontend_url,
            code => code
        })
        .unwrap()
}
