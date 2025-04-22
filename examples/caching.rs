use webull_rs::{WebullClient, WebullError};
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;
    
    // Login to Webull
    println!("Logging in...");
    match client.login("username", "password").await {
        Ok(_) => {
            println!("Logged in successfully");
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
            // Continue anyway for demonstration purposes
        }
        Err(e) => {
            return Err(e.into());
        }
    }
    
    // Make a request to get account information
    println!("\nGetting account information (first request)...");
    let start = Instant::now();
    match client.accounts().get_accounts().await {
        Ok(accounts) => {
            let elapsed = start.elapsed();
            println!("Found {} accounts in {:?}", accounts.len(), elapsed);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Make the same request again (should be cached)
    println!("\nGetting account information (second request, should be cached)...");
    let start = Instant::now();
    match client.accounts().get_accounts().await {
        Ok(accounts) => {
            let elapsed = start.elapsed();
            println!("Found {} accounts in {:?} (should be faster)", accounts.len(), elapsed);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Make a request to get a quote
    println!("\nGetting quote for AAPL (first request)...");
    let start = Instant::now();
    match client.market_data().get_quote("AAPL").await {
        Ok(quote) => {
            let elapsed = start.elapsed();
            println!("Got quote for AAPL in {:?}", elapsed);
            println!("  Last price: ${}", quote.last_price);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Make the same request again (should be cached)
    println!("\nGetting quote for AAPL (second request, should be cached)...");
    let start = Instant::now();
    match client.market_data().get_quote("AAPL").await {
        Ok(quote) => {
            let elapsed = start.elapsed();
            println!("Got quote for AAPL in {:?} (should be faster)", elapsed);
            println!("  Last price: ${}", quote.last_price);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Logout from Webull
    println!("\nLogging out...");
    match client.logout().await {
        Ok(_) => {
            println!("Logged out successfully");
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
