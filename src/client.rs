use crate::auth::{AuthManager, MemoryTokenStore, TokenStore};
use crate::config::WebullConfig;
use crate::endpoints::{account::AccountEndpoints, market_data::MarketDataEndpoints, orders::OrderEndpoints, watchlists::WatchlistEndpoints};
use crate::error::{WebullError, WebullResult};
use crate::streaming::client::WebSocketClient;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

/// Builder for creating a WebullClient.
pub struct WebullClientBuilder {
    api_key: Option<String>,
    api_secret: Option<String>,
    device_id: Option<String>,
    timeout: Duration,
    base_url: String,
    paper_trading: bool,
    token_store: Option<Box<dyn TokenStore>>,
}

impl WebullClientBuilder {
    /// Create a new builder with default values.
    pub fn new() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            device_id: None,
            timeout: Duration::from_secs(30),
            base_url: "https://api.webull.com".to_string(),
            paper_trading: false,
            token_store: None,
        }
    }

    /// Set the API key.
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }

    /// Set the API secret.
    pub fn with_api_secret(mut self, api_secret: impl Into<String>) -> Self {
        self.api_secret = Some(api_secret.into());
        self
    }

    /// Set the device ID.
    pub fn with_device_id(mut self, device_id: impl Into<String>) -> Self {
        self.device_id = Some(device_id.into());
        self
    }

    /// Set the timeout for API requests.
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set a custom base URL.
    pub fn with_custom_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set whether to use paper trading.
    pub fn with_paper_trading(mut self, paper_trading: bool) -> Self {
        self.paper_trading = paper_trading;
        self
    }

    /// Set a custom token store.
    pub fn with_token_store(mut self, store: impl TokenStore + 'static) -> Self {
        self.token_store = Some(Box::new(store));
        self
    }

    /// Build the WebullClient.
    pub fn build(self) -> WebullResult<WebullClient> {
        // Generate a random device ID if not provided
        let device_id = self.device_id.unwrap_or_else(|| Uuid::new_v4().to_hyphenated().to_string());

        // Create the configuration
        let config = WebullConfig {
            api_key: self.api_key,
            api_secret: self.api_secret,
            device_id: Some(device_id),
            timeout: self.timeout,
            base_url: self.base_url,
            paper_trading: self.paper_trading,
        };

        // Create the HTTP client
        let client = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .map_err(|e| WebullError::NetworkError(e))?;

        // Create the token store
        let token_store = self.token_store.unwrap_or_else(|| Box::new(MemoryTokenStore::default()));

        // Create the auth manager
        let auth_manager = Arc::new(AuthManager::new(config.clone(), token_store, client.clone()));

        Ok(WebullClient {
            inner: client,
            config,
            auth_manager,
        })
    }
}

/// Client for interacting with the Webull API.
pub struct WebullClient {
    /// HTTP client
    inner: reqwest::Client,

    /// Configuration
    config: WebullConfig,

    /// Authentication manager
    auth_manager: Arc<AuthManager>,
}

impl WebullClient {
    /// Create a new builder for configuring the client.
    pub fn builder() -> WebullClientBuilder {
        WebullClientBuilder::new()
    }

    /// Login to Webull.
    pub async fn login(&self, username: &str, password: &str) -> WebullResult<()> {
        // Create a new AuthManager with the same configuration
        let mut auth_manager = AuthManager::new(
            self.config.clone(),
            Box::new(MemoryTokenStore::default()),
            self.inner.clone(),
        );

        // Authenticate
        let token = auth_manager.authenticate(username, password).await?;

        // Store the token in the original auth_manager
        let token_store = self.auth_manager.token_store.as_ref();
        token_store.store_token(token)?;

        Ok(())
    }

    /// Logout from Webull.
    pub async fn logout(&self) -> WebullResult<()> {
        // Create a new AuthManager with the same configuration
        let mut auth_manager = AuthManager::new(
            self.config.clone(),
            Box::new(MemoryTokenStore::default()),
            self.inner.clone(),
        );

        // Get the current token from the original auth_manager
        let token = match self.auth_manager.token_store.get_token()? {
            Some(token) => token,
            None => {
                // No token to revoke
                return Ok(());
            }
        };

        // Store the token in the new auth_manager
        auth_manager.token_store.store_token(token)?;

        // Revoke the token
        auth_manager.revoke_token().await?;

        // Clear the token in the original auth_manager
        self.auth_manager.token_store.clear_token()?;

        Ok(())
    }

    /// Refresh the authentication token.
    pub async fn refresh_token(&self) -> WebullResult<()> {
        // Create a new AuthManager with the same configuration
        let mut auth_manager = AuthManager::new(
            self.config.clone(),
            Box::new(MemoryTokenStore::default()),
            self.inner.clone(),
        );

        // Get the current token from the original auth_manager
        let token = match self.auth_manager.token_store.get_token()? {
            Some(token) => token,
            None => {
                return Err(WebullError::InvalidRequest("No token available for refresh".to_string()));
            }
        };

        // Store the token in the new auth_manager
        auth_manager.token_store.store_token(token)?;

        // Refresh the token
        let new_token = auth_manager.refresh_token().await?;

        // Store the new token in the original auth_manager
        self.auth_manager.token_store.store_token(new_token)?;

        Ok(())
    }

    /// Get account endpoints.
    pub fn accounts(&self) -> AccountEndpoints {
        AccountEndpoints::new(
            self.inner.clone(),
            self.config.base_url.clone(),
            self.auth_manager.clone(),
        )
    }

    /// Get market data endpoints.
    pub fn market_data(&self) -> MarketDataEndpoints {
        MarketDataEndpoints::new(
            self.inner.clone(),
            self.config.base_url.clone(),
            self.auth_manager.clone(),
        )
    }

    /// Get order endpoints.
    pub fn orders(&self) -> OrderEndpoints {
        OrderEndpoints::new(
            self.inner.clone(),
            self.config.base_url.clone(),
            self.auth_manager.clone(),
        )
    }

    /// Get watchlist endpoints.
    pub fn watchlists(&self) -> WatchlistEndpoints {
        WatchlistEndpoints::new(
            self.inner.clone(),
            self.config.base_url.clone(),
            self.auth_manager.clone(),
        )
    }

    /// Create a WebSocket client for streaming data.
    pub fn streaming(&self) -> WebSocketClient {
        let ws_base_url = self.config.base_url.clone().replace("http", "ws");
        WebSocketClient::new(ws_base_url, self.auth_manager.clone())
    }
}
