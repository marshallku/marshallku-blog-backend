use std::env::var;

use axum::{
    http::{HeaderName, HeaderValue},
    serve,
};
use controllers::app::app;
use env::state::AppState;
use reqwest::Method;
use tokio::{net::TcpListener, signal};
use tower_http::{
    cors::CorsLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::{info, Level};
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use utils::log::trace_layer_on_request;

mod auth;
mod constants;
mod controllers;
mod database;
mod env;
mod models;
mod utils;

fn setup_tracing() {
    let fmt_layer = fmt::layer().with_target(false);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}

#[tokio::main]
async fn main() {
    setup_tracing();

    let state = AppState::new().await.unwrap();
    let address = format!("{}:{}", state.host, state.port);
    let trusted_domains = var("TRUSTED_DOMAINS").unwrap_or_default();
    let origins = trusted_domains
        .split(',')
        .map(|domain| domain.trim())
        .filter(|domain| !domain.is_empty())
        .filter_map(|domain| match HeaderValue::from_str(&domain.to_string()) {
            Ok(value) => Some(value),
            Err(_) => None,
        })
        .collect::<Vec<_>>();
    let cors_layer = CorsLayer::new()
        .allow_credentials(true)
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
            HeaderName::from_static("accept"),
            HeaderName::from_static("origin"),
        ])
        .allow_methods(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(origins);
    let app = app()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO))
                .on_request(trace_layer_on_request),
        )
        .layer(cors_layer)
        .with_state(state);
    let listener = TcpListener::bind(address.as_str()).await.unwrap();

    info!("Listening on http://{}", address);

    serve(listener, app.into_make_service())
        .with_graceful_shutdown(handle_shutdown())
        .await
        .unwrap();
}

async fn handle_shutdown() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("Shutting down...");
}
