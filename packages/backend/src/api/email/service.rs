use crate::BackendError;
use lettre::{Message, SmtpTransport, Transport, message::header::ContentType};
use minijinja::{Environment, context};
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
        .from(
            env::var("EMAIL_FROM")
                .expect("EMAIL_FROM key not set in .env")
                .parse()
                .map_err(|_| BackendError::InternalError)?,
        )
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

pub async fn send_reset_password_email(
    email: String,
    reset_token: String,
) -> Result<(), BackendError> {
    let html = render_reset_password_template(reset_token);
    send_email(email, "Сброс пароля".to_string(), html).await?;
    Ok(())
}

fn render_reset_password_template(reset_token: String) -> String {
    let frontend_url = env::var("FRONTEND_URL").expect("FRONTEND_URL key not set in .env");
    let mut env = Environment::new();
    env.add_template(
        "reset_password.html",
        include_str!("./templates/reset_password.html"),
    )
    .unwrap();
    let template = env.get_template("reset_password.html").unwrap();
    template
        .render(context! {
            frontend_url => frontend_url,
            reset_token => reset_token
        })
        .unwrap()
}
