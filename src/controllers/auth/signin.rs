use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Deserialize;
use serde_json::json;

use crate::{env::state::AppState, models::user::User, utils::encryption::verify_password};

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
        );
    }

    let user = user.unwrap();

    if user.is_none() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid name or password" })),
        );
    }

    let user = user.unwrap();
    if !verify_password(&payload.password, &user.password).unwrap() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "message": "Invalid name or password" })),
        );
    }

    // TODO: Need to implement token generation
    println!("User: {:?}", user);

    (
        StatusCode::OK,
        Json(json!({ "message": "Login successful" })),
    )
}
