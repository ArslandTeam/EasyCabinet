use axum::{
    Json,
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::SignedCookieJar;

use crate::{
    AppState, BackendError,
    api::{auth::jwt::extract_jwt_token, user::service},
};

pub async fn get_profile(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    let user = extract_jwt_token(&jar)?;
    let profile = service::get_profile(&state.conn, user.uuid).await?;
    Ok(Json(profile))
}

// TODO хуйня на постной масле. Надо логику на js переосмыслить и переписать
pub async fn update_profile(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, BackendError> {
    let user = extract_jwt_token(&jar)?;

    let mut skin: Option<Bytes> = None;
    let mut cape: Option<Bytes> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| BackendError::BadRequest("Invalid multipart data".into()))?
    {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        match name.as_str() {
            "skin" if !data.is_empty() => skin = Some(data),
            "cape" if !data.is_empty() => cape = Some(data),
            _ => {}
        }
    }

    service::update_profile(user, &state.conn, skin.as_deref(), cape.as_deref()).await?;

    Ok(StatusCode::OK)
}
