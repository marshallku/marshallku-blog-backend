use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;

use crate::{
    env::state::AppState,
    models::user::{User, UserRole},
    utils::encryption::hash_password,
};

#[derive(Deserialize)]
pub struct SignUpPayload {
    pub name: String,
    pub password: String,
}

pub async fn post(
    State(app_state): State<AppState>,
    Json(payload): Json<SignUpPayload>,
) -> impl IntoResponse {
    let hashed_password = hash_password(&payload.password);

    if hashed_password.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to hash password" })),
        );
    }

    let user = User {
        name: payload.name,
        password: hashed_password.unwrap(),
        role: UserRole::User,
        id: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let user = User::create(&app_state.db, user).await;

    if user.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to create user" })),
        );
    }

    let user = user.unwrap();

    (
        StatusCode::CREATED,
        Json(json!({
            "name": user.name,
            "role": user.role,
            "createdAt": user.created_at.to_rfc3339(),
            "updatedAt": user.updated_at.to_rfc3339(),
        })),
    )
}
