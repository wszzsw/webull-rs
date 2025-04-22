use webull_rs::{WebullClient, WebullError};
use webull_rs::models::order::{OrderRequest, OrderSide, OrderType, TimeInForce};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;
    
    // Note: The API calls are not yet implemented with actual API integration
    // This is just a demonstration of how the API would be used
    
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
    
    // Create a market order to buy 1 share of AAPL
    let market_order = OrderRequest::new()
        .symbol("AAPL")
        .quantity(1.0)
        .side(OrderSide::Buy)
        .order_type(OrderType::Market)
        .time_in_force(TimeInForce::Day);
    
    println!("\nPlacing market order for AAPL...");
    match client.orders().place_order(&market_order).await {
        Ok(response) => {
            println!("Order placed successfully:");
            println!("  Order ID: {}", response.id);
            println!("  Status: {:?}", response.status);
            println!("  Symbol: {}", response.symbol);
            println!("  Quantity: {}", response.quantity);
            
            // Get the order details
            println!("\nGetting order details...");
            match client.orders().get_order(&response.id).await {
                Ok(order) => {
                    println!("Order details:");
                    println!("  Order ID: {}", order.id);
                    println!("  Status: {:?}", order.status);
                    println!("  Symbol: {}", order.symbol);
                    println!("  Quantity: {}", order.quantity);
                    println!("  Filled Quantity: {}", order.filled_quantity);
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented: {}", msg);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
            
            // Cancel the order
            println!("\nCanceling order...");
            match client.orders().cancel_order(&response.id).await {
                Ok(_) => {
                    println!("Order canceled successfully");
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented: {}", msg);
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Create a limit order to buy 1 share of AAPL at $150
    let limit_order = OrderRequest::new()
        .symbol("AAPL")
        .quantity(1.0)
        .price(150.0)
        .side(OrderSide::Buy)
        .order_type(OrderType::Limit)
        .time_in_force(TimeInForce::Gtc);
    
    println!("\nPlacing limit order for AAPL...");
    match client.orders().place_order(&limit_order).await {
        Ok(response) => {
            println!("Order placed successfully:");
            println!("  Order ID: {}", response.id);
            println!("  Status: {:?}", response.status);
            println!("  Symbol: {}", response.symbol);
            println!("  Quantity: {}", response.quantity);
            println!("  Price: ${}", response.price.unwrap_or(0.0));
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    // Get active orders
    println!("\nGetting active orders...");
    match client.orders().get_active_orders().await {
        Ok(orders) => {
            println!("Active orders: {}", orders.len());
            for order in orders {
                println!("  Order ID: {}", order.id);
                println!("  Status: {:?}", order.status);
                println!("  Symbol: {}", order.symbol);
                println!("  Quantity: {}", order.quantity);
                println!("  Filled Quantity: {}", order.filled_quantity);
                println!();
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    println!("\nLogging out...");
    match client.logout().await {
        Ok(_) => {
            println!("Logged out successfully");
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
    
    Ok(())
}
