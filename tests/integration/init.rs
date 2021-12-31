use std::net::SocketAddr;
use tokio::sync::OnceCell;
use zero2prod::configuration::{DatabaseSettings, Settings};

static ADDR: OnceCell<SocketAddr> = OnceCell::const_new();

pub async fn init_global_server() -> SocketAddr {
    ADDR.get_or_init(|| spawn_app()).await.clone()
}

async fn spawn_app() -> SocketAddr {
    let configuration = Settings {
        application_port: 0,
        database: DatabaseSettings {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: "password".to_string(),
            database_name: "newsletter".to_string(),
        },
    };

    let (server, addr) = zero2prod::run(configuration)
        .await
        .expect("Failed to start server");

    let _ = tokio::spawn(server);

    addr
}
