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
pub mod models;
pub mod endpoints;
pub mod utils;
pub mod streaming;

// Re-export key types for convenience
pub use client::{WebullClient, WebullClientBuilder};
pub use error::{WebullError, WebullResult};
pub use config::WebullConfig;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_builder_works() {
        let client = WebullClient::builder().build();
        assert!(client.is_ok());
    }
}
