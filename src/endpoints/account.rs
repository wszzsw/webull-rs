use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::account::{Account, AccountBalance, Position};
use reqwest::Client;
use std::sync::Arc;

/// Endpoints for account operations.
pub struct AccountEndpoints {
    /// Base endpoint
    base: BaseEndpoint,
}

impl AccountEndpoints {
    /// Create new account endpoints.
    pub fn new(client: Client, base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base: BaseEndpoint::new(client, base_url, auth_manager),
        }
    }
    
    /// Get a list of accounts.
    pub async fn get_accounts(&self) -> WebullResult<Vec<Account>> {
        self.base.get("/api/account/getSecAccountList").await
    }
    
    /// Get account details.
    pub async fn get_account(&self, account_id: &str) -> WebullResult<Account> {
        let path = format!("/api/account/getAccountMembers/{}", account_id);
        self.base.get(&path).await
    }
    
    /// Get account balance.
    pub async fn get_account_balance(&self, account_id: &str) -> WebullResult<AccountBalance> {
        let path = format!("/api/asset/getAssetSummary/{}", account_id);
        self.base.get(&path).await
    }
    
    /// Get account positions.
    pub async fn get_positions(&self, account_id: &str) -> WebullResult<Vec<Position>> {
        let path = format!("/api/position/getUserPositions/{}", account_id);
        self.base.get(&path).await
    }
    
    /// Get account position by symbol.
    pub async fn get_position(&self, account_id: &str, symbol: &str) -> WebullResult<Position> {
        let path = format!("/api/position/getUserPositions/{}/{}", account_id, symbol);
        self.base.get(&path).await
    }
    
    /// Get account trade history.
    pub async fn get_trade_history(&self, account_id: &str) -> WebullResult<Vec<Position>> {
        let path = format!("/api/trade/history/{}", account_id);
        self.base.get(&path).await
    }
}
