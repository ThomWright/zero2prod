use fake::{
    faker::{internet::en::SafeEmail, name::en::Name},
    Fake,
};
use init::init_global_server;
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

mod init;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    // Arrange
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();

    let addr = init_global_server();
    let client = reqwest::Client::new();
    let body = format!(
        "name={}&email={}",
        utf8_percent_encode(&name, NON_ALPHANUMERIC),
        utf8_percent_encode(&email, NON_ALPHANUMERIC)
    );

    // Act
    let response = client
        .post(&format!("http://{}/subscriptions", &addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    // Arrange
    let name: String = Name().fake();
    let email: String = SafeEmail().fake();

    let addr = init_global_server();
    let client = reqwest::Client::new();
    let test_cases = vec![
        (
            format!("name={}", utf8_percent_encode(&name, NON_ALPHANUMERIC),),
            "missing the email",
        ),
        (
            format!("email={}", utf8_percent_encode(&email, NON_ALPHANUMERIC)),
            "missing the name",
        ),
        ("".into(), "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("http://{}/subscriptions", &addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}
