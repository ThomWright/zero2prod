use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tokio::{runtime::Runtime, sync::OnceCell};
use zero2prod::configuration::{DatabaseSettings, Settings};

// TODO: Could we run the global server in another thread, which uses a
//       separate runtime to the one the tests use?

pub fn test_global_rt<F: std::future::Future>(f: F) -> F::Output {
    static RT: Lazy<Runtime> = Lazy::new(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed building the Runtime")
    });
    RT.block_on(f)
}

pub async fn get_global_server_address() -> SocketAddr {
    static ADDR: OnceCell<SocketAddr> = OnceCell::const_new();
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
