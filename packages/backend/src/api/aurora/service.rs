use crate::{
    BackendError,
    api::{aurora, auth::service::AuthService, entities, user::service::UserService},
};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};

#[derive(Default)]
pub struct AuroraService;

impl AuroraService {
    pub async fn auth(
        db: &DatabaseConnection,
        login: String,
        password: String,
    ) -> Result<Json<Value>, BackendError> {
        let user = AuthService::verify_auth(db, &login, password)
            .await
            .map_err(|_| BackendError::BadRequestAurora("Incorecrt password or login".into()))?;
        let access_token = uuid::Uuid::new_v4().to_string();
        UserService::update_user(
            db,
            entities::users::Column::AccessToken,
            &access_token,
            entities::users::Column::Login,
            &login,
        )
        .await
        .map_err(|_| BackendError::InternalError)?;

        Ok(Json(json!({
            "success": true,
            "result": {
                "username": user.login,
                "userUUID": user.uuid,
                "accessToken": access_token
            }
        })))
    }

    pub async fn join(
        db: &DatabaseConnection,
        body: aurora::dto::RequestJoinDto,
    ) -> Result<Json<Value>, BackendError> {
        let Some(user) = UserService::find_user(db, entities::users::Column::Uuid, &body.user_uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
        else {
            return Ok(Json(json!({
                "success": true,
                "result": false
            })));
        };

        if user.access_token != Some(body.access_token) {
            return Ok(Json(json!({
                "success": true,
                "result": false
            })));
        }

        UserService::update_user(
            db,
            entities::users::Column::Uuid,
            &body.user_uuid,
            entities::users::Column::ServerId,
            &body.server_id,
        )
        .await
        .map_err(|_| BackendError::InternalError)?;

        Ok(Json(json!({
            "success": true,
            "result": true
        })))
    }

    pub async fn has_join(
        db: &DatabaseConnection,
        body: aurora::dto::RequestHasJoinedDto,
    ) -> Result<Json<Value>, BackendError> {
        let Some(user) = UserService::find_user(db, entities::users::Column::Login, &body.username)
            .await
            .map_err(|_| BackendError::InternalError)?
        else {
            return Ok(Json(json!({
                "success": true,
                "result": false
            })));
        };

        if user.server_id != Some(body.server_id) {
            return Ok(Json(json!({
                "success": true,
                "result": false
            })));
        }

        Ok(Json(json!({
            "success": true,
            "result": {
                "userUUID": user.uuid,
            }
        })))
    }

    pub async fn profile(
        db: &DatabaseConnection,
        body: aurora::dto::RequestProfileDTO,
    ) -> Result<Json<Value>, BackendError> {
        let user = UserService::find_user(db, entities::users::Column::Uuid, &body.user_uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequestAurora("User not found".into()))?;

        Ok(Json(json!({
            "success": true,
            "result": {
                "username": user.login
            }
        })))
    }

    pub async fn profiles(
        db: &DatabaseConnection,
        body: aurora::dto::RequestProfilesDto,
    ) -> Result<Json<Value>, BackendError> {
        let users = UserService::find_users(db, entities::users::Column::Login, body.usernames)
            .await
            .map_err(|_| BackendError::BadRequestAurora("Users not found".into()))?;

        let resposne: Vec<Value> = users
            .into_iter()
            .map(|user| {
                json!({
                        "id": user.uuid,
                        "name": user.login,
                })
            })
            .collect();

        Ok(Json(json!({"success": true, "result": resposne})))
    }
}
