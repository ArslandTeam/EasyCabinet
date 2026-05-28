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

pub async fn update_profile(
    State(state): State<AppState>,
    Extension(payload): Extension<auth::jwt::JwtPayload>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, BackendError> {
    let mut is_alex = false;
    let mut skin: Option<Bytes> = None;
    let mut cape: Option<Bytes> = None;
    let mut del_skin = false;
    let mut del_cape = false;

    let parse_bool = |data: &[u8]| -> bool {
        let text = String::from_utf8_lossy(data);
        text.trim().eq_ignore_ascii_case("true")
    };

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| BackendError::BadRequest("Invalid multipart data".into()))?
    {
        let name = field.name().unwrap_or_default().to_string();
        let data = field
            .bytes()
            .await
            .map_err(|_| BackendError::BadRequest("Error read file".into()))?;

        match &*name {
            "is_alex" => is_alex = parse_bool(&data),
            "del_skin" => del_skin = parse_bool(&data),
            "del_cape" => del_cape = parse_bool(&data),
            "skin" if !data.is_empty() => skin = Some(data),
            "cape" if !data.is_empty() => cape = Some(data),
            _ => {}
        }
    }

    let profile = dto::ReqwestProfileDTO {
        is_alex,
        del_cape,
        del_skin,
    };

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
