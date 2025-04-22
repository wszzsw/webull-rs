use std::time::Duration;
use webull_rs::models::market::TimeFrame;
use webull_rs::{WebullClient, WebullError};

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

    println!("Getting quote for AAPL...");
    match client.market_data().get_quote("AAPL").await {
        Ok(quote) => {
            println!("AAPL quote:");
            println!("  Last price: ${}", quote.last_price);
            println!("  Change: ${} ({}%)", quote.change, quote.change_percent);
            println!("  Volume: {}", quote.volume);
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\nGetting daily bars for AAPL...");
    match client.market_data().get_daily_bars("AAPL", Some(5)).await {
        Ok(bars) => {
            println!("AAPL daily bars:");
            for bar in bars {
                println!(
                    "  {}: Open=${}, High=${}, Low=${}, Close=${}, Volume={}",
                    bar.timestamp.format("%Y-%m-%d"),
                    bar.open,
                    bar.high,
                    bar.low,
                    bar.close,
                    bar.volume
                );
            }
        }
        Err(WebullError::InvalidRequest(msg)) => {
            println!("API not yet implemented: {}", msg);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    println!("\nGetting intraday bars for AAPL...");
    match client
        .market_data()
        .get_intraday_bars("AAPL", TimeFrame::Minute5, Some(5))
        .await
    {
        Ok(bars) => {
            println!("AAPL 5-minute bars:");
            for bar in bars {
                println!(
                    "  {}: Open=${}, High=${}, Low=${}, Close=${}, Volume={}",
                    bar.timestamp.format("%Y-%m-%d %H:%M"),
                    bar.open,
                    bar.high,
                    bar.low,
                    bar.close,
                    bar.volume
                );
            }
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
