use crate::{BackendError, generate_config::CONFIG};
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use std::sync::LazyLock;

static SMTP_MAILER: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::from_url(&CONFIG.smtp)
        .expect("Not correct format smtp url")
        .build()
});

static MINIJINJA: LazyLock<minijinja::Environment<'_>> = LazyLock::new(|| {
    let mut env = minijinja::Environment::new();
    env.set_loader(minijinja::path_loader("templates_email"));
    env
});

async fn send_email(email: String, subject: String, html: String) -> Result<(), BackendError> {
    tokio::task::spawn_blocking(move || {
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

        SMTP_MAILER.send(&message).map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?;

        Ok(())
    })
    .await
    .map_err(|e| {
        tracing::error!("{e}");
        BackendError::InternalError
    })?
}

fn render_template(file: &str, ctx: minijinja::Value) -> String {
    let template = MINIJINJA.get_template(file).unwrap();
    template.render(ctx).unwrap()
}

pub async fn send_reset_password_email(email: &str, reset_token: &str) -> Result<(), BackendError> {
    let html = render_template(
        "reset_password.html",
        minijinja::context! {
            frontend_url => &CONFIG.frontend_url,
            reset_token => reset_token
        },
    );
    send_email(email.to_string(), "Сброс пароля".to_string(), html).await
}

pub async fn send_verify_email(email: &str, code: u32) -> Result<(), BackendError> {
    let html = render_template(
        "verify_email.html",
        minijinja::context! {
            frontend_url => &CONFIG.frontend_url,
            code => code
        },
    );
    send_email(email.to_string(), "Подтверждение почты".to_string(), html).await
}
