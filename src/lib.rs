//! A Rust client for the Webull trading API.
//!
//! This crate provides a robust, type-safe, and idiomatic interface to the Webull trading API,
//! enabling developers to build trading applications, algorithms, and bots in Rust.

// Re-export core modules
pub mod auth;
pub mod client;
pub mod config;
pub mod error;

// Re-export models and endpoints
pub mod endpoints;
pub mod models;
pub mod streaming;
pub mod utils;

// Re-export key types for convenience
pub use client::{WebullClient, WebullClientBuilder};
pub use config::WebullConfig;
pub use error::{WebullError, WebullResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_builder_works() {
        let client = WebullClient::builder().build();
        assert!(client.is_ok());
    }
}

// Test modules will be added in the future
