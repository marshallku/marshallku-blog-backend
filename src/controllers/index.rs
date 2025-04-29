use axum::{extract::State, http::StatusCode, response::IntoResponse};
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use crate::env::state::AppState;

pub async fn get() -> impl IntoResponse {
    let user = get_user(State(AppState::new().await.unwrap()))
        .await
        .unwrap();

    user.name
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub password: String,
    pub role: String,
}

pub async fn get_user(State(state): State<AppState>) -> Result<User, StatusCode> {
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
