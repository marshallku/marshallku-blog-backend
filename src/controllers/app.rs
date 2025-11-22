use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::env::state::AppState;

pub const API_VERSION_PREFIX: &str = "/api/v2";

pub fn app() -> Router<AppState> {
    Router::new()
        .route("/health", get(super::health::get))
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
            &format!("{}/comment/:id", API_VERSION_PREFIX),
            delete(super::comments::delete::delete),
        )
        .route(
            &format!("{}/recent", API_VERSION_PREFIX),
            get(super::recent::index::get),
        )
        .route(
            &format!("{}/thumbnail/*path", API_VERSION_PREFIX),
            get(super::thumbnail::get::get),
        )
}
