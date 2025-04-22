# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2023-10-15

### Added

#### Core Infrastructure
- Implemented `WebullClientBuilder` with configuration options
- Created `WebullClient` struct with core methods
- Set up HTTP client infrastructure with reqwest
- Added support for authentication and token management
- Implemented error handling with `WebullError` enum

#### Authentication
- Implemented `AuthManager` for handling Webull authentication flow
- Added login/logout functionality
- Added support for MFA (Multi-Factor Authentication)
- Implemented token refresh mechanism
- Created token storage interface with memory and file-based implementations
- Added secure credential storage

#### Account Endpoints
- Implemented account list retrieval
- Added account details endpoint
- Created methods for account summary data
- Implemented position retrieval functionality
- Added trade history endpoints
- Implemented account profile retrieval
- Added pagination for positions and trades
- Implemented paper trading support

#### Market Data Endpoints
- Implemented real-time quote functionality
- Created ticker information methods
- Added intraday data retrieval
- Implemented market calendar endpoints
- Added market news functionality
- Created option chain data retrieval
- Implemented snapshot data retrieval
- Added instrument information endpoints
- Implemented end-of-day bars functionality
- Added corporate action endpoints

#### Order Endpoints
- Implemented market order functionality
- Added limit order support
- Created stop order methods
- Implemented order validation
- Added trailing stop order support
- Implemented option order functionality
- Added order preview support
- Added order modification endpoints
- Implemented order cancellation
- Created order status retrieval
- Added methods for order history

#### Watchlist Endpoints
- Implemented watchlist retrieval
- Added watchlist creation methods
- Created watchlist modification functionality
- Implemented watchlist deletion

#### Streaming API
- Set up WebSocket connection handling
- Implemented authentication for streaming
- Created reconnection logic
- Added ping/pong heartbeat mechanism
- Implemented real-time quote streaming
- Added order status update events
- Created account update notifications
- Implemented market data streaming

#### Documentation
- Added comprehensive rustdoc comments
- Created usage examples for all major features
- Added detailed getting started guide
- Created API endpoint reference guide

### Changed

- N/A (initial release)

### Deprecated

- N/A (initial release)

### Removed

- N/A (initial release)

### Fixed

- N/A (initial release)

### Security

- Implemented secure credential storage with encryption
- Added token refresh mechanism
- Implemented proper error handling for authentication failures
