use std::time::Duration;
use webull_rs::streaming::events::EventType;
use webull_rs::streaming::subscription::SubscriptionRequest;
use webull_rs::{WebullClient, WebullError};

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

    // Create a WebSocket client
    println!("Creating WebSocket client...");
    let mut ws_client = client.streaming();

    // Connect to the WebSocket server
    println!("Connecting to WebSocket server...");
    let mut event_receiver = match ws_client.connect().await {
        Ok(receiver) => {
            println!("Connected to WebSocket server");
            receiver
        }
        Err(e) => {
            println!("Failed to connect to WebSocket server: {}", e);
            return Err(e.into());
        }
    };

    // Subscribe to quotes for AAPL and MSFT
    println!("Subscribing to quotes...");
    let subscription = SubscriptionRequest::new_quote(vec!["AAPL".to_string(), "MSFT".to_string()]);
    match ws_client.subscribe(subscription).await {
        Ok(_) => {
            println!("Subscribed to quotes");
        }
        Err(e) => {
            println!("Failed to subscribe to quotes: {}", e);
            return Err(e.into());
        }
    }

    // Handle events for 60 seconds
    println!("Handling events for 60 seconds...");
    let start_time = std::time::Instant::now();

    while start_time.elapsed() < Duration::from_secs(60) {
        match tokio::time::timeout(Duration::from_secs(1), event_receiver.recv()).await {
            Ok(Some(event)) => match event.event_type {
                EventType::Quote => {
                    println!("Received quote update: {:?}", event);
                }
                EventType::Connection => {
                    println!("Connection event: {:?}", event);
                }
                EventType::Error => {
                    println!("Error event: {:?}", event);
                }
                _ => {
                    println!("Other event: {:?}", event);
                }
            },
            Ok(None) => {
                // Channel closed
                println!("Event channel closed");
                break;
            }
            Err(_) => {
                // Timeout, continue
            }
        }
    }

    // Disconnect from the WebSocket server
    println!("Disconnecting from WebSocket server...");
    match ws_client.disconnect().await {
        Ok(_) => {
            println!("Disconnected from WebSocket server");
        }
        Err(e) => {
            println!("Failed to disconnect from WebSocket server: {}", e);
            return Err(e.into());
        }
    }

    // Logout from Webull
    println!("Logging out...");
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
