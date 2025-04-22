use std::collections::HashMap;
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

    // Example 1: Portfolio rebalancing
    println!("\nExample 1: Portfolio rebalancing");

    // Step 1: Get account information
    let accounts = match client.accounts().get_accounts().await {
        Ok(accounts) => accounts,
        Err(e) => {
            println!("Error getting accounts: {}", e);
            return Err(e.into());
        }
    };

    if let Some(account) = accounts.first() {
        let account_id = &account.id;
        println!("Using account ID: {}", account_id);

        // Step 2: Get current positions
        let positions = match client.accounts().get_positions(account_id).await {
            Ok(positions) => positions,
            Err(WebullError::InvalidRequest(msg)) => {
                println!("API not yet implemented: {}", msg);
                Vec::new() // Continue with empty positions for demonstration
            }
            Err(e) => {
                println!("Error getting positions: {}", e);
                return Err(e.into());
            }
        };

        // Step 3: Get account balance
        let balance = match client.accounts().get_account_balance(account_id).await {
            Ok(balance) => balance,
            Err(WebullError::InvalidRequest(msg)) => {
                println!("API not yet implemented: {}", msg);
                // Create a dummy balance for demonstration
                webull_rs::models::account::AccountBalance {
                    cash: 10000.0,
                    buying_power: 10000.0,
                    market_value: 10000.0,
                    total_value: 20000.0,
                    unrealized_profit_loss: 0.0,
                    unrealized_profit_loss_percentage: 0.0,
                    currency: "USD".to_string(),
                    settled_cash: Some(10000.0),
                    unsettled_cash: Some(0.0),
                    withdrawable_cash: Some(10000.0),
                    tradable_cash: Some(10000.0),
                    margin_buying_power: Some(20000.0),
                    option_buying_power: Some(10000.0),
                    day_trading_buying_power: Some(40000.0),
                }
            }
            Err(e) => {
                println!("Error getting account balance: {}", e);
                return Err(e.into());
            }
        };

        // Step 4: Define target allocation
        let target_allocation = HashMap::from([
            ("AAPL", 0.25),
            ("MSFT", 0.25),
            ("GOOG", 0.25),
            ("AMZN", 0.25),
        ]);

        // Step 5: Get current prices
        let mut current_prices = HashMap::new();
        for symbol in target_allocation.keys() {
            match client.market_data().get_quote(symbol).await {
                Ok(quote) => {
                    current_prices.insert(symbol.to_string(), quote.last_price);
                    println!("Current price of {}: ${}", symbol, quote.last_price);
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented for {}: {}", symbol, msg);
                    // Use a dummy price for demonstration
                    current_prices.insert(symbol.to_string(), 150.0);
                }
                Err(e) => {
                    println!("Error getting quote for {}: {}", symbol, e);
                    return Err(e.into());
                }
            }
        }

        // Step 6: Calculate current allocation
        let mut current_allocation = HashMap::new();
        let total_value = balance.market_value;

        for position in &positions {
            current_allocation.insert(position.symbol.clone(), position.market_value / total_value);
            println!(
                "Current allocation of {}: {:.2}%",
                position.symbol,
                current_allocation[&position.symbol] * 100.0
            );
        }

        // Step 7: Calculate trades needed
        let mut trades = Vec::new();

        for (symbol, target) in &target_allocation {
            let current = current_allocation.get(*symbol).unwrap_or(&0.0);
            let difference = target - current;

            if difference.abs() > 0.01 {
                // Only rebalance if difference is more than 1%
                let trade_value = difference * total_value;
                let price = current_prices.get(*symbol).unwrap_or(&150.0);
                let quantity = (trade_value / price).abs().floor();

                if quantity > 0.0 {
                    let side = if difference > 0.0 {
                        OrderSide::Buy
                    } else {
                        OrderSide::Sell
                    };

                    println!(
                        "Need to {} {:.0} shares of {} at ${} (${:.2})",
                        if side == OrderSide::Buy {
                            "buy"
                        } else {
                            "sell"
                        },
                        quantity,
                        symbol,
                        price,
                        quantity * price
                    );

                    trades.push((symbol.to_string(), side, quantity, *price));
                }
            }
        }

        // Step 8: Execute trades
        for (symbol, side, quantity, price) in trades {
            let order = OrderRequest::limit()
                .symbol(&symbol)
                .quantity(quantity)
                .price(price)
                .side(side)
                .time_in_force(TimeInForce::Day)
                .client_order_id(format!("rebalance-{}", symbol));

            match client.orders().place_order(&order).await {
                Ok(response) => {
                    println!("Order placed for {}: ID {}", symbol, response.id);
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented for {}: {}", symbol, msg);
                }
                Err(e) => {
                    println!("Error placing order for {}: {}", symbol, e);
                }
            }
        }
    }

    // Example 2: Market data analysis and trading
    println!("\nExample 2: Market data analysis and trading");

    // Step 1: Get historical data for multiple symbols
    let symbols = vec!["AAPL", "MSFT", "GOOG", "AMZN"];
    let mut historical_data = HashMap::new();

    for symbol in &symbols {
        match client.market_data().get_daily_bars(symbol, Some(30)).await {
            Ok(bars) => {
                let bar_count = bars.len();
                historical_data.insert(symbol.to_string(), bars);
                println!(
                    "Retrieved {} days of historical data for {}",
                    bar_count,
                    symbol
                );
            }
            Err(WebullError::InvalidRequest(msg)) => {
                println!("API not yet implemented for {}: {}", symbol, msg);
            }
            Err(e) => {
                println!("Error getting historical data for {}: {}", symbol, e);
            }
        }
    }

    // Step 2: Calculate simple moving averages
    let mut sma_20 = HashMap::new();
    let mut sma_50 = HashMap::new();

    for (symbol, bars) in &historical_data {
        if bars.len() >= 20 {
            let sum: f64 = bars.iter().take(20).map(|bar| bar.close).sum();
            let avg = sum / 20.0;
            sma_20.insert(symbol.clone(), avg);
            println!("20-day SMA for {}: ${:.2}", symbol, avg);
        }

        if bars.len() >= 50 {
            let sum: f64 = bars.iter().take(50).map(|bar| bar.close).sum();
            let avg = sum / 50.0;
            sma_50.insert(symbol.clone(), avg);
            println!("50-day SMA for {}: ${:.2}", symbol, avg);
        }
    }

    // Step 3: Get current prices
    let mut current_prices = HashMap::new();
    for symbol in &symbols {
        match client.market_data().get_quote(symbol).await {
            Ok(quote) => {
                current_prices.insert(symbol.to_string(), quote.last_price);
                println!("Current price of {}: ${}", symbol, quote.last_price);
            }
            Err(WebullError::InvalidRequest(msg)) => {
                println!("API not yet implemented for {}: {}", symbol, msg);
                // Use a dummy price for demonstration
                current_prices.insert(symbol.to_string(), 150.0);
            }
            Err(e) => {
                println!("Error getting quote for {}: {}", symbol, e);
            }
        }
    }

    // Step 4: Identify trading signals (SMA crossover)
    let mut buy_signals = Vec::new();
    let mut sell_signals = Vec::new();

    for symbol in &symbols {
        if let (Some(sma20), Some(sma50), Some(current_price)) = (
            sma_20.get(*symbol),
            sma_50.get(*symbol),
            current_prices.get(*symbol),
        ) {
            // Buy signal: Current price and 20-day SMA are above 50-day SMA
            if *current_price > *sma50 && *sma20 > *sma50 {
                buy_signals.push(symbol.to_string());
                println!(
                    "Buy signal for {}: Current price ${} > 50-day SMA ${:.2}",
                    symbol, current_price, sma50
                );
            }

            // Sell signal: Current price and 20-day SMA are below 50-day SMA
            if *current_price < *sma50 && *sma20 < *sma50 {
                sell_signals.push(symbol.to_string());
                println!(
                    "Sell signal for {}: Current price ${} < 50-day SMA ${:.2}",
                    symbol, current_price, sma50
                );
            }
        }
    }

    // Step 5: Execute trades based on signals
    if let Some(account) = accounts.first() {
        let account_id = &account.id;

        // Get current positions
        let positions = match client.accounts().get_positions(account_id).await {
            Ok(positions) => positions,
            Err(WebullError::InvalidRequest(msg)) => {
                println!("API not yet implemented: {}", msg);
                Vec::new() // Continue with empty positions for demonstration
            }
            Err(e) => {
                println!("Error getting positions: {}", e);
                Vec::new()
            }
        };

        // Map positions by symbol
        let position_map: HashMap<String, &webull_rs::models::account::Position> =
            positions.iter().map(|p| (p.symbol.clone(), p)).collect();

        // Execute buy signals
        for symbol in buy_signals {
            // Skip if we already have a position
            if position_map.contains_key(&symbol) {
                println!("Already have a position in {}, skipping buy", symbol);
                continue;
            }

            let price = current_prices.get(&symbol).unwrap_or(&150.0);
            let quantity = 1.0; // Buy 1 share for demonstration

            let order = OrderRequest::limit()
                .symbol(&symbol)
                .quantity(quantity)
                .price(*price * 1.01) // Set limit price 1% above current price
                .side(OrderSide::Buy)
                .time_in_force(TimeInForce::Day)
                .client_order_id(format!("sma-crossover-buy-{}", symbol));

            match client.orders().place_order(&order).await {
                Ok(response) => {
                    println!("Buy order placed for {}: ID {}", symbol, response.id);
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented for {}: {}", symbol, msg);
                }
                Err(e) => {
                    println!("Error placing buy order for {}: {}", symbol, e);
                }
            }
        }

        // Execute sell signals
        for symbol in sell_signals {
            // Skip if we don't have a position
            if !position_map.contains_key(&symbol) {
                println!("No position in {}, skipping sell", symbol);
                continue;
            }

            let position = position_map[&symbol];
            let price = current_prices.get(&symbol).unwrap_or(&150.0);

            let order = OrderRequest::limit()
                .symbol(&symbol)
                .quantity(position.quantity)
                .price(*price * 0.99) // Set limit price 1% below current price
                .side(OrderSide::Sell)
                .time_in_force(TimeInForce::Day)
                .client_order_id(format!("sma-crossover-sell-{}", symbol));

            match client.orders().place_order(&order).await {
                Ok(response) => {
                    println!("Sell order placed for {}: ID {}", symbol, response.id);
                }
                Err(WebullError::InvalidRequest(msg)) => {
                    println!("API not yet implemented for {}: {}", symbol, msg);
                }
                Err(e) => {
                    println!("Error placing sell order for {}: {}", symbol, e);
                }
            }
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
