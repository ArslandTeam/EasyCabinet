use crate::{BackendError, generate_config::CONFIG};
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use std::sync::LazyLock;

static SMTP_MAILER: LazyLock<SmtpTransport> = LazyLock::new(|| {
    SmtpTransport::from_url(&CONFIG.smtp)
        .expect("Not correct format smtp url")
        .build()
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

async fn render_template(
    file: &str,
    replacements: &[(&str, &str)],
) -> Result<String, BackendError> {
    let path = std::path::Path::new("templates_email").join(file);
    let content = tokio::fs::read_to_string(&path).await.map_err(|e| {
        tracing::error!("{e}");
        BackendError::InternalError
    })?;

    Ok(replacements
        .iter()
        .fold(content, |text, (from, to)| text.replace(from, to)))
}

pub async fn send_reset_password_email(email: &str, reset_token: &str) -> Result<(), BackendError> {
    let html = render_template(
        "verify_email.html",
        &[
            ("{{ frontend_url }}", &CONFIG.frontend_url),
            ("{{ reset_token }}", reset_token),
        ],
    )
    .await?;
    send_email(email.to_string(), "Сброс пароля".to_string(), html).await
}

pub async fn send_verify_email(email: &str, code: u32) -> Result<(), BackendError> {
    let html = render_template(
        "verify_email.html",
        &[
            ("{{ frontend_url }}", &CONFIG.frontend_url),
            ("{{ code }}", &code.to_string()),
        ],
    )
    .await?;
    send_email(email.to_string(), "Подтверждение почты".to_string(), html).await
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    /// Example: `RENDER_TEMPLATE="verify_email.html" FRONTEND_URL="http://example.com" CODE="123456" cargo test render_template_test -- --nocapture`
    async fn render_template_test() {
        let result = render_template(
            &std::env::var("RENDER_TEMPLATE").unwrap(),
            &[
                (
                    "{{ frontend_url }}",
                    &std::env::var("FRONTEND_URL").unwrap(),
                ),
                ("{{ code }}", &std::env::var("CODE").unwrap()),
            ],
        )
        .await;

        assert!(result.is_ok(), "{:?}", result.err());
        println!("{}", result.unwrap());
    }
}
