mod blog;
mod cli;
mod config;
mod db;
mod error;
mod service;
mod utils;

use std::process;

use axum::Router;
use blog::ApiDoc;
use db::{migrations::run_migrations, DbPool};
use error::BlogError;
use tracing::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{config::CONFIG, db::create_pool, utils::SHUTDOWN};

#[derive(Clone)]
pub struct AppState {
    pool: DbPool,
}

#[tokio::main]
async fn main() -> Result<(), BlogError> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(CONFIG.log_level())
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap_or_else(|e| {
        error!("{}", e);
        process::exit(1);
    });

    run_migrations()?;

    info!("starting Blog...");
    let pool = create_pool().await?;
    let appstate = AppState { pool };

    let app = Router::new()
        .nest("/api", blog::router(appstate))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    let listener = tokio::net::TcpListener::bind(&CONFIG.listener_host()).await?;

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(SHUTDOWN.wait_for_shutdown())
        .await?;

    Ok(())
}
