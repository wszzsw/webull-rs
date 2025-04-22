use std::time::Duration;
use webull_rs::{WebullClient, WebullError};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client for live trading
    let live_client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;

    println!("Live trading client created");
    println!("Is paper trading: {}", live_client.is_paper_trading());

    // Create a paper trading client
    let paper_client = live_client.paper_trading()?;

    println!("Paper trading client created");
    println!("Is paper trading: {}", paper_client.is_paper_trading());

    // Alternatively, create a paper trading client directly
    let direct_paper_client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .paper_trading() // Enable paper trading
        .build()?;

    println!("Direct paper trading client created");
    println!(
        "Is paper trading: {}",
        direct_paper_client.is_paper_trading()
    );

    // Login to paper trading account
    println!("\nLogging in to paper trading account...");
    match paper_client.login("username", "password").await {
        Ok(_) => {
            println!("Logged in successfully to paper trading account");
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
            // Continue anyway for demonstration purposes
        }
        Err(e) => {
            return Err(e.into());
        }
    }

    // Get paper trading accounts
    println!("\nGetting paper trading accounts...");
    match paper_client.accounts().get_accounts().await {
        Ok(accounts) => {
            println!("Found {} paper trading accounts", accounts.len());

            if let Some(account) = accounts.first() {
                println!("  Account ID: {}", account.id);
                println!("  Account Number: {}", account.account_number);
                println!("  Account Type: {:?}", account.account_type);
                println!("  Status: {:?}", account.status);
                println!("  Currency: {}", account.currency);
                println!("  Paper Trading: {}", account.paper_trading); // Should be true

                // Get paper trading account balance
                println!("\nGetting paper trading account balance...");
                match paper_client
                    .accounts()
                    .get_account_balance(&account.id)
                    .await
                {
                    Ok(balance) => {
                        println!("  Cash: ${}", balance.cash);
                        println!("  Buying Power: ${}", balance.buying_power);
                        println!("  Market Value: ${}", balance.market_value);
                        println!("  Total Value: ${}", balance.total_value);
                    }
                    Err(WebullError::InvalidRequest(msg)) => {
                        println!("API not yet implemented: {}", msg);
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }

                // Get paper trading account positions
                println!("\nGetting paper trading account positions...");
                match paper_client.accounts().get_positions(&account.id).await {
                    Ok(positions) => {
                        println!(
                            "Found {} positions in paper trading account",
                            positions.len()
                        );
                        for position in positions {
                            println!("  Symbol: {}", position.symbol);
                            println!("  Quantity: {}", position.quantity);
                            println!("  Cost Basis: ${}", position.cost_basis);
                            println!("  Market Value: ${}", position.market_value);
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

    // Get real-time quote (works the same for both live and paper trading)
    println!("\nGetting real-time quote for AAPL...");
    match paper_client.market_data().get_quote("AAPL").await {
        Ok(quote) => {
            println!("AAPL current price: ${}", quote.last_price);
            println!("Change: ${} ({}%)", quote.change, quote.change_percent);
            println!("Volume: {}", quote.volume);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Logout from paper trading account
    println!("\nLogging out from paper trading account...");
    match paper_client.logout().await {
        Ok(_) => {
            println!("Logged out successfully from paper trading account");
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
