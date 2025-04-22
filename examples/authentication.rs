use std::time::Duration;
use webull_rs::WebullClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let _client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;

    // Note: The login functionality is not yet implemented
    // This is just a placeholder to demonstrate the intended API
    println!("Client created successfully");

    // In a real implementation, you would do:
    // client.login("username", "password").await?;
    // println!("Logged in successfully");
    // client.logout().await?;
    // println!("Logged out successfully");

    Ok(())
}
