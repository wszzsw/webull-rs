use std::time::Duration;
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
                println!("  Account ID: {}", account.id);
                println!("  Account Number: {}", account.account_number);
                println!("  Account Type: {:?}", account.account_type);
                println!("  Status: {:?}", account.status);
                println!("  Currency: {}", account.currency);
                println!("  Paper Trading: {}", account.paper_trading);

                // Get account profile
                println!("\nGetting account profile...");
                match client.accounts().get_account_profile(&account.id).await {
                    Ok(profile) => {
                        println!("  Name: {}", profile.name);
                        println!("  Region: {}", profile.region);
                        if let Some(email) = &profile.email {
                            println!("  Email: {}", email);
                        }
                        if let Some(phone) = &profile.phone {
                            println!("  Phone: {}", phone);
                        }
                    }
                    Err(WebullError::InvalidRequest(msg)) => {
                        println!("API not yet implemented: {}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                // Get account balance
                println!("\nGetting account balance...");
                match client.accounts().get_account_balance(&account.id).await {
                    Ok(balance) => {
                        println!("  Cash: ${}", balance.cash);
                        println!("  Buying Power: ${}", balance.buying_power);
                        println!("  Market Value: ${}", balance.market_value);
                        println!("  Total Value: ${}", balance.total_value);
                        println!(
                            "  Unrealized P/L: ${} ({}%)",
                            balance.unrealized_profit_loss,
                            balance.unrealized_profit_loss_percentage
                        );
                    }
                    Err(WebullError::InvalidRequest(msg)) => {
                        println!("API not yet implemented: {}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                // Get account positions
                println!("\nGetting account positions...");
                match client.accounts().get_positions(&account.id).await {
                    Ok(positions) => {
                        println!("Found {} positions", positions.len());
                        for position in positions {
                            println!("  Symbol: {}", position.symbol);
                            println!("  Quantity: {}", position.quantity);
                            println!("  Cost Basis: ${}", position.cost_basis);
                            println!("  Market Value: ${}", position.market_value);
                            println!(
                                "  Unrealized P/L: ${} ({}%)",
                                position.unrealized_profit_loss,
                                position.unrealized_profit_loss_percentage
                            );
                            println!("  Current Price: ${}", position.current_price);
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

                // Get trade history
                println!("\nGetting trade history...");
                match client.accounts().get_trade_history(&account.id).await {
                    Ok(trades) => {
                        println!("Found {} trades", trades.len());
                        for trade in trades {
                            println!("  ID: {}", trade.id);
                            println!("  Symbol: {}", trade.symbol);
                            println!("  Action: {}", trade.action);
                            println!("  Quantity: {}", trade.quantity);
                            println!("  Price: ${}", trade.price);
                            println!("  Amount: ${}", trade.amount);
                            println!("  Trade Time: {}", trade.trade_time);
                            println!("  Status: {}", trade.status);
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
