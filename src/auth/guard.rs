use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::cookie::CookieJar;

use crate::{constants::auth::TOKEN_COOKIE_KEY, env::state::AppState};

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: String,
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
        let token = cookie_jar
            .get(TOKEN_COOKIE_KEY)
            .ok_or((StatusCode::UNAUTHORIZED, "Missing auth-token cookie"))?
            .value();

        // TODO: Parse token
        Ok(AuthUser {
            user_id: "placeholder".to_string(),
        })
    }
}
