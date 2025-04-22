use crate::config::WebullConfig;
use crate::error::{WebullError, WebullResult};
use crate::utils::crypto::{encrypt_password, generate_signature, generate_timestamp};
use crate::utils::serialization::{from_json, to_json};
use chrono::{DateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Mutex;

/// Credentials for authentication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credentials {
    /// Username for authentication
    pub username: String,

    /// Password for authentication
    pub password: String,
}

/// Access token for API requests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessToken {
    /// The access token
    pub token: String,

    /// When the token expires
    pub expires_at: DateTime<Utc>,

    /// The refresh token
    pub refresh_token: Option<String>,
}

/// Interface for storing and retrieving tokens.
pub trait TokenStore: Send + Sync {
    /// Get the current access token.
    fn get_token(&self) -> WebullResult<Option<AccessToken>>;

    /// Store an access token.
    fn store_token(&self, token: AccessToken) -> WebullResult<()>;

    /// Clear the stored token.
    fn clear_token(&self) -> WebullResult<()>;
}

/// In-memory token store.
#[derive(Debug, Default)]
pub struct MemoryTokenStore {
    token: Mutex<Option<AccessToken>>,
}

impl TokenStore for MemoryTokenStore {
    fn get_token(&self) -> WebullResult<Option<AccessToken>> {
        Ok(self.token.lock().unwrap().clone())
    }

    fn store_token(&self, token: AccessToken) -> WebullResult<()> {
        *self.token.lock().unwrap() = Some(token);
        Ok(())
    }

    fn clear_token(&self) -> WebullResult<()> {
        *self.token.lock().unwrap() = None;
        Ok(())
    }
}

/// Manager for authentication.
pub struct AuthManager {
    /// Credentials for authentication
    credentials: Option<Credentials>,

    /// Token store
    pub token_store: Box<dyn TokenStore>,

    /// Configuration
    config: WebullConfig,

    /// HTTP client
    client: reqwest::Client,
}

impl AuthManager {
    /// Create a new authentication manager.
    pub fn new(
        config: WebullConfig,
        token_store: Box<dyn TokenStore>,
        client: reqwest::Client,
    ) -> Self {
        Self {
            credentials: None,
            token_store,
            config,
            client,
        }
    }

    /// Authenticate with username and password.
    pub async fn authenticate(
        &mut self,
        username: &str,
        password: &str,
    ) -> WebullResult<AccessToken> {
        // Store credentials for potential token refresh
        self.credentials = Some(Credentials {
            username: username.to_string(),
            password: password.to_string(),
        });

        // Encrypt the password
        let encrypted_password = encrypt_password(
            password,
            &self.config.api_secret.clone().unwrap_or_default(),
        )?;

        // Create the request body
        let body = json!({
            "username": username,
            "password": encrypted_password,
            "deviceId": self.config.device_id.clone().unwrap_or_default(),
            "deviceName": "Rust API Client",
            "deviceType": "Web",
        });

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add API key if available
        if let Some(api_key) = &self.config.api_key {
            headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
        }

        // Generate timestamp and signature
        let timestamp = generate_timestamp();
        let signature = if let Some(api_secret) = &self.config.api_secret {
            let message = format!("{}{}", timestamp, to_json(&body)?);
            generate_signature(api_secret, &message)?
        } else {
            String::new()
        };

        // Add timestamp and signature to headers
        headers.insert("timestamp", HeaderValue::from_str(&timestamp).unwrap());
        headers.insert("signature", HeaderValue::from_str(&signature).unwrap());

        // Send the request
        let response = self
            .client
            .post(format!(
                "{}/api/passport/login/v5/account",
                self.config.base_url
            ))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if status.as_u16() == 401 {
                return Err(WebullError::Unauthorized);
            } else if status.as_u16() == 429 {
                return Err(WebullError::RateLimitExceeded);
            } else {
                return Err(WebullError::ApiError {
                    code: status.as_u16().to_string(),
                    message: text,
                });
            }
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        #[derive(Debug, Deserialize)]
        struct LoginResponse {
            access_token: String,
            refresh_token: String,
            token_type: String,
            expires_in: i64,
        }

        let login_response: LoginResponse = from_json(&response_text)?;

        // Create the token
        let token = AccessToken {
            token: login_response.access_token,
            expires_at: Utc::now() + chrono::Duration::seconds(login_response.expires_in),
            refresh_token: Some(login_response.refresh_token),
        };

        // Store the token
        self.token_store.store_token(token.clone())?;

        Ok(token)
    }

    /// Handle multi-factor authentication.
    pub async fn multi_factor_auth(&mut self, mfa_code: &str) -> WebullResult<AccessToken> {
        // Check if we have credentials
        let credentials = self.credentials.as_ref().ok_or_else(|| {
            WebullError::InvalidRequest("No credentials available for MFA".to_string())
        })?;

        // Create the request body
        let body = json!({
            "username": credentials.username,
            "verificationCode": mfa_code,
            "deviceId": self.config.device_id.clone().unwrap_or_default(),
        });

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add API key if available
        if let Some(api_key) = &self.config.api_key {
            headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
        }

        // Generate timestamp and signature
        let timestamp = generate_timestamp();
        let signature = if let Some(api_secret) = &self.config.api_secret {
            let message = format!("{}{}", timestamp, to_json(&body)?);
            generate_signature(api_secret, &message)?
        } else {
            String::new()
        };

        // Add timestamp and signature to headers
        headers.insert("timestamp", HeaderValue::from_str(&timestamp).unwrap());
        headers.insert("signature", HeaderValue::from_str(&signature).unwrap());

        // Send the request
        let response = self
            .client
            .post(format!(
                "{}/api/passport/verificationCode/verify",
                self.config.base_url
            ))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if status.as_u16() == 401 {
                return Err(WebullError::Unauthorized);
            } else if status.as_u16() == 429 {
                return Err(WebullError::RateLimitExceeded);
            } else {
                return Err(WebullError::ApiError {
                    code: status.as_u16().to_string(),
                    message: text,
                });
            }
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        #[derive(Debug, Deserialize)]
        struct MfaResponse {
            access_token: String,
            refresh_token: String,
            token_type: String,
            expires_in: i64,
        }

        let mfa_response: MfaResponse = from_json(&response_text)?;

        // Create the token
        let token = AccessToken {
            token: mfa_response.access_token,
            expires_at: Utc::now() + chrono::Duration::seconds(mfa_response.expires_in),
            refresh_token: Some(mfa_response.refresh_token),
        };

        // Store the token
        self.token_store.store_token(token.clone())?;

        Ok(token)
    }

    /// Refresh the access token.
    pub async fn refresh_token(&mut self) -> WebullResult<AccessToken> {
        // Get the current token
        let current_token = self.token_store.get_token()?.ok_or_else(|| {
            WebullError::InvalidRequest("No token available for refresh".to_string())
        })?;

        // Check if we have a refresh token
        let refresh_token = current_token
            .refresh_token
            .ok_or_else(|| WebullError::InvalidRequest("No refresh token available".to_string()))?;

        // Create the request body
        let body = json!({
            "refreshToken": refresh_token,
            "deviceId": self.config.device_id.clone().unwrap_or_default(),
        });

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        // Add API key if available
        if let Some(api_key) = &self.config.api_key {
            headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
        }

        // Generate timestamp and signature
        let timestamp = generate_timestamp();
        let signature = if let Some(api_secret) = &self.config.api_secret {
            let message = format!("{}{}", timestamp, to_json(&body)?);
            generate_signature(api_secret, &message)?
        } else {
            String::new()
        };

        // Add timestamp and signature to headers
        headers.insert("timestamp", HeaderValue::from_str(&timestamp).unwrap());
        headers.insert("signature", HeaderValue::from_str(&signature).unwrap());

        // Send the request
        let response = self
            .client
            .post(format!(
                "{}/api/passport/refreshToken",
                self.config.base_url
            ))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if status.as_u16() == 401 {
                return Err(WebullError::Unauthorized);
            } else if status.as_u16() == 429 {
                return Err(WebullError::RateLimitExceeded);
            } else {
                return Err(WebullError::ApiError {
                    code: status.as_u16().to_string(),
                    message: text,
                });
            }
        }

        // Parse the response
        let response_text = response
            .text()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        #[derive(Debug, Deserialize)]
        struct RefreshResponse {
            access_token: String,
            refresh_token: String,
            token_type: String,
            expires_in: i64,
        }

        let refresh_response: RefreshResponse = from_json(&response_text)?;

        // Create the token
        let token = AccessToken {
            token: refresh_response.access_token,
            expires_at: Utc::now() + chrono::Duration::seconds(refresh_response.expires_in),
            refresh_token: Some(refresh_response.refresh_token),
        };

        // Store the token
        self.token_store.store_token(token.clone())?;

        Ok(token)
    }

    /// Get the current access token.
    pub async fn get_token(&self) -> WebullResult<AccessToken> {
        match self.token_store.get_token()? {
            Some(token) => {
                // Check if token is expired
                if token.expires_at <= Utc::now() {
                    return Err(WebullError::Unauthorized);
                }
                Ok(token)
            }
            None => Err(WebullError::Unauthorized),
        }
    }

    /// Revoke the current token.
    pub async fn revoke_token(&mut self) -> WebullResult<()> {
        // Get the current token
        let current_token = match self.token_store.get_token()? {
            Some(token) => token,
            None => {
                // No token to revoke
                self.credentials = None;
                return Ok(());
            }
        };

        // Create the request body
        let body = json!({
            "accessToken": current_token.token,
            "deviceId": self.config.device_id.clone().unwrap_or_default(),
        });

        // Create headers
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", current_token.token)).unwrap(),
        );

        // Add API key if available
        if let Some(api_key) = &self.config.api_key {
            headers.insert("api-key", HeaderValue::from_str(api_key).unwrap());
        }

        // Generate timestamp and signature
        let timestamp = generate_timestamp();
        let signature = if let Some(api_secret) = &self.config.api_secret {
            let message = format!("{}{}", timestamp, to_json(&body)?);
            generate_signature(api_secret, &message)?
        } else {
            String::new()
        };

        // Add timestamp and signature to headers
        headers.insert("timestamp", HeaderValue::from_str(&timestamp).unwrap());
        headers.insert("signature", HeaderValue::from_str(&signature).unwrap());

        // Send the request
        let response = self
            .client
            .post(format!("{}/api/passport/logout", self.config.base_url))
            .headers(headers)
            .json(&body)
            .send()
            .await
            .map_err(|e| WebullError::NetworkError(e))?;

        // Check for errors
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());

            if status.as_u16() == 401 {
                // Token is already invalid, so we can just clear it
            } else if status.as_u16() == 429 {
                return Err(WebullError::RateLimitExceeded);
            } else {
                return Err(WebullError::ApiError {
                    code: status.as_u16().to_string(),
                    message: text,
                });
            }
        }

        // Clear the token and credentials
        self.token_store.clear_token()?;
        self.credentials = None;

        Ok(())
    }
}
