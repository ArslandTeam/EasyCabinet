use crate::{
    BackendError,
    api::{
        aurora,
        auth::service::AuthService,
        database::{entities::users, service::DatabaseService},
        storage_manager::StorageService,
        user::service::UserService,
    },
};
use axum::Json;
use sea_orm::DatabaseConnection;
use serde_json::{Value, json};

pub struct AuroraService;

impl AuroraService {
    pub async fn auth(
        db: &DatabaseConnection,
        storage: &StorageService,
        login: String,
        password: String,
    ) -> Result<Json<Value>, BackendError> {
        let user = AuthService::verify_auth(db, &login, password)
            .await
            .map_err(|_| BackendError::BadRequestAurora("Incorecrt password or login".into()))?;

        let access_token = uuid::Uuid::new_v4().to_string();

        DatabaseService::update_user(
            db,
            users::Column::AccessToken,
            &access_token,
            users::Column::Login,
            &login,
        )
        .await
        .map_err(|_| BackendError::InternalError)?;

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Json(json!({
            "success": true,
            "result": {
                "username": user.login,
                "userUUID": user.uuid,
                "accessToken": access_token,
                "isAlex": textures.is_alex,
                "skinUrl": textures.skin_url,
                "capeUrl": textures.cape_url,
            }
        })))
    }

    pub async fn join(
        db: &DatabaseConnection,
        body: aurora::dto::RequestJoinDto,
    ) -> Result<Json<Value>, BackendError> {
        let Some(user) = DatabaseService::find_user(db, users::Column::Uuid, &body.user_uuid)
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

        DatabaseService::update_user(
            db,
            users::Column::ServerId,
            &body.server_id,
            users::Column::Uuid,
            &body.user_uuid,
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
        storage: &StorageService,
        body: aurora::dto::RequestHasJoinedDto,
    ) -> Result<Json<Value>, BackendError> {
        let user = DatabaseService::find_user(db, users::Column::Login, &body.username)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequestAurora("User not found".into()))?;

        if user.server_id != Some(body.server_id) {
            return Err(BackendError::BadRequestAurora("Invalid server id".into()));
        }

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Json(json!({
            "success": true,
            "result": {
                "userUUID": user.uuid,
                "isAlex": textures.is_alex,
                "skinUrl": textures.skin_url,
                "capeUrl": textures.cape_url,
            }
        })))
    }

    pub async fn profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        body: aurora::dto::RequestProfileDTO,
    ) -> Result<Json<Value>, BackendError> {
        let user = DatabaseService::find_user(db, users::Column::Uuid, &body.user_uuid)
            .await
            .map_err(|_| BackendError::InternalError)?
            .ok_or(BackendError::BadRequestAurora("User not found".into()))?;

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Json(json!({
            "success": true,
            "result": {
                "username": user.login,
                "isAlex": textures.is_alex,
                "skinUrl": textures.skin_url,
                "capeUrl": textures.cape_url,
            }
        })))
    }

    pub async fn profiles(
        db: &DatabaseConnection,
        body: aurora::dto::RequestProfilesDto,
    ) -> Result<Json<Value>, BackendError> {
        let users = DatabaseService::find_users(db, users::Column::Login, body.usernames)
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
