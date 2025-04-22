use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Parameters for querying snapshot data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotParams {
    /// Symbols to query (comma-separated)
    #[serde(rename = "symbols")]
    pub symbols: String,

    /// Security category (e.g., "STK" for stocks)
    #[serde(rename = "category")]
    pub category: String,
}

impl SnapshotParams {
    /// Create new snapshot query parameters.
    pub fn new(symbols: impl Into<String>, category: impl Into<String>) -> Self {
        Self {
            symbols: symbols.into(),
            category: category.into(),
        }
    }

    /// Create new snapshot query parameters for stocks.
    pub fn new_stock(symbols: impl Into<String>) -> Self {
        Self::new(symbols, "STK")
    }

    /// Create new snapshot query parameters for multiple stock symbols.
    pub fn new_stocks(symbols: &[&str]) -> Self {
        let symbols_str = symbols.join(",");
        Self::new_stock(symbols_str)
    }
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
    #[serde(rename = "m1")]
    Minute1,

    /// 5-minute bars
    #[serde(rename = "m5")]
    Minute5,

    /// 15-minute bars
    #[serde(rename = "m15")]
    Minute15,

    /// 30-minute bars
    #[serde(rename = "m30")]
    Minute30,

    /// 1-hour bars
    #[serde(rename = "h1")]
    Hour1,

    /// 4-hour bars
    #[serde(rename = "h4")]
    Hour4,

    /// Daily bars
    #[serde(rename = "d1")]
    Day1,

    /// Weekly bars
    #[serde(rename = "w1")]
    Week1,

    /// Monthly bars
    #[serde(rename = "mo1")]
    Month1,
}

/// Parameters for querying historical bar data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarQueryParams {
    /// Symbol to query
    #[serde(rename = "symbol")]
    pub symbol: String,

    /// Security category (e.g., "STK" for stocks)
    #[serde(rename = "category")]
    pub category: String,

    /// Time frame for the bars
    #[serde(rename = "timespan")]
    pub time_frame: TimeFrame,

    /// Number of bars to return (max 1200)
    #[serde(rename = "count")]
    pub count: String,
}

impl BarQueryParams {
    /// Create new bar query parameters.
    pub fn new(
        symbol: impl Into<String>,
        category: impl Into<String>,
        time_frame: TimeFrame,
        count: u32,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            category: category.into(),
            time_frame,
            count: count.to_string(),
        }
    }

    /// Create new bar query parameters for stocks with default count of 200.
    pub fn new_stock(symbol: impl Into<String>, time_frame: TimeFrame) -> Self {
        Self::new(symbol, "STK", time_frame, 200)
    }

    /// Set the count of bars to return.
    pub fn count(mut self, count: u32) -> Self {
        self.count = count.to_string();
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

/// Instrument information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instrument {
    /// Instrument ID
    pub id: String,

    /// Symbol
    pub symbol: String,

    /// Name
    pub name: String,

    /// Exchange
    pub exchange: String,

    /// Security type
    pub security_type: String,

    /// Region
    pub region: String,

    /// Currency
    pub currency: String,

    /// Is tradable
    pub tradable: bool,

    /// Is shortable
    pub shortable: bool,

    /// Is marginable
    pub marginable: bool,

    /// Is fractional tradable
    pub fractional_tradable: bool,
}

/// Parameters for querying instrument data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstrumentParams {
    /// Symbols to query (comma-separated)
    #[serde(rename = "symbols")]
    pub symbols: String,

    /// Security category (e.g., "STK" for stocks)
    #[serde(rename = "category")]
    pub category: String,
}

impl InstrumentParams {
    /// Create new instrument query parameters.
    pub fn new(symbols: impl Into<String>, category: impl Into<String>) -> Self {
        Self {
            symbols: symbols.into(),
            category: category.into(),
        }
    }

    /// Create new instrument query parameters for stocks.
    pub fn new_stock(symbols: impl Into<String>) -> Self {
        Self::new(symbols, "STK")
    }

    /// Create new instrument query parameters for multiple stock symbols.
    pub fn new_stocks(symbols: &[&str]) -> Self {
        let symbols_str = symbols.join(",");
        Self::new_stock(symbols_str)
    }
}

/// Parameters for querying end-of-day bars.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EodBarsParams {
    /// Instrument IDs (comma-separated)
    #[serde(rename = "instrument_ids")]
    pub instrument_ids: String,

    /// Date (UTC, format: yyyy-MM-dd)
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,

    /// Number of bars to return (max 800)
    #[serde(rename = "count")]
    pub count: String,
}

impl EodBarsParams {
    /// Create new EOD bars query parameters.
    pub fn new(instrument_ids: impl Into<String>, count: u32) -> Self {
        Self {
            instrument_ids: instrument_ids.into(),
            date: None,
            count: count.to_string(),
        }
    }

    /// Set the date filter.
    pub fn date(mut self, date: impl Into<String>) -> Self {
        self.date = Some(date.into());
        self
    }
}

/// Corporate action event type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CorpActionEventType {
    /// Stock split
    #[serde(rename = "SPLIT")]
    Split,

    /// Reverse stock split
    #[serde(rename = "REVERSE_SPLIT")]
    ReverseSplit,
}

/// Parameters for querying corporate actions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpActionParams {
    /// Instrument IDs (comma-separated)
    #[serde(rename = "instrument_ids")]
    pub instrument_ids: String,

    /// Event types (comma-separated)
    #[serde(rename = "event_types")]
    pub event_types: String,

    /// Start date (UTC, format: yyyy-MM-dd)
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,

    /// End date (UTC, format: yyyy-MM-dd)
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,

    /// Page number
    #[serde(rename = "page_number", skip_serializing_if = "Option::is_none")]
    pub page_number: Option<u32>,

    /// Page size (max 200)
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,

    /// Last update time (UTC, format: yyyy-MM-dd HH:mm:ss)
    #[serde(rename = "last_update_time", skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
}

impl CorpActionParams {
    /// Create new corporate action query parameters.
    pub fn new(instrument_ids: impl Into<String>, event_types: Vec<CorpActionEventType>) -> Self {
        let event_types_str = event_types
            .iter()
            .map(|et| match et {
                CorpActionEventType::Split => "SPLIT",
                CorpActionEventType::ReverseSplit => "REVERSE_SPLIT",
            })
            .collect::<Vec<_>>()
            .join(",");

        Self {
            instrument_ids: instrument_ids.into(),
            event_types: event_types_str,
            start_date: None,
            end_date: None,
            page_number: None,
            page_size: None,
            last_update_time: None,
        }
    }

    /// Set the start date filter.
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = Some(start_date.into());
        self
    }

    /// Set the end date filter.
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = Some(end_date.into());
        self
    }

    /// Set the page number.
    pub fn page_number(mut self, page_number: u32) -> Self {
        self.page_number = Some(page_number);
        self
    }

    /// Set the page size.
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// Set the last update time.
    pub fn last_update_time(mut self, last_update_time: impl Into<String>) -> Self {
        self.last_update_time = Some(last_update_time.into());
        self
    }
}
