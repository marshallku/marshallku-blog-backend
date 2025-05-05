use axum::{
    routing::{get, post},
    Router,
};

use crate::env::state::AppState;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(super::index::get))
        .route("/auth/signin", post(super::auth::signin::signin))
        .route("/auth/signup", post(super::auth::signup::signup))
        .route("/auth/status", get(super::auth::status::get))
}
