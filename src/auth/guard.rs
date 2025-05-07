use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::cookie::CookieJar;

use crate::{constants::auth::TOKEN_COOKIE_KEY, env::state::AppState, models::user::User};

use super::token::Token;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user: User,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let cookie_jar = CookieJar::from_headers(&parts.headers);
        let token = cookie_jar.get(TOKEN_COOKIE_KEY);

        if token.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "Missing auth-token cookie"));
        }

        let token = token.unwrap().value();
        let user = get_user_from_token(&token, &state).await;

        if user.is_err() {
            return Err((StatusCode::UNAUTHORIZED, "Invalid auth-token cookie"));
        }

        let user = user.unwrap();

        Ok(AuthUser { user })
    }
}

#[derive(Debug, Clone)]
pub struct AuthUserOrPublic {
    #[allow(dead_code)]
    pub user: Option<User>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUserOrPublic
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);
        let cookie_jar = CookieJar::from_headers(&parts.headers);
        let token = cookie_jar.get(TOKEN_COOKIE_KEY);

        if token.is_none() {
            return Ok(AuthUserOrPublic { user: None });
        }

        let token = token.unwrap().value();
        let user = get_user_from_token(&token, &state).await;

        if user.is_err() {
            return Ok(AuthUserOrPublic { user: None });
        }

        let user = user.unwrap();

        Ok(AuthUserOrPublic { user: Some(user) })
    }
}

async fn get_user_from_token(
    token: &str,
    state: &AppState,
) -> Result<User, (StatusCode, &'static str)> {
    let token_claims = Token::parse(&token, &state.jwt_secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid auth-token cookie"))?;

    let user = User::find_by_id(&state.db, &token_claims.sub).await;

    if user.is_err() {
        return Err((StatusCode::UNAUTHORIZED, "Invalid auth-token cookie"));
    }

    Ok(user.unwrap())
}
