#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(missing_docs)]

use axum::Router;
use axum_sessions::{async_session::MemoryStore, SessionLayer};
use dotenvy::dotenv;
use std::net::SocketAddr;
use std::{env, sync::Arc};
use tracing::log::warn;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub mod db;
pub mod middlewares;
pub mod routes;
mod services;
mod store;

// SETUP Constants

/// Server that is split into a Frontend to serve static files (Svelte) and Backend
/// Backend is further split into a non authorized area and a secure area
/// The Back end is using 2 middleware: sessions (managing session data) and user_secure (checking for authorization)
#[tokio::main]
async fn main() {
    dotenv().ok();

    let (port, host, secret, log_level, token) = from_env();

    // start tracing - level set by either RUST_LOG env variable or defaults to debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(log_level))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // configure server from environmental variables

    let addr: SocketAddr = format!("{}:{}", host, port)
        .parse()
        .expect("Can not parse address and port");

    // create store for backend.  Stores an api_token.
    let shared_state = Arc::new(store::Store::new(&token));

    let session_cookie = env::var("SESSION_COOKIE_NAME").ok().unwrap();
    // setup up sessions and store to keep track of session information
    let session_layer =
        SessionLayer::new(MemoryStore::new(), secret.as_bytes()).with_cookie_name(session_cookie);

    // combine the front and backend into server
    let app = Router::new()
        .merge(services::front_public_route())
        .merge(services::backend(session_layer, shared_state));

    tracing::info!("🚀 listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}

// Variables from Environment or default to configure server
// port, host, secret
fn from_env() -> (String, String, String, String, String) {
    if env::var("SERVER_SECRET").is_err() {
        warn!("env var SERVER_SECRET should be set and unique (64 bytes long)");
    }
    if env::var("API_TOKEN").is_err() {
        warn!("env var API_TOKEN should be set and unique (64 bytes long)");
    }
    (
        env::var("SERVER_PORT").ok().unwrap(),
        env::var("SERVER_HOST").ok().unwrap(),
        env::var("SERVER_SECRET").ok().unwrap_or_else(|| {
            "this needs to be 64bytes. recommended that you set Secret instead of fixed value"
                .into()
        }),
        env::var("RUST_LOG").unwrap_or_else(|_| "svelte_axum_project=debug".into()),
        env::var("API_TOKEN").ok().unwrap_or_else(|| {
            "this needs to be 64bytes. recommended that you set Secret instead of fixed value"
                .into()
        }),
    )
}
