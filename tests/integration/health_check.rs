// use crate::globals::{get_global_server_address, test_global_rt};
use crate::globals::APP;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let addr = &*APP;

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
