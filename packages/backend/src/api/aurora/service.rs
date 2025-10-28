// TODO переписать и добавить в ответ json строки со скинами
use crate::{
    BackendError,
    api::{aurora, auth, entities, user},
};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};

pub async fn auth(
    db: &DatabaseConnection,
    login: String,
    password: String,
) -> Result<Json<Value>, BackendError> {
    let user = auth::service::verify_auth(db, login.clone(), password)
        .await
        .unwrap();
    let access_token = uuid::Uuid::new_v4().to_string();
    user::service::update_user(
        db,
        entities::users::Column::AccessToken,
        access_token.clone(),
        entities::users::Column::Login,
        login,
    )
    .await
    .map_err(|_| BackendError::InternalError)?;

    Ok(Json(json!({
        "username": user.login,
        "userUUID": user.uuid,
        "accessToken": access_token
    })))
}

pub async fn join(
    db: &DatabaseConnection,
    body: aurora::dto::RequestJoinDto,
) -> Result<bool, BackendError> {
    match user::service::find_user(db, entities::users::Column::Uuid, body.user_uuid.clone())
        .await
        .map_err(|_| BackendError::InternalError)?
    {
        Some(user) if user.access_token.as_deref() == Some(&body.access_token) => {
            user::service::update_user(
                db,
                entities::users::Column::ServerId,
                body.server_id,
                entities::users::Column::Uuid,
                body.user_uuid,
            )
            .await
            .map_err(|_| BackendError::InternalError)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

pub async fn has_join(
    db: &DatabaseConnection,
    body: aurora::dto::RequestHasJoinedDto,
) -> Result<Json<Value>, BackendError> {
    let user = match user::service::find_user(db, entities::users::Column::Login, body.username)
        .await
        .map_err(|_| BackendError::InternalError)?
    {
        Some(user) if user.server_id.as_deref() == Some(&body.server_id) => user,
        _ => return Err(BackendError::InternalError),
    };

    Ok(Json(json!({
        "userUUID": user.uuid,
    })))
}

pub async fn profile(
    db: &DatabaseConnection,
    body: aurora::dto::RequestProfileDTO,
) -> Result<Json<Value>, BackendError> {
    let user = user::service::find_user(db, entities::users::Column::Uuid, body.user_uuid)
        .await
        .map_err(|_| BackendError::InternalError)?
        .ok_or(BackendError::BadRequest("User not found".into()))?;

    Ok(Json(json!({
        "username": user.login
    })))
}

pub async fn profiles(
    db: &DatabaseConnection,
    body: aurora::dto::RequestProfilesDto,
) -> Result<Json<Value>, BackendError> {
    let users = user::service::find_users(db, entities::users::Column::Login, body.usernames)
        .await
        .map_err(|_| BackendError::InternalError)?;

    Ok(Json(
        users
            .into_iter()
            .map(|user| {
                json!({
                    "id": user.uuid,
                    "name": user.login,
                })
            })
            .collect(),
    ))
}
