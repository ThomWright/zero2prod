use std::net::{SocketAddr, TcpListener};

#[actix_rt::test]
async fn health_check_works() {
    // Arrange
    let addr = spawn_app();

    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("http://{}/health_check", addr))
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let addr = listener.local_addr().unwrap();

    let server = zero2prod::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    addr
}
