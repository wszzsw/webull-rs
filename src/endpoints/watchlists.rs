use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Watchlist information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Watchlist {
    /// Watchlist ID
    pub id: String,
    
    /// Watchlist name
    pub name: String,
    
    /// Watchlist symbols
    pub symbols: Vec<String>,
}

/// Request to create a watchlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWatchlistRequest {
    /// Watchlist name
    pub name: String,
    
    /// Watchlist symbols
    pub symbols: Vec<String>,
}

/// Request to modify a watchlist.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifyWatchlistRequest {
    /// Watchlist ID
    pub id: String,
    
    /// Watchlist name
    pub name: Option<String>,
    
    /// Watchlist symbols to add
    pub add_symbols: Option<Vec<String>>,
    
    /// Watchlist symbols to remove
    pub remove_symbols: Option<Vec<String>>,
}

/// Endpoints for watchlist operations.
pub struct WatchlistEndpoints {
    /// Base endpoint
    base: BaseEndpoint,
}

impl WatchlistEndpoints {
    /// Create new watchlist endpoints.
    pub fn new(client: Client, base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base: BaseEndpoint::new(client, base_url, auth_manager),
        }
    }
    
    /// Get all watchlists.
    pub async fn get_watchlists(&self) -> WebullResult<Vec<Watchlist>> {
        self.base.get("/api/wlas/watchlist").await
    }
    
    /// Get a watchlist by ID.
    pub async fn get_watchlist(&self, watchlist_id: &str) -> WebullResult<Watchlist> {
        let path = format!("/api/wlas/watchlist/{}", watchlist_id);
        self.base.get(&path).await
    }
    
    /// Create a watchlist.
    pub async fn create_watchlist(&self, request: &CreateWatchlistRequest) -> WebullResult<Watchlist> {
        self.base.post("/api/wlas/watchlist", request).await
    }
    
    /// Modify a watchlist.
    pub async fn modify_watchlist(&self, request: &ModifyWatchlistRequest) -> WebullResult<Watchlist> {
        self.base.post("/api/wlas/watchlist/modify", request).await
    }
    
    /// Delete a watchlist.
    pub async fn delete_watchlist(&self, watchlist_id: &str) -> WebullResult<()> {
        let path = format!("/api/wlas/watchlist/delete/{}", watchlist_id);
        self.base.delete(&path).await
    }
    
    /// Add symbols to a watchlist.
    pub async fn add_symbols(&self, watchlist_id: &str, symbols: &[String]) -> WebullResult<Watchlist> {
        let request = ModifyWatchlistRequest {
            id: watchlist_id.to_string(),
            name: None,
            add_symbols: Some(symbols.to_vec()),
            remove_symbols: None,
        };
        
        self.modify_watchlist(&request).await
    }
    
    /// Remove symbols from a watchlist.
    pub async fn remove_symbols(&self, watchlist_id: &str, symbols: &[String]) -> WebullResult<Watchlist> {
        let request = ModifyWatchlistRequest {
            id: watchlist_id.to_string(),
            name: None,
            add_symbols: None,
            remove_symbols: Some(symbols.to_vec()),
        };
        
        self.modify_watchlist(&request).await
    }
    
    /// Rename a watchlist.
    pub async fn rename_watchlist(&self, watchlist_id: &str, name: &str) -> WebullResult<Watchlist> {
        let request = ModifyWatchlistRequest {
            id: watchlist_id.to_string(),
            name: Some(name.to_string()),
            add_symbols: None,
            remove_symbols: None,
        };
        
        self.modify_watchlist(&request).await
    }
}
