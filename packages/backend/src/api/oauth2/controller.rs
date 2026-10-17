use super::{dto::*, service::*};
use axum::{
    body::Body,
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
    pub async fn get_discord_link(
        State(state): State<AppState>,
        jar: SignedCookieJar,
    ) -> Result<impl IntoResponse, BackendError> {
        let link_token = OAuth2Service::get_link_discord(&state.cache, &jar).await?;
        let state_param = format!("link:{}", link_token);
        let redirect_uri = format!("{}/auth/discord", CONFIG.backend_url);

        let discord_url = format!(
            "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify%20email&state={}",
            CONFIG.client_id_discord, &redirect_uri, &state_param
        );

        Ok(Redirect::to(&discord_url))
    }

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

        let form_body = format!(
            "client_id={}&client_secret={}&grant_type=authorization_code&code={}&redirect_uri={}",
            CONFIG.client_id_discord,
            CONFIG.client_secret_discord,
            params.code,
            format!("{}/auth/discord", CONFIG.backend_url)
        );

        let token_req = Request::builder()
            .method(Method::POST)
            .uri("https://discord.com/api/oauth2/token")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(axum::body::Body::from(form_body))
            .map_err(|e| BackendError::InternalError(e.into()))?;

        let res = state
            .http_client
            .request(token_req)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))?;

        if !res.status().is_success() {
            return Err(BackendError::InternalError(
                format!("Discord returned error status: {}", res.status()).into(),
            ));
        }

        let body_bytes = axum::body::to_bytes(Body::new(res.into_body()), usize::MAX)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))?;

        let token_data: DiscordTokenResponse = serde_json::from_slice(&body_bytes)
            .map_err(|e| BackendError::InternalError(e.into()))?;

        let user_req = Request::builder()
            .method(Method::GET)
            .uri("https://discord.com/api/v10/users/@me")
            .header(
                "Authorization",
                format!("Bearer {}", token_data.access_token),
            )
            .body(Body::empty())
            .map_err(|e| BackendError::InternalError(e.into()))?;

        let res = state
            .http_client
            .request(user_req)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))?;

        if !res.status().is_success() {
            return Err(BackendError::InternalError("".into()));
        }

        let body_bytes = axum::body::to_bytes(Body::new(res.into_body()), usize::MAX)
            .await
            .map_err(|e| BackendError::InternalError(e.into()))?;

        let user_data: DiscordUserResponse = serde_json::from_slice(&body_bytes)
            .map_err(|e| BackendError::InternalError(e.into()))?;

        // if params.state.starts_with("link") {
        //     let uuid = AuthService::extract_jwt_token(&jar).await?.uuid;

        //     OAuth2Service::link_discord(&state.db, &user_data.id, &uuid).await?;

        //     return Ok((
        //         jar,
        //         Redirect::to(&format!("{}/profile?link=success", &CONFIG.frontend_url)),
        //     ));
        // }

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
