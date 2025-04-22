# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2023-10-15

### Added

- Initial release of the webull-rs crate
- Core infrastructure with WebullClient and builder pattern
- Authentication module with token management and MFA support
- Account endpoints for retrieving account information
- Market data endpoints for quotes, bars, and option chains
- Order endpoints for placing and managing orders
- Watchlist endpoints for managing watchlists
- WebSocket client for streaming data
- Rate limiting and backoff strategies
- Response caching for improved performance
- Secure credential storage with encryption
- Comprehensive documentation and examples

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
