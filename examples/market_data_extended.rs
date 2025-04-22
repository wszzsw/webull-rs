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

    // Get snapshot data for AAPL
    println!("\nGetting snapshot for AAPL...");
    match client.market_data().get_stock_snapshot("AAPL").await {
        Ok(snapshots) => {
            println!("Found {} snapshots", snapshots.len());
            for snapshot in snapshots {
                println!("  Symbol: {}", snapshot.symbol);
                println!("  Last price: ${}", snapshot.last_price);
                println!(
                    "  Change: ${} ({}%)",
                    snapshot.change, snapshot.change_percent
                );
                println!("  Volume: {}", snapshot.volume);
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Get instrument information for AAPL
    println!("\nGetting instrument information for AAPL...");
    match client.market_data().get_stock_instrument("AAPL").await {
        Ok(instruments) => {
            println!("Found {} instruments", instruments.len());
            for instrument in instruments {
                println!("  ID: {}", instrument.id);
                println!("  Symbol: {}", instrument.symbol);
                println!("  Name: {}", instrument.name);
                println!("  Exchange: {}", instrument.exchange);
                println!("  Type: {}", instrument.security_type);
                println!("  Region: {}", instrument.region);
                println!("  Currency: {}", instrument.currency);
                println!("  Tradable: {}", instrument.tradable);
                println!("  Shortable: {}", instrument.shortable);
                println!("  Marginable: {}", instrument.marginable);
                println!("  Fractional tradable: {}", instrument.fractional_tradable);
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // Get historical bars for AAPL
    println!("\nGetting historical bars for AAPL...");
    match client.market_data().get_daily_bars("AAPL", Some(10)).await {
        Ok(bars) => {
            println!("Found {} bars", bars.len());
            for bar in bars {
                println!("  Date: {}", bar.timestamp.format("%Y-%m-%d"));
                println!("  Open: ${}", bar.open);
                println!("  High: ${}", bar.high);
                println!("  Low: ${}", bar.low);
                println!("  Close: ${}", bar.close);
                println!("  Volume: {}", bar.volume);
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

    // Note: The following endpoints are only available for Webull JP

    // Get EOD bars for an instrument (Webull JP only)
    println!("\nGetting EOD bars for instrument (Webull JP only)...");
    match client
        .market_data()
        .get_instrument_eod_bars("913256135", 5)
        .await
    {
        Ok(bars) => {
            println!("Found {} EOD bars", bars.len());
            for bar in bars {
                println!("  Date: {}", bar.timestamp.format("%Y-%m-%d"));
                println!("  Open: ${}", bar.open);
                println!("  High: ${}", bar.high);
                println!("  Low: ${}", bar.low);
                println!("  Close: ${}", bar.close);
                println!("  Volume: {}", bar.volume);
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

    // Get corporate actions for an instrument (Webull JP only)
    println!("\nGetting corporate actions for instrument (Webull JP only)...");
    match client.market_data().get_stock_splits("913256135").await {
        Ok(actions) => {
            println!("Found {} corporate actions", actions.len());
            for action in actions {
                println!("  ID: {}", action.id);
                println!("  Symbol: {}", action.symbol);
                println!("  Name: {}", action.name);
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
