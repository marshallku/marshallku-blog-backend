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
pub struct ListCommentsQuery {
    #[serde(rename = "postSlug")]
    pub slug: String,
}

pub async fn get(
    State(state): State<AppState>,
    Query(query): Query<ListCommentsQuery>,
) -> impl IntoResponse {
    let comments = Comment::get_by_slug(&state.db, &query.slug).await;

    if comments.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to get comments" })),
        )
            .into_response();
    }

    (StatusCode::OK, Json(comments.unwrap())).into_response()
}
