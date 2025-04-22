# Getting Started with webull-rs

This guide will help you get started with the webull-rs crate, a Rust client for the Webull trading API.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
webull-rs = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

## Basic Usage

### Creating a Client

```rust
use webull_rs::WebullClient;
use std::time::Duration;

// Create a client
let client = WebullClient::builder()
    .with_api_key("your-api-key")
    .with_api_secret("your-api-secret")
    .with_timeout(Duration::from_secs(30))
    .build()?;
```

### Authentication

```rust
// Login to Webull
client.login("username", "password").await?;

// Logout from Webull
client.logout().await?;

// Refresh the authentication token
client.refresh_token().await?;
```

### Account Information

```rust
// Get a list of accounts
let accounts = client.accounts().get_accounts().await?;

// Get account details
let account = client.accounts().get_account(&account_id).await?;

// Get account profile
let profile = client.accounts().get_account_profile(&account_id).await?;

// Get account balance
let balance = client.accounts().get_account_balance(&account_id).await?;

// Get account positions
let positions = client.accounts().get_positions(&account_id).await?;

// Get a specific position
let position = client.accounts().get_position(&account_id, "AAPL").await?;

// Get trade history
let trades = client.accounts().get_trade_history(&account_id).await?;

// Get trade history with pagination
let trades = client.accounts().get_trade_history_paged(&account_id, 1, 20).await?;
```

### Market Data

```rust
// Get a real-time quote for a symbol
let quote = client.market_data().get_quote("AAPL").await?;

// Get real-time quotes for multiple symbols
let quotes = client.market_data().get_quotes(&["AAPL", "MSFT", "GOOG"]).await?;

// Get snapshot data for a symbol
let snapshots = client.market_data().get_stock_snapshot("AAPL").await?;

// Get snapshot data for multiple symbols
let snapshots = client.market_data().get_stock_snapshots(&["AAPL", "MSFT", "GOOG"]).await?;

// Get instrument information
let instruments = client.market_data().get_stock_instrument("AAPL").await?;

// Get historical bars
let bars = client.market_data().get_daily_bars("AAPL", Some(30)).await?;

// Get intraday bars
let bars = client.market_data().get_intraday_bars("AAPL", TimeFrame::Minute5, Some(100)).await?;

// Get option chain
let option_chain = client.market_data().get_option_chain("AAPL").await?;

// Get market news
let news = client.market_data().get_market_news().await?;

// Get news for a symbol
let news = client.market_data().get_symbol_news("AAPL", Some(10)).await?;

// Get market calendar
let calendar = client.market_data().get_market_calendar().await?;
```

### Orders

```rust
// Place a market order
let order_request = OrderRequest::market()
    .symbol("AAPL")
    .side(OrderSide::Buy)
    .quantity(1.0)
    .time_in_force(TimeInForce::Day);

let order = client.orders().place_order(order_request).await?;

// Place a limit order
let order_request = OrderRequest::limit()
    .symbol("AAPL")
    .side(OrderSide::Buy)
    .quantity(1.0)
    .price(150.0)
    .time_in_force(TimeInForce::Day);

let order = client.orders().place_order(order_request).await?;

// Get order status
let order = client.orders().get_order(&order_id).await?;

// Cancel an order
client.orders().cancel_order(&order_id).await?;

// Get open orders
let orders = client.orders().get_open_orders(&account_id).await?;

// Get order history
let orders = client.orders().get_order_history(&account_id).await?;
```

### Watchlists

```rust
// Get watchlists
let watchlists = client.watchlists().get_watchlists().await?;

// Get watchlist items
let items = client.watchlists().get_watchlist_items(&watchlist_id).await?;

// Add item to watchlist
client.watchlists().add_to_watchlist(&watchlist_id, "AAPL").await?;

// Remove item from watchlist
client.watchlists().remove_from_watchlist(&watchlist_id, "AAPL").await?;

// Create a new watchlist
client.watchlists().create_watchlist("My Watchlist").await?;

// Delete a watchlist
client.watchlists().delete_watchlist(&watchlist_id).await?;
```

### Streaming Data

```rust
// Create a WebSocket client
let mut ws_client = client.streaming();

// Connect to the WebSocket server
ws_client.connect().await?;

// Subscribe to quote updates for AAPL
let subscription = SubscriptionRequest::new()
    .add_symbol("AAPL")
    .add_event_type(EventType::QuoteUpdate);

ws_client.subscribe(subscription).await?;

// Listen for events
while let Some(event) = ws_client.next_event().await {
    println!("Received event: {:?}", event);
}

// Disconnect
ws_client.disconnect().await?;
```

### Paper Trading

```rust
// Create a paper trading client
let paper_client = client.paper_trading()?;

// Or create a paper trading client directly
let paper_client = WebullClient::builder()
    .with_api_key("your-api-key")
    .with_api_secret("your-api-secret")
    .paper_trading() // Enable paper trading
    .build()?;

// Check if the client is configured for paper trading
let is_paper = paper_client.is_paper_trading(); // Should be true

// Use the paper trading client just like a regular client
paper_client.login("username", "password").await?;
let accounts = paper_client.accounts().get_accounts().await?;
// ...
```

### Secure Credential Storage

```rust
// Create a credential store
let credential_store = EncryptedCredentialStore::new(
    "credentials.json".to_string(),
    "token.json".to_string(),
    "my-secret-key".to_string(),
);

// Create a client with the credential store
let client = WebullClient::builder()
    .with_api_key("your-api-key")
    .with_api_secret("your-api-secret")
    .with_credential_store(credential_store)
    .build()?;

// Login to Webull
client.login("username", "password").await?;

// Credentials are now securely stored
// ...
```

## Error Handling

The crate uses a custom `WebullError` type for error handling:

```rust
match client.market_data().get_quote("AAPL").await {
    Ok(quote) => {
        println!("AAPL current price: ${}", quote.last_price);
    }
    Err(WebullError::NetworkError(e)) => {
        println!("Network error: {}", e);
    }
    Err(WebullError::ApiError { code, message }) => {
        println!("API error: {} - {}", code, message);
    }
    Err(WebullError::Unauthorized) => {
        println!("Unauthorized. Please login again.");
    }
    Err(WebullError::RateLimitExceeded) => {
        println!("Rate limit exceeded. Please try again later.");
    }
    Err(e) => {
        println!("Error: {}", e);
    }
}
```

## Advanced Configuration

```rust
// Configure the client with custom settings
let client = WebullClient::builder()
    .with_api_key("your-api-key")
    .with_api_secret("your-api-secret")
    .with_device_id("your-device-id") // Optional
    .with_timeout(Duration::from_secs(60))
    .with_base_url("https://api.webull.com") // Custom API URL
    .with_paper_trading(true) // Enable paper trading
    .with_token_store(MyCustomTokenStore::new()) // Custom token store
    .with_credential_store(MyCustomCredentialStore::new()) // Custom credential store
    .build()?;
```

## Examples

The crate includes several examples to help you get started:

- [Authentication](examples/auth.rs): Demonstrates how to authenticate with the Webull API
- [Account Information](examples/account.rs): Shows how to retrieve account information
- [Market Data](examples/market_data.rs): Demonstrates how to get quotes, bars, and option chains
- [Order Placement](examples/orders.rs): Shows how to place, modify, and cancel orders
- [Streaming Data](examples/streaming.rs): Demonstrates how to use the WebSocket client for real-time data
- [Credential Storage](examples/credentials.rs): Shows how to use the secure credential storage
- [Caching](examples/caching.rs): Demonstrates how to use response caching for improved performance
- [Paper Trading](examples/paper_trading.rs): Shows how to use paper trading
- [Account Extended](examples/account_extended.rs): Demonstrates advanced account functionality
- [Market Data Extended](examples/market_data_extended.rs): Shows advanced market data functionality

To run an example:

```bash
cargo run --example account
```

## Documentation

For detailed documentation, see [docs.rs/webull-rs](https://docs.rs/webull-rs).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
