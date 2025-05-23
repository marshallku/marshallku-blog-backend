use axum::{
    routing::{get, post},
    Router,
};

use crate::env::state::AppState;

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/", get(super::index::get))
        .route("/auth/signin", post(super::auth::signin::post))
        .route("/auth/signup", post(super::auth::signup::post))
        .route("/auth/status", get(super::auth::status::get))
        .route("/comment/create", post(super::comments::create::post))
        .route("/comment/list", get(super::comments::list::get))
}
