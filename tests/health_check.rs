use init::init_global_server;

mod init;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = init_global_server();

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
