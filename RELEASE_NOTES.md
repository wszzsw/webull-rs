# Release Notes - webull-rs v0.1.0

We're excited to announce the initial release of `webull-rs`, a comprehensive Rust client for the Webull trading API. This crate provides a robust, type-safe, and idiomatic interface to the Webull trading platform, enabling developers to build trading applications, algorithms, and bots in Rust.

## Highlights

- **Complete API Coverage**: Comprehensive support for all major Webull API endpoints
- **Type Safety**: Strongly-typed models for all API entities
- **Async/Await**: Built with Tokio for efficient asynchronous operations
- **Error Handling**: Detailed error types for better error management
- **Authentication**: Support for multi-factor authentication and token refresh
- **Paper Trading**: Full support for paper trading accounts
- **Options Trading**: Complete functionality for options trading
- **Streaming Data**: WebSocket client for real-time market data and account updates
- **Secure Storage**: Encrypted credential and token storage

## Features

### Core Infrastructure

- `WebullClientBuilder` with comprehensive configuration options
- `WebullClient` with factory methods for specific API domains
- HTTP client infrastructure with reqwest and rustls-tls
- Comprehensive error handling with `WebullError` enum
- Rate limiting and backoff strategies

### Authentication

- `AuthManager` for handling Webull's authentication flow
- Login/logout functionality with username/password
- Support for multi-factor authentication
- Token refresh mechanism
- Token storage interface with memory and file-based implementations
- Secure credential storage with encryption

### Account Endpoints

- Account list retrieval
- Account details and profile information
- Account balance and summary data
- Position retrieval with pagination
- Trade history with filtering options
- Paper trading support

### Market Data Endpoints

- Real-time quotes for stocks, options, and other securities
- Historical bars with various timeframes
- Option chains with greeks and implied volatility
- Market news and calendar information
- Snapshot data for multiple symbols
- Instrument information and details
- End-of-day bars and corporate actions

### Order Endpoints

- Market, limit, stop, and trailing stop orders
- Option order placement and preview
- Order modification and cancellation
- Order status retrieval
- Order history with filtering
- Open orders and filled orders retrieval

### Watchlist Endpoints

- Watchlist retrieval and creation
- Watchlist item management
- Watchlist deletion and modification

### Streaming API

- WebSocket connection handling
- Authentication for streaming
- Reconnection logic
- Real-time quote streaming
- Order status update events
- Account update notifications
- Market data streaming

## Examples

The crate includes comprehensive examples for all major features:

- Authentication and session management
- Account information retrieval
- Market data access
- Order placement and management
- Options trading
- Paper trading
- Streaming data
- Error handling strategies
- Combined operations for complex trading scenarios

## Documentation

- Comprehensive rustdoc comments for all public API elements
- Detailed getting started guide
- API endpoint reference
- Usage examples for all major features

## Getting Started

Add this to your `Cargo.toml`:

```toml
[dependencies]
webull-rs = "0.1.0"
tokio = { version = "1", features = ["full"] }
```

Then, check out the [GETTING_STARTED.md](GETTING_STARTED.md) guide for detailed instructions on how to use the crate.

## Feedback and Contributions

We welcome feedback and contributions! Please open an issue on GitHub if you encounter any problems or have suggestions for improvements.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
