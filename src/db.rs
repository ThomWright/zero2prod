use sqlx::{migrate, Connection, PgConnection};

use crate::configuration::Settings;

pub async fn run_migrations(configuration: &Settings) {
    let mut connection = PgConnection::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    migrate!()
        .run(&mut connection)
        .await
        .expect("Failed to run migrations");
}
