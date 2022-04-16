use secrecy::ExposeSecret;
use sqlx::migrate;
use sqlx::postgres::PgPoolOptions;
use sqlx::{migrate::Migrate, Acquire, Pool, Postgres};
use std::ops::Deref;

use crate::configuration::Settings;

pub fn create_connection_pool(configuration: &Settings) -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy(configuration.database.connection_string().expose_secret())
        .expect("Failed to connect to Postgres")
}

pub async fn run_migrations<'a, A>(migrator: A)
where
    A: Acquire<'a>,
    <<A as Acquire<'a>>::Connection as Deref>::Target: Migrate,
{
    migrate!()
        .run(migrator)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database migrations applied");
}
