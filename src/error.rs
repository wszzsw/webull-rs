use thiserror::Error;

/// Errors that can occur when interacting with the Webull API.
#[derive(Debug, Error)]
pub enum WebullError {
    /// Authentication error
    #[error("Authentication error: {0}")]
    AuthenticationError(String),
    
    /// API error with code and message
    #[error("API error: {code} - {message}")]
    ApiError { 
        code: String, 
        message: String 
    },
    
    /// Rate limit exceeded
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    /// Network error
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    /// Invalid request
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    
    /// Serialization error
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    /// MFA required
    #[error("MFA required")]
    MfaRequired,
    
    /// Unauthorized
    #[error("Unauthorized")]
    Unauthorized,
    
    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Result type for Webull API operations
pub type WebullResult<T> = Result<T, WebullError>;
