use super::dto::*;
use crate::{
    BackendError,
    api::{
        auth::service::AuthService,
        database::{entities::users, service::DatabaseService},
        storage_manager::StorageService,
        user::service::UserService,
    },
};
use axum::Json;
use sea_orm::{ColumnTrait, DatabaseConnection};

pub struct AuroraService;

impl AuroraService {
    pub fn response<T>(result: T) -> Json<AuroraResponse<T>> {
        Json(AuroraResponse {
            success: true,
            result,
        })
    }

    pub async fn auth(
        db: &DatabaseConnection,
        storage: &StorageService,
        login: String,
        password: String,
    ) -> Result<Json<AuroraResponse<AuthResponseDto>>, BackendError> {
        let user = AuthService::verify_auth(db, &login, &password)
            .await
            .map_err(|_| BackendError::BadRequestAurora("Incorecrt password or login".into()))?;

        let access_token = uuid::Uuid::new_v4().to_string();

        DatabaseService::update_user(
            db,
            users::Column::AccessToken,
            Some(&access_token),
            users::Column::Login,
            &login,
        )
        .await?;

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Self::response(AuthResponseDto {
            username: user.login,
            user_uuid: user.uuid,
            access_token,
            is_alex: textures.is_alex,
            skin_url: textures.skin_url,
            cape_url: textures.cape_url,
        }))
    }

    pub async fn join(
        db: &DatabaseConnection,
        body: RequestJoinDto,
    ) -> Result<Json<AuroraResponse<bool>>, BackendError> {
        let Some(user) =
            DatabaseService::find::<users::Entity>(db, users::Column::Uuid, &body.user_uuid)
                .await?
        else {
            return Ok(Self::response(false));
        };

        if user.access_token != Some(body.access_token) {
            return Ok(Self::response(false));
        }

        DatabaseService::update_user(
            db,
            users::Column::ServerId,
            Some(&body.server_id),
            users::Column::Uuid,
            &body.user_uuid,
        )
        .await?;

        Ok(Self::response(true))
    }

    pub async fn has_join(
        db: &DatabaseConnection,
        storage: &StorageService,
        body: RequestHasJoinedDto,
    ) -> Result<Json<AuroraResponse<HasJoinResponseDto>>, BackendError> {
        let user = DatabaseService::find::<users::Entity>(db, users::Column::Login, &body.username)
            .await?
            .ok_or(BackendError::BadRequestAurora("User not found".into()))?;

        if user.server_id != Some(body.server_id) {
            return Err(BackendError::BadRequestAurora("Invalid server id".into()));
        }

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Self::response(HasJoinResponseDto {
            user_uuid: user.uuid,
            is_alex: textures.is_alex,
            skin_url: textures.skin_url,
            cape_url: textures.cape_url,
        }))
    }

    pub async fn profile(
        db: &DatabaseConnection,
        storage: &StorageService,
        body: RequestProfileDTO,
    ) -> Result<Json<AuroraResponse<ProfileResponseDto>>, BackendError> {
        let user = DatabaseService::find::<users::Entity>(db, users::Column::Uuid, &body.user_uuid)
            .await?
            .ok_or(BackendError::BadRequestAurora("User not found".into()))?;

        let textures = UserService::get_textures_data(storage, &user).await;

        Ok(Self::response(ProfileResponseDto {
            username: user.login,
            is_alex: textures.is_alex,
            skin_url: textures.skin_url,
            cape_url: textures.cape_url,
        }))
    }

    pub async fn profiles(
        db: &DatabaseConnection,
        body: RequestProfilesDto,
    ) -> Result<Json<AuroraResponse<Vec<ProfilesResponseDto>>>, BackendError> {
        let users = DatabaseService::find_all::<users::Entity, _>(
            db,
            users::Column::Login.is_in(body.usernames),
        )
        .await?;

        let resposne: Vec<ProfilesResponseDto> = users
            .into_iter()
            .map(|user| ProfilesResponseDto {
                id: user.id.to_string(),
                name: user.login,
            })
            .collect();

        Ok(Self::response(resposne))
    }
}
