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
    
    // Since login is not yet implemented, we expect an error
    assert!(result.is_err());
    match result {
        Err(WebullError::InvalidRequest(msg)) => {
            assert!(msg.contains("not yet implemented"));
        }
        _ => panic!("Expected InvalidRequest error"),
    }
}
