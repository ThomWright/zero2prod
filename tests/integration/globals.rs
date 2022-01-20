use once_cell::sync::Lazy;
use secrecy::Secret;
use std::net::SocketAddr;
use tokio::{runtime::Runtime, sync::OnceCell};
use zero2prod::{
    configuration::{ApplicationSettings, DatabaseSettings, Settings},
    telemetry,
};

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
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        telemetry::init_subscriber(subscriber);
    } else {
        let subscriber =
            telemetry::get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        telemetry::init_subscriber(subscriber);
    };

    let configuration = Settings {
        application: ApplicationSettings {
            host: "localhost".into(),
            port: 0,
        },
        database: DatabaseSettings {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: Secret::new("password".into()),
            database_name: "newsletter".to_string(),
        },
    };

    let (server, addr) = zero2prod::run(configuration)
        .await
        .expect("Failed to start server");

    let _ = tokio::spawn(server);

    addr
}
