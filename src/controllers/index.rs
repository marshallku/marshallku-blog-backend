use axum::{extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::doc;
use serde_json::to_string;

use crate::{
    auth::guard::AuthUser,
    env::state::AppState,
    models::{comment::Comment, user::User},
};

pub async fn get(
    State(state): State<AppState>,
    AuthUser { user_id }: AuthUser,
) -> impl IntoResponse {
    let user = get_user(state.clone()).await.unwrap();
    let comment = get_comment(state).await.unwrap();

    format!(
        "{{ \"user\": {}, \"comment\": {}, \"current_user_id\": {} }}",
        to_string(&user).unwrap(),
        to_string(&comment).unwrap(),
        user_id
    )
}

pub async fn get_user(state: AppState) -> Result<User, StatusCode> {
    let collection = state.db.collection::<User>("users");
    let user = collection
        .find_one(doc! {"name": "admin"})
        .await
        .map_err(|err| {
            println!("Error finding user: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(user.unwrap())
}

pub async fn get_comment(state: AppState) -> Result<Comment, StatusCode> {
    let collection = state.db.collection::<Comment>("comment");
    let comment = collection
        .find_one(doc! {"postSlug": "/dev/유튜브-썸네일-추출하기"})
        .await
        .map_err(|err| {
            println!("Error finding comment: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if let Some(comment) = comment {
        Ok(comment)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
