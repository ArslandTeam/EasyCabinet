use axum::{Json, extract::State, response::IntoResponse};
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
