use axum::{
    Json,
    extract::{Multipart, State},
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
// pub async fn update_profile(
//     State(state): State<AppState>,
//     jar: SignedCookieJar,
//     mut multipart: Multipart,
// ) -> Result<impl IntoResponse, BackendError> {
//     while let Some(mut field) = multipart.next_field().await.unwrap() {
//         let name = field.name().unwrap().to_string();
//         let data = field.bytes().await.unwrap();
//     }
//     Ok(())
// }
