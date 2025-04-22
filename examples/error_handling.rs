use std::time::Duration;
use webull_rs::models::order::{OrderRequest, OrderSide, TimeInForce};
use webull_rs::{WebullClient, WebullError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;

    // Example 1: Handle authentication errors
    println!("Example 1: Handle authentication errors");
    match client.login("invalid-username", "invalid-password").await {
        Ok(_) => {
            println!("Login successful (unexpected)");
        }
        Err(WebullError::Unauthorized) => {
            println!("✓ Correctly handled unauthorized error: Invalid credentials");
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Example 2: Handle network errors
    println!("\nExample 2: Handle network errors");
    let bad_client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_custom_url("https://invalid-url.example.com")
        .with_timeout(Duration::from_secs(5))
        .build()?;

    match bad_client.login("username", "password").await {
        Ok(_) => {
            println!("Login successful (unexpected)");
        }
        Err(WebullError::NetworkError(e)) => {
            println!("✓ Correctly handled network error: {}", e);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Example 3: Handle API errors
    println!("\nExample 3: Handle API errors");
    // Try to place an invalid order (negative quantity)
    let invalid_order = OrderRequest::market()
        .symbol("AAPL")
        .quantity(-1.0)
        .side(OrderSide::Buy)
        .time_in_force(TimeInForce::Day);

    match client.orders().place_order(&invalid_order).await {
        Ok(_) => {
            println!("Order placed successfully (unexpected)");
        }
        Err(WebullError::ApiError { code, message }) => {
            println!("✓ Correctly handled API error: {} - {}", code, message);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Example 4: Handle rate limit errors
    println!("\nExample 4: Handle rate limit errors");
    // Make multiple requests in quick succession to trigger rate limiting
    for i in 1..=5 {
        println!("Request {}/5...", i);
        match client.market_data().get_quote("AAPL").await {
            Ok(_) => {
                println!("  Quote retrieved successfully");
            }
            Err(WebullError::RateLimitExceeded) => {
                println!("✓ Correctly handled rate limit error");
                // Implement exponential backoff
                let backoff_seconds = 2_u64.pow(i);
                println!("  Backing off for {} seconds", backoff_seconds);
                tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
            }
            Err(WebullError::InvalidRequest(msg)) => {
                println!("  API not yet implemented: {}", msg);
            }
            Err(e) => {
                println!("  Other error: {}", e);
            }
        }
    }

    // Example 5: Handle validation errors
    println!("\nExample 5: Handle validation errors");
    // Try to place an order with an invalid symbol
    let invalid_symbol_order = OrderRequest::market()
        .symbol("")
        .quantity(1.0)
        .side(OrderSide::Buy)
        .time_in_force(TimeInForce::Day);

    match client.orders().place_order(&invalid_symbol_order).await {
        Ok(_) => {
            println!("Order placed successfully (unexpected)");
        }
        Err(WebullError::SerializationError(msg)) => {
            println!("✓ Correctly handled serialization error: {}", msg);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Example 6: Handle token expiration and refresh
    println!("\nExample 6: Handle token expiration and refresh");
    // Simulate a token expiration by making a request after the token has expired
    match client.market_data().get_quote("AAPL").await {
        Ok(_) => {
            println!("Quote retrieved successfully");
        }
        Err(WebullError::Unauthorized) => {
            println!("Token expired, refreshing...");
            match client.refresh_token().await {
                Ok(_) => {
                    println!("✓ Token refreshed successfully");
                    // Try the request again
                    match client.market_data().get_quote("AAPL").await {
                        Ok(_) => {
                            println!("✓ Quote retrieved successfully after token refresh");
                        }
                        Err(e) => {
                            println!("Error after token refresh: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error refreshing token: {}", e);
                }
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }

    // Example 7: Comprehensive error handling with a custom function
    println!("\nExample 7: Comprehensive error handling with a custom function");

    async fn get_quote_with_retry(
        client: &WebullClient,
        symbol: &str,
        max_retries: u32,
    ) -> Result<(), WebullError> {
        let mut retries = 0;
        loop {
            match client.market_data().get_quote(symbol).await {
                Ok(_) => {
                    println!("Quote retrieved successfully");
                    return Ok(());
                }
                Err(WebullError::Unauthorized) => {
                    println!("Token expired, refreshing...");
                    client.refresh_token().await?;
                    println!("Token refreshed, retrying...");
                }
                Err(WebullError::RateLimitExceeded) => {
                    retries += 1;
                    if retries > max_retries {
                        return Err(WebullError::RateLimitExceeded);
                    }
                    let backoff_seconds = 2_u64.pow(retries);
                    println!(
                        "Rate limit exceeded, backing off for {} seconds",
                        backoff_seconds
                    );
                    tokio::time::sleep(Duration::from_secs(backoff_seconds)).await;
                }
                Err(WebullError::NetworkError(e)) => {
                    retries += 1;
                    if retries > max_retries {
                        return Err(WebullError::NetworkError(e));
                    }
                    println!("Network error: {}, retrying in 2 seconds", e);
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    match get_quote_with_retry(&client, "AAPL", 3).await {
        Ok(_) => {
            println!("✓ Successfully retrieved quote with retry logic");
        }
        Err(e) => {
            println!("Failed to retrieve quote after retries: {}", e);
        }
    }

    Ok(())
}
