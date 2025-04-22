use std::time::Duration;
use webull_rs::models::order::{
    OptionOrderRequest, OrderRequest, OrderSide, OrderType, TimeInForce, TrailingStopType,
};
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

    // Get accounts
    println!("\nGetting accounts...");
    match client.accounts().get_accounts().await {
        Ok(accounts) => {
            println!("Found {} accounts", accounts.len());

            if let Some(account) = accounts.first() {
                let account_id = &account.id;
                println!("Using account ID: {}", account_id);

                // Place a market order
                println!("\nPlacing a market order...");
                let market_order = OrderRequest::market()
                    .symbol("AAPL")
                    .quantity(1.0)
                    .side(OrderSide::Buy)
                    .time_in_force(TimeInForce::Day)
                    .client_order_id("market-order-1");

                match client.orders().place_order(&market_order).await {
                    Ok(response) => {
                        println!("Market order placed successfully");
                        println!("Order ID: {}", response.id);
                        println!("Status: {:?}", response.status);

                        // Get order details
                        println!("\nGetting order details...");
                        match client.orders().get_order(&response.id).await {
                            Ok(order) => {
                                println!("Order details:");
                                println!("  Symbol: {}", order.symbol);
                                println!("  Quantity: {}", order.quantity);
                                println!("  Side: {:?}", order.side);
                                println!("  Type: {:?}", order.order_type);
                                println!("  Status: {:?}", order.status);
                            }
                            Err(WebullError::InvalidRequest(msg)) => {
                                println!("API not yet implemented: {}", msg);
                            }
                            Err(e) => {
                                println!("Error: {}", e);
                            }
                        }

                        // Cancel the order
                        println!("\nCanceling the order...");
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

                // Place a limit order
                println!("\nPlacing a limit order...");
                let limit_order = OrderRequest::limit()
                    .symbol("MSFT")
                    .quantity(1.0)
                    .price(300.0)
                    .side(OrderSide::Buy)
                    .time_in_force(TimeInForce::Day)
                    .client_order_id("limit-order-1");

                match client.orders().place_order(&limit_order).await {
                    Ok(response) => {
                        println!("Limit order placed successfully");
                        println!("Order ID: {}", response.id);
                        println!("Status: {:?}", response.status);

                        // Modify the order
                        println!("\nModifying the order...");
                        let modified_order = OrderRequest::limit()
                            .symbol("MSFT")
                            .quantity(2.0)
                            .price(305.0)
                            .side(OrderSide::Buy)
                            .time_in_force(TimeInForce::Day)
                            .client_order_id("limit-order-1-modified");

                        match client
                            .orders()
                            .modify_order(&response.id, &modified_order)
                            .await
                        {
                            Ok(modified_response) => {
                                println!("Order modified successfully");
                                println!("New Order ID: {}", modified_response.id);
                                println!("Status: {:?}", modified_response.status);
                                println!("New Quantity: {}", modified_response.quantity);
                                println!("New Price: ${}", modified_response.price.unwrap_or(0.0));
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

                // Place a stop order
                println!("\nPlacing a stop order...");
                let stop_order = OrderRequest::stop()
                    .symbol("GOOG")
                    .quantity(1.0)
                    .stop_price(2500.0)
                    .side(OrderSide::Sell)
                    .time_in_force(TimeInForce::Day)
                    .client_order_id("stop-order-1");

                match client.orders().place_order(&stop_order).await {
                    Ok(response) => {
                        println!("Stop order placed successfully");
                        println!("Order ID: {}", response.id);
                        println!("Status: {:?}", response.status);
                    }
                    Err(WebullError::InvalidRequest(msg)) => {
                        println!("API not yet implemented: {}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                // Place a trailing stop order
                println!("\nPlacing a trailing stop order...");
                let trailing_stop_order = OrderRequest::trailing_stop()
                    .symbol("AMZN")
                    .quantity(1.0)
                    .side(OrderSide::Sell)
                    .time_in_force(TimeInForce::Day)
                    .trailing_type(TrailingStopType::Percent)
                    .trailing_stop_step(5.0)
                    .client_order_id("trailing-stop-order-1");

                match client.orders().place_order(&trailing_stop_order).await {
                    Ok(response) => {
                        println!("Trailing stop order placed successfully");
                        println!("Order ID: {}", response.id);
                        println!("Status: {:?}", response.status);
                    }
                    Err(WebullError::InvalidRequest(msg)) => {
                        println!("API not yet implemented: {}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                // Get open orders
                println!("\nGetting open orders...");
                match client.orders().get_open_orders(account_id).await {
                    Ok(orders) => {
                        println!("Found {} open orders", orders.len());
                        for order in orders {
                            println!("  Order ID: {}", order.id);
                            println!("  Symbol: {}", order.symbol);
                            println!("  Quantity: {}", order.quantity);
                            println!("  Side: {:?}", order.side);
                            println!("  Type: {:?}", order.order_type);
                            println!("  Status: {:?}", order.status);
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

                // Get today's orders
                println!("\nGetting today's orders...");
                match client.orders().get_today_orders(account_id).await {
                    Ok(orders) => {
                        println!("Found {} orders today", orders.len());
                        for order in orders {
                            println!("  Order ID: {}", order.id);
                            println!("  Symbol: {}", order.symbol);
                            println!("  Quantity: {}", order.quantity);
                            println!("  Side: {:?}", order.side);
                            println!("  Type: {:?}", order.order_type);
                            println!("  Status: {:?}", order.status);
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

                // Place an option order (preview first)
                println!("\nPreviewing an option order...");
                let option_order = OptionOrderRequest::new("option-order-1", "123456789", 1.0)
                    .side(OrderSide::Buy)
                    .order_type(OrderType::Limit)
                    .price(5.0)
                    .time_in_force(TimeInForce::Day);

                let preview_request =
                    webull_rs::models::order::OptionOrderPreviewRequest::new(account_id)
                        .add_order(option_order.clone());

                match client.orders().preview_option_order(&preview_request).await {
                    Ok(preview) => {
                        println!("Option order preview successful");
                        println!("Commission: ${}", preview.commission);
                        println!("Estimated Cost: ${}", preview.estimated_cost);
                        println!("Buying Power Effect: ${}", preview.buying_power_effect);

                        // Place the option order
                        println!("\nPlacing the option order...");
                        match client
                            .orders()
                            .place_option_order(account_id, &[option_order])
                            .await
                        {
                            Ok(responses) => {
                                println!("Option order placed successfully");
                                for response in responses {
                                    println!("  Order ID: {}", response.id);
                                    println!("  Status: {:?}", response.status);
                                }
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
            }
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
