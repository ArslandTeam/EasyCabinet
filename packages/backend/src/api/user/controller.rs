use crate::{
    AppState, BackendError,
    api::{auth, user},
};
use axum::{
    Json,
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};
use axum_extra::extract::SignedCookieJar;

pub async fn get_profile(
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> Result<impl IntoResponse, BackendError> {
    let user = auth::jwt::extract_jwt_token(&jar)?;
    let profile = user::service::get_profile(&state.conn, user.uuid).await?;
    Ok(Json(profile))
}

// TODO хуйня на постной масле. Надо логику на js переосмыслить и переписать
pub async fn update_profile(
    State(state): State<AppState>,
    jar: SignedCookieJar,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, BackendError> {
    let user = auth::jwt::extract_jwt_token(&jar)?;
    let mut is_alex: bool = false;
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
            "isAlex" => {
                let val = String::from_utf8_lossy(&data).trim().to_lowercase();
                match val.as_str() {
                    "true" => is_alex = true,
                    "false" => is_alex = false,
                    _ => {}
                }
            }
            "skin" if !data.is_empty() => skin = Some(data),
            "cape" if !data.is_empty() => cape = Some(data),
            _ => {}
        }
    }

    let profile = user::dto::ReqwestProfileDTO { is_alex };

    user::service::update_profile(&state.conn, user, profile, skin.as_deref(), cape.as_deref())
        .await?;

    Ok(StatusCode::OK)
}
