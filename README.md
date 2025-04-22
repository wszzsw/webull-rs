# webull-rs

A Rust client for the Webull trading API.

## Overview

`webull-rs` is a Rust crate that provides a robust, type-safe, and idiomatic interface to the Webull trading API. It enables developers to build trading applications, algorithms, and bots in Rust.

## Features

- Type-safe API client for Webull trading platform
- Comprehensive error handling
- Async/await support with Tokio
- Support for account management, market data, and trading operations
- Streaming data via WebSockets
- Strongly-typed models for all API entities

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
webull-rs = "0.1.0"
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
    
    // Logout
    client.logout().await?;
    
    Ok(())
}
```

## Documentation

For detailed documentation, see [docs.rs/webull-rs](https://docs.rs/webull-rs).

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
