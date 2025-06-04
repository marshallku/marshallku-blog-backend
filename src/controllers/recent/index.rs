use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::{env::state::AppState, models::comment::Comment};

#[derive(Deserialize)]
pub struct RecentCommentsQuery {
    #[serde(rename = "limit", default = "default_limit")]
    pub limit: i64,
}

fn default_limit() -> i64 {
    6
}

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<RecentCommentsQuery>,
) -> impl IntoResponse {
    let comments = Comment::get_recent(&state.db, query.limit).await;

    if comments.is_err() {
        log::error!("Failed to get comments: {:?}", comments.err());
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to get comments" })),
        )
            .into_response();
    }

    (StatusCode::OK, Json(comments.unwrap())).into_response()
}
