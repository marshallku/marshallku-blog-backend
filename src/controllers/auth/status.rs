use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::auth::guard::AuthUser;

pub async fn get(AuthUser { user }: AuthUser) -> impl IntoResponse {
    (StatusCode::OK, Json(user))
}
