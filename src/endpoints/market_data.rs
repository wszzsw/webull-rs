use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::market::{
    Bar, BarQueryParams, NewsArticle, NewsQueryParams, OptionChain, OptionChainQueryParams, Quote,
    TimeFrame,
};
use reqwest::Client;
use serde::Serialize;
use std::sync::Arc;

/// Endpoints for market data operations.
pub struct MarketDataEndpoints {
    /// Base endpoint
    base: BaseEndpoint,
}

impl MarketDataEndpoints {
    /// Create new market data endpoints.
    pub fn new(client: Client, base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base: BaseEndpoint::new(client, base_url, auth_manager),
        }
    }
    
    /// Get a real-time quote for a symbol.
    pub async fn get_quote(&self, symbol: &str) -> WebullResult<Quote> {
        let path = format!("/api/quote/tickerRealTimes/{}", symbol);
        self.base.get(&path).await
    }
    
    /// Get real-time quotes for multiple symbols.
    pub async fn get_quotes(&self, symbols: &[&str]) -> WebullResult<Vec<Quote>> {
        #[derive(Serialize)]
        struct SymbolsRequest<'a> {
            symbols: Vec<&'a str>,
        }
        
        let request = SymbolsRequest {
            symbols: symbols.to_vec(),
        };
        
        self.base.post("/api/quote/tickerRealTimes", &request).await
    }
    
    /// Get historical bar data for a symbol.
    pub async fn get_bars(&self, params: &BarQueryParams) -> WebullResult<Vec<Bar>> {
        self.base.post("/api/quote/stockMinutes", params).await
    }
    
    /// Get option chain for a symbol.
    pub async fn get_option_chain(&self, params: &OptionChainQueryParams) -> WebullResult<OptionChain> {
        self.base.post("/api/options/list", params).await
    }
    
    /// Get market news.
    pub async fn get_news(&self, params: &NewsQueryParams) -> WebullResult<Vec<NewsArticle>> {
        self.base.post("/api/securities/news/list", params).await
    }
    
    /// Get market calendar.
    pub async fn get_market_calendar(&self) -> WebullResult<Vec<String>> {
        self.base.get("/api/securities/financial/calendar").await
    }
    
    /// Helper method to get daily bars for a symbol.
    pub async fn get_daily_bars(&self, symbol: &str, limit: Option<u32>) -> WebullResult<Vec<Bar>> {
        let mut params = BarQueryParams::new(symbol, TimeFrame::Day1);
        if let Some(limit) = limit {
            params = params.limit(limit);
        }
        
        self.get_bars(&params).await
    }
    
    /// Helper method to get intraday bars for a symbol.
    pub async fn get_intraday_bars(
        &self,
        symbol: &str,
        time_frame: TimeFrame,
        limit: Option<u32>,
    ) -> WebullResult<Vec<Bar>> {
        let mut params = BarQueryParams::new(symbol, time_frame);
        if let Some(limit) = limit {
            params = params.limit(limit);
        }
        
        self.get_bars(&params).await
    }
}
