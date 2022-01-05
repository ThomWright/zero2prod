use crate::globals::{get_global_server_address, test_global_rt};

#[test]
fn health_check_works() {
    test_global_rt(async {
        // Arrange
        let addr = get_global_server_address().await;

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
    })
}
