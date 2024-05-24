mod blog;
mod config;
mod db;
mod error;
mod notify;
mod utils;

use axum::{routing::get, Router};
use db::migrations::run_migrations;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;
use utils::shutdown_signal;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    run_migrations().unwrap();

    info!("Starting Blog...");
    let app = Router::new().route("/", get(|| async { "hello world" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
