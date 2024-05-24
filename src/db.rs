pub mod models;

use std::time::Duration;

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use bb8::{Pool, PooledConnection};
use diesel::{ConnectionError, ConnectionResult};
use diesel_async::{
    pooled_connection::{AsyncDieselConnectionManager, ManagerConfig},
    AsyncPgConnection,
};
use futures_util::future::BoxFuture;
use futures_util::FutureExt;

use crate::{config::CONFIG, error::BlogError};

type DbPool = Pool<AsyncDieselConnectionManager<AsyncPgConnection>>;

pub async fn create_pool() -> Result<DbPool, BlogError> {
    let mut config = ManagerConfig::default();
    if CONFIG.db.ssl {
        config.custom_setup = Box::new(establish_connection);
    }
    let mgr = AsyncDieselConnectionManager::<AsyncPgConnection>::new_with_config(
        &CONFIG.db_url(),
        config,
    );

    let pool = Pool::builder()
        .max_size(15)
        .min_idle(Some(5))
        .max_lifetime(Some(Duration::from_secs(60 * 60 * 24)))
        .idle_timeout(Some(Duration::from_secs(60 * 2)))
        .build(mgr)
        .await?;
    Ok(pool)
}

struct DbConn(PooledConnection<'static, AsyncDieselConnectionManager<AsyncPgConnection>>);

#[async_trait]
impl<S> FromRequestParts<S> for DbConn
where
    S: Send + Sync,
    DbPool: FromRef<S>,
{
    type Rejection = crate::BlogError;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let pool = Pool::from_ref(state);

        let conn = pool
            .get_owned()
            .await
            .map_err(|_| BlogError::InternalServerError)?;

        Ok(Self(conn))
    }
}

fn establish_connection(config: &str) -> BoxFuture<ConnectionResult<AsyncPgConnection>> {
    let fut = async {
        let rustls_config = rustls::ClientConfig::builder()
            .with_root_certificates(root_certs())
            .with_no_client_auth();
        let tls = tokio_postgres_rustls::MakeRustlsConnect::new(rustls_config);
        let (client, conn) = tokio_postgres::connect(config, tls)
            .await
            .map_err(|e| ConnectionError::BadConnection(e.to_string()))?;
        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Database connection: {e}");
            }
        });
        AsyncPgConnection::try_from(client).await
    };
    fut.boxed()
}

fn root_certs() -> rustls::RootCertStore {
    let mut roots = rustls::RootCertStore::empty();
    let certs = rustls_native_certs::load_native_certs().expect("Certs not loadable!");
    roots.add_parsable_certificates(certs);
    roots
}

pub mod migrations {
    use crate::{config::CONFIG, error::BlogError};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use tracing::info;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    pub fn run_migrations() -> Result<(), BlogError> {
        use diesel::Connection;
        let mut connection = diesel::pg::PgConnection::establish(&CONFIG.db_url())
            .map_err(|e| BlogError::MigrationError(format!("Failed to get connection: {}", e)))?;

        info!("migrations...");
        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|e| BlogError::MigrationError(format!("Error running migrations: {}", e)))?;
        info!("migrations done!");

        Ok(())
    }
}
