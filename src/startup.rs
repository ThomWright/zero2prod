use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use std::net::{SocketAddr, TcpListener};

use crate::configuration::Settings;
use crate::db;
use crate::routes::{health_check, subscribe};

pub async fn run(configuration: Settings) -> std::io::Result<(Server, SocketAddr)> {
    db::run_migrations(&configuration).await;

    let listener = TcpListener::bind(format!("127.0.0.1:{}", configuration.application_port))?;
    let socket_addr = listener.local_addr()?;

    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok((server, socket_addr))
}
