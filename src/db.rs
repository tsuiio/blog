pub mod models;

pub mod migrations {
    use crate::{config::Config, error::BlogError};
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    use tracing::info;

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

    pub fn run_migrations() -> Result<(), BlogError> {
        use diesel::Connection;
        let url = "postgres://blog:blog@127.0.0.1/blog";
        let mut connection = diesel::pg::PgConnection::establish(&url)?;
        info!("migrations...");
        connection
            .run_pending_migrations(MIGRATIONS)
            .expect("Error running migrations");
        info!("migrations done!");

        Ok(())
    }
}
