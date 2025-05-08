use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use cookie::{Cookie, SameSite};
use reqwest::header::SET_COOKIE;
use serde::Deserialize;
use serde_json::json;
use time::Duration;

use crate::{
    auth::token::Token, constants::auth::TOKEN_COOKIE_KEY, env::state::AppState,
    models::user::User, utils::encryption::verify_password,
};

#[derive(Deserialize)]
pub struct SignInPayload {
    pub name: String,
    pub password: String,
}

pub async fn post(
    State(state): State<AppState>,
    Json(payload): Json<SignInPayload>,
) -> impl IntoResponse {
    let user = User::find_by_name(&state.db, &payload.name).await;

    if user.is_err() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid name or password" })),
        )
            .into_response();
    }

    let user = user.unwrap();
    if !verify_password(&payload.password, &user.password).unwrap() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid name or password" })),
        )
            .into_response();
    }

    let token = Token::from_user(&user, &state.jwt_secret);

    if token.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to generate token" })),
        )
            .into_response();
    }

    let token = token.unwrap();

    let cookie = Cookie::build((TOKEN_COOKIE_KEY, token))
        .path("/")
        .secure(true)
        .http_only(true)
        .max_age(Duration::days(1))
        .same_site(SameSite::None)
        .domain(state.cookie_domain);

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.to_string().parse().unwrap());

    (
        StatusCode::OK,
        headers,
        Json(json!({ "message": "Login successful" })),
    )
        .into_response()
}
