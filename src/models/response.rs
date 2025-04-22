use serde::{Deserialize, Serialize};

/// Common API response structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Success flag
    pub success: bool,

    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,

    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Create a new successful response.
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            code: None,
            message: None,
        }
    }

    /// Create a new error response.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            code: Some(code.into()),
            message: Some(message.into()),
        }
    }

    /// Check if the response is successful.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get the data from the response.
    pub fn get_data(&self) -> Option<&T> {
        self.data.as_ref()
    }
}

/// Pagination information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    /// Current page
    pub page: u32,

    /// Page size
    pub page_size: u32,

    /// Total number of items
    pub total: u32,

    /// Total number of pages
    pub total_pages: u32,
}

/// Paginated API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// Success flag
    pub success: bool,

    /// Response data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<T>>,

    /// Pagination information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,

    /// Error code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// Error message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> PaginatedResponse<T> {
    /// Create a new successful paginated response.
    pub fn success(data: Vec<T>, pagination: Pagination) -> Self {
        Self {
            success: true,
            data: Some(data),
            pagination: Some(pagination),
            code: None,
            message: None,
        }
    }

    /// Create a new error response.
    pub fn error(code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            pagination: None,
            code: Some(code.into()),
            message: Some(message.into()),
        }
    }

    /// Check if the response is successful.
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// Get the data from the response.
    pub fn get_data(&self) -> Option<&Vec<T>> {
        self.data.as_ref()
    }

    /// Get the pagination information.
    pub fn get_pagination(&self) -> Option<&Pagination> {
        self.pagination.as_ref()
    }
}
