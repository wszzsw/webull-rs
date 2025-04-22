use webull_rs::{WebullClient, WebullError};
use webull_rs::utils::credentials::EncryptedCredentialStore;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a credential store
    let credential_store = EncryptedCredentialStore::new(
        "credentials.json".to_string(),
        "token.json".to_string(),
        "my-secret-key".to_string(),
    );
    
    // Create a client with the credential store
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .with_credential_store(credential_store)
        .build()?;
    
    // Login to Webull
    println!("Logging in...");
    match client.login("username", "password").await {
        Ok(_) => {
            println!("Logged in successfully");
            
            // Get the stored credentials
            match client.get_credentials()? {
                Some(credentials) => {
                    println!("Stored credentials:");
                    println!("  Username: {}", credentials.username);
                    println!("  Password: {}", "*".repeat(credentials.password.len()));
                }
                None => {
                    println!("No credentials stored");
                }
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
            // Continue anyway for demonstration purposes
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    
    // Logout from Webull
    println!("\nLogging out...");
    match client.logout().await {
        Ok(_) => {
            println!("Logged out successfully");
            
            // Check if credentials were cleared
            match client.get_credentials()? {
                Some(_) => {
                    println!("Credentials still stored (unexpected)");
                }
                None => {
                    println!("Credentials cleared successfully");
                }
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    
    Ok(())
}
