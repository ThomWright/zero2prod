use sqlx::migrate;
use sqlx::{migrate::Migrate, Acquire};
use std::ops::Deref;

pub async fn run_migrations<'a, A>(migrator: A)
where
    A: Acquire<'a>,
    <<A as Acquire<'a>>::Connection as Deref>::Target: Migrate,
{
    migrate!()
        .run(migrator)
        .await
        .expect("Failed to run migrations");

    println!("Finished running migrations");
}
