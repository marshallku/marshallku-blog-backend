use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::cookie::CookieJar;
use bson::{doc, oid::ObjectId};

use crate::{constants::auth::TOKEN_COOKIE_KEY, env::state::AppState, models::user::User};

use super::token::Token;

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
        let token = cookie_jar.get(TOKEN_COOKIE_KEY);

        if token.is_none() {
            return Err((StatusCode::UNAUTHORIZED, "Missing auth-token cookie"));
        }

        let token = token.unwrap().value();
        let token_claims = Token::parse(&token, &state.jwt_secret).map_err(|e| {
            println!("Error parsing token: {:?}", e);
            (StatusCode::UNAUTHORIZED, "Invalid auth-token cookie")
        })?;
        let token_id = ObjectId::parse_str(&token_claims.sub).map_err(|e| {
            println!("Error parsing token id: {:?}", e);
            (StatusCode::UNAUTHORIZED, "Invalid auth-token cookie")
        })?;

        println!("Token claims: {:?}", token_claims);

        let collection = state.db.collection::<User>("users");
        let user: Result<Option<User>, StatusCode> = collection
            .find_one(doc! { "_id": token_id })
            .await
            .map_err(|err| {
                println!("Error finding user: {:?}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            });

        if user.is_err() {
            println!("Error finding user: {:?}", user.err());
            return Err((StatusCode::UNAUTHORIZED, "Invalid auth-token cookie"));
        }

        let user = user.unwrap();

        if user.is_none() {
            println!("User not found");
            return Err((StatusCode::UNAUTHORIZED, "Invalid auth-token cookie"));
        }

        let user = user.unwrap();

        Ok(AuthUser {
            user_id: user.id.unwrap().to_string(),
        })
    }
}
