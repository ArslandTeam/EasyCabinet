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
    env.add_template(
        "reset_password.html",
        include_str!("./templates/reset_password.html"),
    )
    .unwrap();
    env.add_template(
        "verify_email.html",
        include_str!("./templates/verify_email.html"),
    )
    .unwrap();

    env
});

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

    SMTP_MAILER.send(&message).map_err(|e| {
        tracing::error!("{e}");
        BackendError::InternalError
    })?;

    Ok(())
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
    send_email(email, "Сброс пароля", html).await
}

pub async fn send_verify_email(email: &str, code: u32) -> Result<(), BackendError> {
    let html = render_template(
        "verify_email.html",
        minijinja::context! {
            frontend_url => &CONFIG.frontend_url,
            code => code
        },
    );
    send_email(email, "Подтверждение почты", html).await
}
