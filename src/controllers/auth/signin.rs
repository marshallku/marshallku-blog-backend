use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{
    auth::token::Token, constants::auth::TOKEN_COOKIE_KEY, env::state::AppState,
    models::user::User, utils::encryption::verify_password,
};

#[derive(Deserialize)]
pub struct SignInPayload {
    pub name: String,
    pub password: String,
}

pub async fn signin(
    State(app_state): State<AppState>,
    Json(payload): Json<SignInPayload>,
) -> impl IntoResponse {
    let user = User::find_by_name(&app_state.db, &payload.name).await;

    if user.is_err() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid name or password" })),
        )
            .into_response();
    }

    let user = user.unwrap();

    if user.is_none() {
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

    let mut headers = HeaderMap::new();
    let token = Token::from_user(&user);

    if token.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to generate token" })),
        )
            .into_response();
    }

    let token = token.unwrap();

    headers.insert(TOKEN_COOKIE_KEY, token.parse().unwrap());

    (
        StatusCode::OK,
        headers,
        Json(json!({ "message": "Login successful" })),
    )
        .into_response()
}
