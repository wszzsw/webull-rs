use webull_rs::{WebullClient, WebullError};

#[tokio::test]
async fn test_client_creation() {
    let client = WebullClient::builder().build();
    assert!(client.is_ok());
}

#[tokio::test]
async fn test_login_placeholder() {
    let client = WebullClient::builder().build().unwrap();
    let result = client.login("test", "test").await;

    // Since we're using invalid credentials, we expect an error
    assert!(result.is_err());
    match result {
        Err(WebullError::NetworkError(_)) => {
            // This is expected since we're not actually connecting to a real server
        }
        Err(WebullError::InvalidRequest(_)) => {
            // This is also acceptable if the API is not fully implemented
        }
        Err(WebullError::Unauthorized) => {
            // This is also acceptable if the auth flow is implemented but credentials are invalid
        }
        Err(WebullError::ApiError { .. }) => {
            // This is also acceptable if the API returns an error response
        }
        err => panic!("Unexpected result: {:?}", err),
    }
}
