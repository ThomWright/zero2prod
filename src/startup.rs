use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::db::{self, create_connection_pool};
use crate::routes::{health_check, subscribe};

pub async fn init(configuration: Settings) -> std::io::Result<Server> {
    let connection_pool = create_connection_pool(&configuration);

    db::run_migrations(&connection_pool).await;

    let listener = TcpListener::bind(configuration.application)?;

    let pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    tracing::info!("Server running");

    Ok(server)
}
