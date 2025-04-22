use crate::auth::AuthManager;
use crate::error::{WebullError, WebullResult};
use crate::models::response::ApiResponse;
use crate::utils::cache::CacheManager;
use crate::utils::rate_limit::RateLimiter;
use reqwest::{Client, Method, RequestBuilder, StatusCode};
use reqwest::header::AUTHORIZATION;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// Base endpoint for API requests.
pub struct BaseEndpoint {
    /// HTTP client
    client: Client,

    /// Base URL for API requests
    base_url: String,

    /// Authentication manager
    auth_manager: Arc<AuthManager>,

    /// Rate limiter
    rate_limiter: Arc<RateLimiter>,

    /// Cache manager
    cache_manager: Arc<CacheManager>,
}

impl BaseEndpoint {
    /// Create a new base endpoint.
    pub fn new(client: Client, base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            client,
            base_url,
            auth_manager,
            rate_limiter: Arc::new(RateLimiter::new(60)), // Default to 60 requests per minute
            cache_manager: Arc::new(CacheManager::new()),
        }
    }

    /// Build a request to the API.
    pub fn request<T>(&self, method: Method, path: &str) -> RequestBuilder
    where
        T: DeserializeOwned,
    {
        let url = self.build_url(path);
        self.client.request(method, url)
    }

    /// Send a request to the API and parse the response.
    pub async fn send_request<T>(&self, request: RequestBuilder) -> WebullResult<T>
    where
        T: DeserializeOwned + Clone,
    {
        // Clone the request URL to get the path
        let req_url = request.try_clone()
            .ok_or_else(|| WebullError::InvalidRequest("Failed to clone request".to_string()))?
            .build()
            .map_err(WebullError::NetworkError)?
            .url()
            .clone();

        let path = req_url.path();

        // Wait for rate limit
        self.rate_limiter.wait(path).await;

        // Send the request
        let response = request.send().await.map_err(WebullError::NetworkError)?;

        let status = response.status();

        // Handle rate limiting
        if status == StatusCode::TOO_MANY_REQUESTS {
            // Get the retry-after header if available
            let retry_after = response.headers()
                .get("retry-after")
                .and_then(|h| h.to_str().ok())
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(1);

            // Wait for the specified time
            tokio::time::sleep(std::time::Duration::from_secs(retry_after)).await;

            return Err(WebullError::RateLimitExceeded);
        }

        // Handle unauthorized
        if status == StatusCode::UNAUTHORIZED {
            return Err(WebullError::Unauthorized);
        }

        // Get the response body
        let body = response.text().await.map_err(WebullError::NetworkError)?;

        // Handle other errors
        if !status.is_success() {
            return Err(WebullError::ApiError {
                code: status.as_u16().to_string(),
                message: body,
            });
        }

        // Parse the response
        let api_response: ApiResponse<T> = serde_json::from_str(&body)
            .map_err(|e| WebullError::SerializationError(e))?;

        // Check for API errors
        if !api_response.is_success() {
            return Err(WebullError::ApiError {
                code: api_response.code.unwrap_or_else(|| "unknown".to_string()),
                message: api_response.message.unwrap_or_else(|| "Unknown error".to_string()),
            });
        }

        // Return the data
        api_response.get_data().cloned().ok_or_else(|| WebullError::ApiError {
            code: "no_data".to_string(),
            message: "Response did not contain data".to_string(),
        })
    }

    /// Build a URL for the API.
    fn build_url(&self, path: &str) -> Url {
        let base = self.base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        let url = format!("{}/{}", base, path);

        Url::parse(&url).unwrap_or_else(|_| {
            // This should never happen if the base URL is valid
            panic!("Invalid URL: {}", url);
        })
    }

    /// Add authentication headers to a request.
    pub async fn authenticate_request(&self, request: RequestBuilder) -> WebullResult<RequestBuilder> {
        // Get the token from the auth manager
        let token = self.auth_manager.get_token().await?;

        // Add the token to the request headers
        let request = request.header(AUTHORIZATION, format!("Bearer {}", token.token));

        Ok(request)
    }

    /// Send a GET request to the API.
    pub async fn get<T>(&self, path: &str) -> WebullResult<T>
    where
        T: DeserializeOwned + Clone + Send + Sync + 'static,
    {
        // Check if we have a cached response
        let cache = self.cache_manager.get_cache::<T>("get");
        if let Some(cached) = cache.get("GET", path, None, None) {
            return Ok(cached);
        }

        // Send the request
        let request = self.request::<T>(Method::GET, path);
        let request = self.authenticate_request(request).await?;
        let response: T = self.send_request(request).await?;

        // Cache the response
        cache.set("GET", path, None, None, response.clone(), Some(Duration::from_secs(60)));

        Ok(response)
    }

    /// Send a POST request to the API.
    pub async fn post<T, B>(&self, path: &str, body: &B) -> WebullResult<T>
    where
        T: DeserializeOwned + Clone + Send + Sync + 'static,
        B: Serialize,
    {
        // For POST requests, we only cache if the body is cacheable
        // For simplicity, we'll just serialize the body and use it as part of the cache key
        let body_str = match serde_json::to_string(body) {
            Ok(s) => Some(s),
            Err(_) => None,
        };

        // Check if we have a cached response
        if let Some(body_str) = &body_str {
            let cache = self.cache_manager.get_cache::<T>("post");
            if let Some(cached) = cache.get("POST", path, None, Some(body_str)) {
                return Ok(cached);
            }
        }

        // Send the request
        let request = self.request::<T>(Method::POST, path).json(body);
        let request = self.authenticate_request(request).await?;
        let response: T = self.send_request(request).await?;

        // Cache the response if the body is cacheable
        if let Some(body_str) = body_str {
            let cache = self.cache_manager.get_cache::<T>("post");
            cache.set("POST", path, None, Some(&body_str), response.clone(), Some(Duration::from_secs(60)));
        }

        Ok(response)
    }

    /// Send a PUT request to the API.
    pub async fn put<T, B>(&self, path: &str, body: &B) -> WebullResult<T>
    where
        T: DeserializeOwned + Clone + Send + Sync + 'static,
        B: Serialize,
    {
        // For PUT requests, we don't cache the response, but we invalidate any cached GET responses
        // for the same path

        // Send the request
        let request = self.request::<T>(Method::PUT, path).json(body);
        let request = self.authenticate_request(request).await?;
        let response: T = self.send_request(request).await?;

        // Invalidate any cached GET responses for this path
        let get_cache = self.cache_manager.get_cache::<T>("get");
        get_cache.clear();

        Ok(response)
    }

    /// Send a DELETE request to the API.
    pub async fn delete<T>(&self, path: &str) -> WebullResult<T>
    where
        T: DeserializeOwned + Clone + Send + Sync + 'static,
    {
        // For DELETE requests, we don't cache the response, but we invalidate any cached responses
        // for the same path

        // Send the request
        let request = self.request::<T>(Method::DELETE, path);
        let request = self.authenticate_request(request).await?;
        let response: T = self.send_request(request).await?;

        // Invalidate any cached responses for this path
        let get_cache = self.cache_manager.get_cache::<T>("get");
        get_cache.clear();

        let post_cache = self.cache_manager.get_cache::<T>("post");
        post_cache.clear();

        Ok(response)
    }
}
