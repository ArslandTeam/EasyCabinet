use super::{dto::*, service::*};
use axum::{
    extract::{Query, State},
    http::{HeaderMap, header::USER_AGENT},
    response::{IntoResponse, Redirect},
};
use axum_extra::extract::SignedCookieJar;
use http_body_util::BodyExt;
use hyper::{Method, Request};

use crate::{AppState, BackendError, api::auth::service::AuthService, generate_config::CONFIG};

pub struct OAuth2Controller;

impl OAuth2Controller {
    pub async fn discord(
        headers: HeaderMap,
        State(state): State<AppState>,
        Query(params): Query<CallbackParams>,
        jar: SignedCookieJar,
    ) -> Result<impl IntoResponse, BackendError> {
        let user_agent = headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("Unknown");

        let redirect_uri = format!("{}/auth/discord", CONFIG.backend_url);

        let form_body = format!(
            "client_id={}&client_secret={}&grant_type=authorization_code&code={}&redirect_uri={}",
            CONFIG.client_id_discord, CONFIG.client_secret_discord, params.code, redirect_uri
        );

        let token_req = Request::builder()
            .method(Method::POST)
            .uri("https://discord.com/api/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(axum::body::Body::from(form_body))
            .map_err(|e| {
                tracing::error!("{e}");
                BackendError::InternalError
            })?;

        let res = state.http_client.request(token_req).await.map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?;

        if !res.status().is_success() {
            tracing::error!("Discord returned error status: {}", res.status());
            return Err(BackendError::InternalError);
        }

        let access_token = OAuth2Service::discord_token(res).await?.access_token;

        let user_req = Request::builder()
            .method(Method::GET)
            .uri("https://discord.com/api/v10/users/@me")
            .header("Authorization", format!("Bearer {access_token}"))
            .body(axum::body::Body::empty())
            .map_err(|e| {
                tracing::error!("{e}");
                BackendError::InternalError
            })?;

        let res = state.http_client.request(user_req).await.map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?;

        if !res.status().is_success() {
            tracing::error!("{}", res.status());
            return Err(BackendError::InternalError);
        }

        let body_bytes = res
            .into_body()
            .collect()
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                BackendError::InternalError
            })?
            .to_bytes();

        let user_data: DiscordUserResponse = serde_json::from_slice(&body_bytes).map_err(|e| {
            tracing::error!("{e}");
            BackendError::InternalError
        })?;

        let Some(email) = user_data.email.as_deref() else {
            return Err(BackendError::BadRequest(
                "Email not set in Discord account".into(),
            ));
        };

        let (access_token, refresh_token) = OAuth2Service::create_user_discord(
            &state.db,
            &state.cache,
            &user_data.id,
            &user_data.username,
            email,
            user_agent,
        )
        .await?;

        let jar = AuthService::set_refresh_token_cookie(jar, refresh_token).await;
        let jar = AuthService::set_access_token(jar, access_token).await;
        Ok((
            jar,
            Redirect::to(&format!("{}/profile", &CONFIG.frontend_url)),
        ))
    }
}
