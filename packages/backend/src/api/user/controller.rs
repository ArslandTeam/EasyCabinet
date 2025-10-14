// use axum::{Json, extract::State, response::IntoResponse};

// use crate::{AppState, BackendError, api::user::service};

// pub async fn get_profile(
//     State(state): State<AppState>,
//     Json(payload): Json,
// ) -> Result<impl IntoResponse, BackendError> {
//     let profile = service::get_profile(&state.conn, uuid)
//         .await
//         .map_err(|_| BackendError::InternalError)?;

//     Ok(Json(profile))
// }
