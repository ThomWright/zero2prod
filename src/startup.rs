use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};

use crate::configuration::Settings;
use crate::db;
use crate::routes::{health_check, subscribe};

pub async fn run(configuration: Settings) -> std::io::Result<(Server, SocketAddr)> {
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    db::run_migrations(&connection_pool).await;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    let socket_addr = listener.local_addr()?;

    let pool = web::Data::new(connection_pool);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    log::info!("Server running");

    Ok((server, socket_addr))
}
