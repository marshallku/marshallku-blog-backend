use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    auth::guard::AuthUser,
    env::state::AppState,
    models::{comment::Comment, user::UserRole},
};

pub async fn delete(
    AuthUser { user }: AuthUser,
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    if user.role != UserRole::Root {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "message": "You don't have permission to delete this comment" })),
        )
            .into_response();
    }

    match Comment::delete(&state.db, &id).await {
        Ok(_) => (
            StatusCode::OK,
            Json(json!({ "message": "Comment deleted successfully" })),
        )
            .into_response(),
        Err(e) => {
            log::error!("Failed to delete comment: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "message": "Failed to delete comment" })),
            )
                .into_response()
        }
    }
}
