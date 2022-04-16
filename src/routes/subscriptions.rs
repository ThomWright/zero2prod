use crate::db::subscriptions;
use crate::subscriber::Subscriber;
use actix_web::{web, HttpResponse, Responder};
use sqlx::{self, PgPool};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name= %form.name
    )
)]
#[allow(clippy::async_yields_async)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    let subscriber = Subscriber {
        email: form.email.clone(),
        name: form.name.clone(),
    };
    let mut conn = pool.acquire().await.unwrap();
    let res = match subscriptions::insert_subscriber(&mut conn, &subscriber).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    };
    res
}
