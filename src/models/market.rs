use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Real-time quote information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quote {
    /// Symbol of the security
    pub symbol: String,
    
    /// Last trade price
    pub last_price: f64,
    
    /// Change in price
    pub change: f64,
    
    /// Percentage change in price
    pub change_percent: f64,
    
    /// Volume of shares traded
    pub volume: u64,
    
    /// Average volume
    pub average_volume: u64,
    
    /// Bid price
    pub bid_price: f64,
    
    /// Bid size
    pub bid_size: u64,
    
    /// Ask price
    pub ask_price: f64,
    
    /// Ask size
    pub ask_size: u64,
    
    /// Day's high price
    pub high: f64,
    
    /// Day's low price
    pub low: f64,
    
    /// Opening price
    pub open: f64,
    
    /// Previous close price
    pub prev_close: f64,
    
    /// 52-week high price
    pub fifty_two_week_high: f64,
    
    /// 52-week low price
    pub fifty_two_week_low: f64,
    
    /// Market cap
    pub market_cap: Option<f64>,
    
    /// Price-to-earnings ratio
    pub pe_ratio: Option<f64>,
    
    /// Timestamp of the quote
    pub timestamp: DateTime<Utc>,
}

/// Bar data for a security.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bar {
    /// Symbol of the security
    pub symbol: String,
    
    /// Opening price
    pub open: f64,
    
    /// High price
    pub high: f64,
    
    /// Low price
    pub low: f64,
    
    /// Closing price
    pub close: f64,
    
    /// Volume of shares traded
    pub volume: u64,
    
    /// Timestamp of the bar
    pub timestamp: DateTime<Utc>,
}

/// Time frame for bar data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TimeFrame {
    /// 1-minute bars
    Minute1,
    
    /// 5-minute bars
    Minute5,
    
    /// 15-minute bars
    Minute15,
    
    /// 30-minute bars
    Minute30,
    
    /// 1-hour bars
    Hour1,
    
    /// 4-hour bars
    Hour4,
    
    /// Daily bars
    Day1,
    
    /// Weekly bars
    Week1,
    
    /// Monthly bars
    Month1,
}

/// Parameters for querying bar data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarQueryParams {
    /// Symbol to query
    pub symbol: String,
    
    /// Time frame for the bars
    pub time_frame: TimeFrame,
    
    /// Start date for the query
    pub start_date: Option<DateTime<Utc>>,
    
    /// End date for the query
    pub end_date: Option<DateTime<Utc>>,
    
    /// Maximum number of bars to return
    pub limit: Option<u32>,
}

impl BarQueryParams {
    /// Create new bar query parameters.
    pub fn new(symbol: impl Into<String>, time_frame: TimeFrame) -> Self {
        Self {
            symbol: symbol.into(),
            time_frame,
            start_date: None,
            end_date: None,
            limit: None,
        }
    }
    
    /// Set the start date filter.
    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }
    
    /// Set the end date filter.
    pub fn end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }
    
    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Option contract information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionContract {
    /// Symbol of the option contract
    pub symbol: String,
    
    /// Underlying symbol
    pub underlying_symbol: String,
    
    /// Strike price
    pub strike_price: f64,
    
    /// Expiration date
    pub expiration_date: DateTime<Utc>,
    
    /// Option type (call/put)
    pub option_type: OptionType,
    
    /// Last trade price
    pub last_price: f64,
    
    /// Change in price
    pub change: f64,
    
    /// Percentage change in price
    pub change_percent: f64,
    
    /// Volume of contracts traded
    pub volume: u64,
    
    /// Open interest
    pub open_interest: u64,
    
    /// Bid price
    pub bid_price: f64,
    
    /// Bid size
    pub bid_size: u64,
    
    /// Ask price
    pub ask_price: f64,
    
    /// Ask size
    pub ask_size: u64,
    
    /// Implied volatility
    pub implied_volatility: f64,
    
    /// Delta
    pub delta: f64,
    
    /// Gamma
    pub gamma: f64,
    
    /// Theta
    pub theta: f64,
    
    /// Vega
    pub vega: f64,
    
    /// Rho
    pub rho: f64,
}

/// Type of option contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OptionType {
    /// Call option
    Call,
    
    /// Put option
    Put,
}

/// Option chain information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChain {
    /// Underlying symbol
    pub underlying_symbol: String,
    
    /// Expiration dates
    pub expiration_dates: Vec<DateTime<Utc>>,
    
    /// Strike prices
    pub strike_prices: Vec<f64>,
    
    /// Option contracts
    pub contracts: Vec<OptionContract>,
}

/// Parameters for querying option chains.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionChainQueryParams {
    /// Underlying symbol
    pub underlying_symbol: String,
    
    /// Expiration date filter
    pub expiration_date: Option<DateTime<Utc>>,
    
    /// Strike price filter
    pub strike_price: Option<f64>,
    
    /// Option type filter
    pub option_type: Option<OptionType>,
}

impl OptionChainQueryParams {
    /// Create new option chain query parameters.
    pub fn new(underlying_symbol: impl Into<String>) -> Self {
        Self {
            underlying_symbol: underlying_symbol.into(),
            expiration_date: None,
            strike_price: None,
            option_type: None,
        }
    }
    
    /// Set the expiration date filter.
    pub fn expiration_date(mut self, expiration_date: DateTime<Utc>) -> Self {
        self.expiration_date = Some(expiration_date);
        self
    }
    
    /// Set the strike price filter.
    pub fn strike_price(mut self, strike_price: f64) -> Self {
        self.strike_price = Some(strike_price);
        self
    }
    
    /// Set the option type filter.
    pub fn option_type(mut self, option_type: OptionType) -> Self {
        self.option_type = Some(option_type);
        self
    }
}

/// Market news article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsArticle {
    /// Article ID
    pub id: String,
    
    /// Article title
    pub title: String,
    
    /// Article summary
    pub summary: String,
    
    /// Article URL
    pub url: String,
    
    /// Article source
    pub source: String,
    
    /// Article publish date
    pub publish_date: DateTime<Utc>,
    
    /// Related symbols
    pub symbols: Vec<String>,
}

/// Parameters for querying news articles.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewsQueryParams {
    /// Symbol filter
    pub symbol: Option<String>,
    
    /// Start date filter
    pub start_date: Option<DateTime<Utc>>,
    
    /// End date filter
    pub end_date: Option<DateTime<Utc>>,
    
    /// Maximum number of articles to return
    pub limit: Option<u32>,
}

impl NewsQueryParams {
    /// Create new news query parameters.
    pub fn new() -> Self {
        Self {
            symbol: None,
            start_date: None,
            end_date: None,
            limit: None,
        }
    }
    
    /// Set the symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
    
    /// Set the start date filter.
    pub fn start_date(mut self, start_date: DateTime<Utc>) -> Self {
        self.start_date = Some(start_date);
        self
    }
    
    /// Set the end date filter.
    pub fn end_date(mut self, end_date: DateTime<Utc>) -> Self {
        self.end_date = Some(end_date);
        self
    }
    
    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Default for NewsQueryParams {
    fn default() -> Self {
        Self::new()
    }
}
