use std::time::Duration;

/// Configuration for the Webull API client.
#[derive(Debug, Clone)]
pub struct WebullConfig {
    /// API key for authentication
    pub api_key: Option<String>,

    /// API secret for authentication
    pub api_secret: Option<String>,

    /// Device ID for authentication
    pub device_id: Option<String>,

    /// Timeout for API requests
    pub timeout: Duration,

    /// Base URL for API requests
    pub base_url: String,

    /// Whether to use paper trading
    pub paper_trading: bool,
}

impl Default for WebullConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            device_id: None,
            timeout: Duration::from_secs(30),
            base_url: "https://api.webull.com".to_string(),
            paper_trading: false,
        }
    }
}

impl WebullConfig {
    /// Create a new configuration with default values.
    pub fn new() -> Self {
        Self::default()
    }
}
