use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use bson::oid::ObjectId;
use chrono::Utc;
use serde::Deserialize;
use serde_json::json;
use validator::Validate;

use crate::{
    auth::guard::AuthUserOrPublic,
    env::state::AppState,
    models::{comment::Comment, user::UserRole},
};

#[derive(Deserialize, Validate)]
pub struct AddCommentPayload {
    #[serde(rename = "postSlug")]
    #[validate(length(min = 1, message = "Post slug cannot be empty"))]
    pub post_slug: String,

    #[validate(length(min = 1, message = "Name cannot be empty"))]
    pub name: String,

    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,

    #[validate(url(message = "Invalid URL format"))]
    pub url: Option<String>,

    #[validate(length(min = 1, message = "Comment body cannot be empty"))]
    pub body: String,

    #[serde(rename = "parentCommentId")]
    pub parent_comment_id: Option<String>,
}

pub async fn post(
    AuthUserOrPublic { user }: AuthUserOrPublic,
    State(state): State<AppState>,
    Json(payload): Json<AddCommentPayload>,
) -> impl IntoResponse {
    if let Err(errors) = payload.validate() {
        return (StatusCode::BAD_REQUEST, format!("{}", errors)).into_response();
    }

    let is_root = user.is_some() && user.unwrap().role == UserRole::Root;
    let comment = Comment {
        id: None,
        post_slug: payload.post_slug,
        name: payload.name,
        email: payload.email.unwrap_or_default(),
        url: payload.url.unwrap_or_default(),
        body: payload.body,
        parent_comment_id: payload
            .parent_comment_id
            .and_then(|id| ObjectId::parse_str(&id).ok()),
        by_post_author: is_root,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let comment_create_result = Comment::create(&state.db, comment).await;

    if comment_create_result.is_err() {
        log::error!(
            "Failed to create comment: {}",
            comment_create_result.err().unwrap()
        );
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "message": "Failed to create comment" })),
        )
            .into_response();
    }

    (
        StatusCode::CREATED,
        Json(json!(comment_create_result.unwrap())),
    )
        .into_response()
}
