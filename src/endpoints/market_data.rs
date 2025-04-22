use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::market::{
    Bar, BarQueryParams, CorpActionEventType, CorpActionParams, EodBarsParams, Instrument,
    InstrumentParams, NewsArticle, NewsQueryParams, OptionChain, OptionChainQueryParams, Quote,
    SnapshotParams, TimeFrame,
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

    /// Get snapshot data for symbols.
    pub async fn get_snapshot(&self, params: &SnapshotParams) -> WebullResult<Vec<Quote>> {
        self.base.post("/api/quote/snapshot", params).await
    }

    /// Helper method to get snapshot for a single stock symbol.
    pub async fn get_stock_snapshot(&self, symbol: &str) -> WebullResult<Vec<Quote>> {
        let params = SnapshotParams::new_stock(symbol);
        self.get_snapshot(&params).await
    }

    /// Helper method to get snapshot for multiple stock symbols.
    pub async fn get_stock_snapshots(&self, symbols: &[&str]) -> WebullResult<Vec<Quote>> {
        let params = SnapshotParams::new_stocks(symbols);
        self.get_snapshot(&params).await
    }

    /// Get historical bar data for a symbol.
    pub async fn get_history_bar(&self, params: &BarQueryParams) -> WebullResult<Vec<Bar>> {
        self.base.post("/api/quote/history/bars", params).await
    }

    /// Get option chain for a symbol.
    pub async fn get_option_chain(
        &self,
        params: &OptionChainQueryParams,
    ) -> WebullResult<OptionChain> {
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

    /// Get instrument information.
    pub async fn get_instrument(&self, params: &InstrumentParams) -> WebullResult<Vec<Instrument>> {
        self.base.post("/api/quote/instruments", params).await
    }

    /// Helper method to get instrument information for a single stock symbol.
    pub async fn get_stock_instrument(&self, symbol: &str) -> WebullResult<Vec<Instrument>> {
        let params = InstrumentParams::new_stock(symbol);
        self.get_instrument(&params).await
    }

    /// Helper method to get instrument information for multiple stock symbols.
    pub async fn get_stock_instruments(&self, symbols: &[&str]) -> WebullResult<Vec<Instrument>> {
        let params = InstrumentParams::new_stocks(symbols);
        self.get_instrument(&params).await
    }

    /// Get end-of-day bars for instruments.
    /// Only available for Webull JP.
    pub async fn get_eod_bar(&self, params: &EodBarsParams) -> WebullResult<Vec<Bar>> {
        self.base.post("/api/quote/eod/bars", params).await
    }

    /// Helper method to get end-of-day bars for an instrument.
    pub async fn get_instrument_eod_bars(
        &self,
        instrument_id: &str,
        count: u32,
    ) -> WebullResult<Vec<Bar>> {
        let params = EodBarsParams::new(instrument_id, count);
        self.get_eod_bar(&params).await
    }

    /// Helper method to get end-of-day bars for an instrument with a specific date.
    pub async fn get_instrument_eod_bars_with_date(
        &self,
        instrument_id: &str,
        date: &str,
        count: u32,
    ) -> WebullResult<Vec<Bar>> {
        let params = EodBarsParams::new(instrument_id, count).date(date);
        self.get_eod_bar(&params).await
    }

    /// Get corporate actions for instruments.
    /// Only available for Webull JP.
    pub async fn get_corp_action(
        &self,
        params: &CorpActionParams,
    ) -> WebullResult<Vec<Instrument>> {
        self.base.post("/api/quote/corp/action", params).await
    }

    /// Helper method to get stock split corporate actions for an instrument.
    pub async fn get_stock_splits(&self, instrument_id: &str) -> WebullResult<Vec<Instrument>> {
        let params = CorpActionParams::new(instrument_id, vec![CorpActionEventType::Split]);
        self.get_corp_action(&params).await
    }

    /// Helper method to get reverse stock split corporate actions for an instrument.
    pub async fn get_reverse_stock_splits(
        &self,
        instrument_id: &str,
    ) -> WebullResult<Vec<Instrument>> {
        let params = CorpActionParams::new(instrument_id, vec![CorpActionEventType::ReverseSplit]);
        self.get_corp_action(&params).await
    }

    /// Helper method to get all corporate actions for an instrument.
    pub async fn get_all_corp_actions(&self, instrument_id: &str) -> WebullResult<Vec<Instrument>> {
        let params = CorpActionParams::new(
            instrument_id,
            vec![
                CorpActionEventType::Split,
                CorpActionEventType::ReverseSplit,
            ],
        );
        self.get_corp_action(&params).await
    }

    /// Helper method to get daily bars for a symbol.
    pub async fn get_daily_bars(&self, symbol: &str, count: Option<u32>) -> WebullResult<Vec<Bar>> {
        let params = if let Some(count) = count {
            BarQueryParams::new(symbol, "STK", TimeFrame::Day1, count)
        } else {
            BarQueryParams::new_stock(symbol, TimeFrame::Day1)
        };

        self.get_history_bar(&params).await
    }

    /// Helper method to get intraday bars for a symbol.
    pub async fn get_intraday_bars(
        &self,
        symbol: &str,
        time_frame: TimeFrame,
        count: Option<u32>,
    ) -> WebullResult<Vec<Bar>> {
        let params = if let Some(count) = count {
            BarQueryParams::new(symbol, "STK", time_frame, count)
        } else {
            BarQueryParams::new_stock(symbol, time_frame)
        };

        self.get_history_bar(&params).await
    }
}
