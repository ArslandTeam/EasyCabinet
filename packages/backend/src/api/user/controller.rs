use crate::{
    AppState, BackendError,
    api::{
        auth,
        user::{dto, service::UserService},
    },
};
use axum::{
    Extension, Json,
    body::Bytes,
    extract::{Multipart, State},
    http::StatusCode,
    response::IntoResponse,
};

pub async fn get_profile(
    State(state): State<AppState>,
    Extension(payload): Extension<auth::jwt::JwtPayload>,
) -> Result<impl IntoResponse, BackendError> {
    let profile =
        UserService::get_profile(&state.conn, &state.storage, &state.cache, payload.uuid).await?;
    Ok((StatusCode::OK, Json(profile)))
}

// TODO хуйня на постной масле. Надо логику на js переосмыслить и переписать
pub async fn update_profile(
    State(state): State<AppState>,
    Extension(payload): Extension<auth::jwt::JwtPayload>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, BackendError> {
    let mut is_alex: bool = false;
    let mut skin: Option<Bytes> = None;
    let mut cape: Option<Bytes> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| BackendError::BadRequest("Invalid multipart data".into()))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let data = field.bytes().await.unwrap_or_default();

        match name.as_str() {
            "is_alex" => {
                let val = String::from_utf8_lossy(&data).trim().to_ascii_lowercase();
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

    let profile = dto::ReqwestProfileDTO { is_alex };

    UserService::update_profile(
        &state.conn,
        &state.storage,
        payload,
        profile,
        skin.as_deref(),
        cape.as_deref(),
    )
    .await?;

    Ok(StatusCode::OK)
}
