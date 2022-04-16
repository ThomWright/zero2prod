use crate::subscriber::Subscriber;
use chrono::Utc;
use sqlx::{self, PgConnection};
use uuid::Uuid;

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(conn, subscriber)
)]
pub async fn insert_subscriber(
    conn: &mut PgConnection,
    subscriber: &Subscriber,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        subscriber.email,
        subscriber.name,
        Utc::now()
    )
    .execute(conn)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
