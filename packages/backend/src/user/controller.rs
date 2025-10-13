use axum::{extract::State, response::IntoResponse};

use crate::{AppState, BackendError};

pub async fn get_profile(State(state): State<AppState>) -> Result<impl IntoResponse, BackendError> {
    Ok(())
}
