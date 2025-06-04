use axum::{
    routing::{get, post},
    Router,
};

use crate::env::state::AppState;

pub const API_VERSION_PREFIX: &str = "/api/v2";

pub fn app() -> Router<AppState> {
    Router::new()
        // TODO: Remove after migration
        .route("/", get(super::index::get))
        .route("/auth/signin", post(super::auth::signin::post))
        .route("/auth/signup", post(super::auth::signup::post))
        .route("/auth/status", get(super::auth::status::get))
        .route("/comment/create", post(super::comments::create::post))
        .route("/comment/list", get(super::comments::list::get))
        .route("/comment/recent", get(super::recent::index::get))
        .route(
            &format!("{}/auth/signin", API_VERSION_PREFIX),
            post(super::auth::signin::post),
        )
        .route(
            &format!("{}/auth/signup", API_VERSION_PREFIX),
            post(super::auth::signup::post),
        )
        .route(
            &format!("{}/auth/status", API_VERSION_PREFIX),
            get(super::auth::status::get),
        )
        .route(
            &format!("{}/comment/create", API_VERSION_PREFIX),
            post(super::comments::create::post),
        )
        .route(
            &format!("{}/comment/list", API_VERSION_PREFIX),
            get(super::comments::list::get),
        )
        .route(
            &format!("{}/recent", API_VERSION_PREFIX),
            get(super::recent::index::get),
        )
}
