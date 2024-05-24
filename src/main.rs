mod blog;
mod config;
mod db;
mod error;
mod notify;
mod utils;

use std::process;

use axum::{routing::get, Router};
use db::migrations::run_migrations;
use error::BlogError;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::{config::CONFIG, db::create_pool, utils::SHUTDOWN};

#[tokio::main]
async fn main() -> Result<(), BlogError> {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(CONFIG.log_level())
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    if let Err(e) = run_migrations() {
        eprintln!("{}", e);
        process::exit(1);
    }

    info!("Starting Blog...");
    let pool = create_pool().await?;
    let app = Router::new()
        .with_state(pool)
        .route("/", get(|| async { "hello world" }));
    let listener = tokio::net::TcpListener::bind(&CONFIG.listener_host()).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(SHUTDOWN.wait_for_shutdown())
        .await?;

    Ok(())
}
