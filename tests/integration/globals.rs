use once_cell::sync::Lazy;
use retry::{delay::Exponential, retry};
use secrecy::Secret;
use std::net::{SocketAddr, TcpListener, TcpStream};
use zero2prod::{
    configuration::{ApplicationSettings, DatabaseSettings, Settings},
    telemetry,
};

pub static APP: Lazy<SocketAddr> = Lazy::new(|| {
    let socket_addr = TcpListener::bind(("127.0.0.1", 0))
        .unwrap()
        .local_addr()
        .unwrap();

    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to build the Runtime");

        rt.block_on(async { spawn_app(socket_addr).await })
    });

    retry(
        Exponential::from_millis_with_factor(150, 1.5).take(10),
        || TcpStream::connect(socket_addr),
    )
    .expect("Could not connect to APP");

    socket_addr
});

async fn spawn_app(socket_addr: SocketAddr) {
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
            host: socket_addr.ip().to_string(),
            port: socket_addr.port(),
        },
        database: DatabaseSettings {
            host: "127.0.0.1".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: Secret::new("password".into()),
            database_name: "newsletter".to_string(),
        },
    };

    let server = zero2prod::init(configuration)
        .await
        .expect("Failed to init server");

    server.await.unwrap();
}
