use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::account::{
    Account, AccountBalance, AccountProfile, BalanceParams, Position, PositionParams, TradeHistory,
};
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
    pub async fn get_trade_history(&self, account_id: &str) -> WebullResult<Vec<TradeHistory>> {
        let path = format!("/api/trade/history/{}", account_id);
        self.base.get(&path).await
    }

    /// Get account trade history with pagination.
    pub async fn get_trade_history_paged(
        &self,
        account_id: &str,
        page: u32,
        page_size: u32,
    ) -> WebullResult<Vec<TradeHistory>> {
        #[derive(serde::Serialize)]
        struct TradeHistoryParams {
            account_id: String,
            page: u32,
            page_size: u32,
        }

        let params = TradeHistoryParams {
            account_id: account_id.to_string(),
            page,
            page_size,
        };

        self.base.post("/api/trade/history", &params).await
    }

    /// Get account profile information.
    pub async fn get_account_profile(&self, account_id: &str) -> WebullResult<AccountProfile> {
        let path = format!("/api/account/profile/{}", account_id);
        self.base.get(&path).await
    }

    /// Get account balance with parameters.
    pub async fn get_balance(&self, params: &BalanceParams) -> WebullResult<AccountBalance> {
        self.base.post("/api/account/balance", params).await
    }

    /// Helper method to get account balance with default currency.
    pub async fn get_balance_with_default_currency(
        &self,
        account_id: &str,
    ) -> WebullResult<AccountBalance> {
        let params = BalanceParams::new_with_default_currency(account_id);
        self.get_balance(&params).await
    }

    /// Get account positions with pagination.
    pub async fn get_positions_with_params(
        &self,
        params: &PositionParams,
    ) -> WebullResult<Vec<Position>> {
        self.base.post("/api/account/positions", params).await
    }

    /// Helper method to get account positions with pagination.
    pub async fn get_positions_paged(
        &self,
        account_id: &str,
        page_size: u32,
        last_instrument_id: Option<&str>,
    ) -> WebullResult<Vec<Position>> {
        let mut params = PositionParams::new(account_id, page_size);
        if let Some(last_id) = last_instrument_id {
            params = params.last_instrument_id(last_id);
        }
        self.get_positions_with_params(&params).await
    }
}
