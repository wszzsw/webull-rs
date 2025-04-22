# webull-rs

A Rust client for the Webull trading API.

[![Crates.io](https://img.shields.io/crates/v/webull-rs.svg)](https://crates.io/crates/webull-rs)
[![Documentation](https://docs.rs/webull-rs/badge.svg)](https://docs.rs/webull-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview

`webull-rs` is a Rust crate that provides a robust, type-safe, and idiomatic interface to the Webull trading API. It enables developers to build trading applications, algorithms, and bots in Rust.

## Features

- **Type-safe API client** for Webull trading platform
- **Comprehensive error handling** with detailed error types
- **Async/await support** with Tokio
- **Authentication** with support for multi-factor authentication and token refresh
- **Account management** for retrieving account information, positions, and balances
- **Market data** for quotes, bars, option chains, and more
- **Trading operations** for placing, modifying, and canceling orders (including options trading)
- **Streaming data** via WebSockets for real-time updates
- **Rate limiting** with configurable strategies to avoid API rate limit violations
- **Response caching** for improved performance
- **Secure credential storage** with encryption
- **Strongly-typed models** for all API entities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
webull-rs = "0.1.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use webull_rs::{WebullClient, OrderSide, OrderType, TimeInForce, OrderRequest};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .with_timeout(Duration::from_secs(30))
        .build()?;

    // Login to Webull
    client.login("username", "password").await?;

    // Get account information
    let accounts = client.accounts().get_accounts().await?;
    println!("Found {} accounts", accounts.len());

    // Get real-time quote
    let quote = client.market_data().get_quote("AAPL").await?;
    println!("AAPL current price: ${}", quote.last_price);

    // Place an order
    let order_request = OrderRequest::market()
        .symbol("AAPL")
        .side(OrderSide::Buy)
        .quantity(1.0)
        .time_in_force(TimeInForce::Day);

    let order = client.orders().place_order(order_request).await?;
    println!("Order placed: {}", order.id);

    // Logout
    client.logout().await?;

    Ok(())
}
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
- [Orders Extended](examples/orders_extended.rs): Demonstrates advanced order functionality including options trading
- [Error Handling](examples/error_handling.rs): Shows comprehensive error handling strategies
- [Combined Operations](examples/combined_operations.rs): Demonstrates combining multiple API calls for complex operations

To run an example:

```bash
cargo run --example account
```

## Architecture

The crate is organized into several modules:

- `client`: The main WebullClient and builder
- `auth`: Authentication and token management
- `endpoints`: API endpoints for different domains
- `models`: Data models for API requests and responses
- `streaming`: WebSocket client for streaming data
- `utils`: Utility functions and helpers

## Advanced Usage

### Secure Credential Storage

```rust
use webull_rs::{WebullClient, WebullError};
use webull_rs::utils::credentials::EncryptedCredentialStore;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}
```

### Streaming Data

```rust
use webull_rs::{WebullClient, WebullError};
use webull_rs::streaming::events::EventType;
use webull_rs::streaming::subscription::SubscriptionRequest;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = WebullClient::builder()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret")
        .build()?;

    // Login to Webull
    client.login("username", "password").await?;

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

    Ok(())
}
```

## Documentation

For detailed documentation, see [docs.rs/webull-rs](https://docs.rs/webull-rs).

For a comprehensive getting started guide, see [GETTING_STARTED.md](GETTING_STARTED.md).

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

## Security

Please see [SECURITY.md](SECURITY.md) for security policies and best practices.

## Changelog

See [CHANGELOG.md](CHANGELOG.md) for a list of changes in each release.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
