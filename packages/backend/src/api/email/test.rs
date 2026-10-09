use super::service::*;

#[tokio::test]
async fn test_send_mail() {
    let email = std::env::var("EMAIL").unwrap();
    let html = "
    <!DOCTYPE html>
    <html>
        <body>
            <h1>Test email</h1>
        </body>
    </html>
    "
    .to_string();

    let result = send_email(email, "Test Email Easy Cabinet".to_string(), html).await;

    assert!(result.is_ok(), "{:?}", result.err());
    println!("{:?}", result.unwrap());
}

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
