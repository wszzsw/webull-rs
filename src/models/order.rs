use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Order information from Webull.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Order ID
    pub id: String,

    /// Symbol of the security
    pub symbol: String,

    /// Quantity of shares
    pub quantity: f64,

    /// Filled quantity of shares
    pub filled_quantity: f64,

    /// Price of the order (for limit orders)
    pub price: Option<f64>,

    /// Stop price (for stop orders)
    pub stop_price: Option<f64>,

    /// Order status
    pub status: OrderStatus,

    /// Order side (buy/sell)
    pub side: OrderSide,

    /// Order type
    pub order_type: OrderType,

    /// Time in force
    pub time_in_force: TimeInForce,

    /// Whether the order is for extended hours trading
    pub extended_hours: bool,

    /// When the order was created
    pub created_at: DateTime<Utc>,

    /// When the order was last updated
    pub updated_at: DateTime<Utc>,

    /// Commission charged for the order
    pub commission: f64,

    /// Rejected reason (if the order was rejected)
    pub rejected_reason: Option<String>,

    /// Average fill price
    pub average_fill_price: Option<f64>,
}

/// Status of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderStatus {
    /// Order is new
    New,

    /// Order is partially filled
    PartiallyFilled,

    /// Order is filled
    Filled,

    /// Order is canceled
    Canceled,

    /// Order is rejected
    Rejected,

    /// Order is pending cancel
    PendingCancel,

    /// Order is pending new
    PendingNew,

    /// Order is pending replace
    PendingReplace,

    /// Order is replaced
    Replaced,

    /// Order is suspended
    Suspended,

    /// Order is expired
    Expired,
}

/// Side of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderSide {
    /// Buy order
    Buy,

    /// Sell order
    Sell,

    /// Sell short order
    SellShort,

    /// Buy to cover order
    BuyToCover,
}

/// Type of an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderType {
    /// Market order
    #[serde(rename = "MARKET")]
    Market,

    /// Limit order
    #[serde(rename = "LIMIT")]
    Limit,

    /// Stop order
    #[serde(rename = "STOP_LOSS")]
    Stop,

    /// Stop limit order
    #[serde(rename = "STOP_LOSS_LIMIT")]
    StopLimit,

    /// Trailing stop order
    #[serde(rename = "TRAILING_STOP")]
    TrailingStop,

    /// Trailing stop limit order
    #[serde(rename = "TRAILING_STOP_LIMIT")]
    TrailingStopLimit,

    /// Enhanced limit order (for Hong Kong stocks)
    #[serde(rename = "ENHANCED_LIMIT")]
    EnhancedLimit,

    /// At auction order (for Hong Kong stocks)
    #[serde(rename = "AT_AUCTION")]
    AtAuction,

    /// At auction limit order (for Hong Kong stocks)
    #[serde(rename = "AT_AUCTION_LIMIT")]
    AtAuctionLimit,
}

/// Time in force for an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Day order
    #[serde(rename = "DAY")]
    Day,

    /// Good till canceled order
    #[serde(rename = "GTC")]
    Gtc,

    /// Good till date order
    #[serde(rename = "GTD")]
    Gtd,

    /// Immediate or cancel order
    #[serde(rename = "IOC")]
    Ioc,

    /// Fill or kill order
    #[serde(rename = "FOK")]
    Fok,
}

/// Trailing stop type for an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TrailingStopType {
    /// Amount in currency
    #[serde(rename = "AMOUNT")]
    Amount,

    /// Percentage
    #[serde(rename = "PERCENT")]
    Percent,
}

/// Request to place an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderRequest {
    /// Symbol of the security
    pub symbol: String,

    /// Quantity of shares
    pub quantity: f64,

    /// Price of the order (for limit orders)
    pub price: Option<f64>,

    /// Stop price (for stop orders)
    pub stop_price: Option<f64>,

    /// Order side (buy/sell)
    pub side: OrderSide,

    /// Order type
    pub order_type: OrderType,

    /// Time in force
    pub time_in_force: TimeInForce,

    /// Whether the order is for extended hours trading
    pub extended_hours: bool,

    /// Trailing stop type (for trailing stop orders)
    pub trailing_type: Option<TrailingStopType>,

    /// Trailing stop step (for trailing stop orders)
    pub trailing_stop_step: Option<f64>,

    /// Client order ID (for tracking purposes)
    pub client_order_id: Option<String>,

    /// Instrument ID (alternative to symbol)
    pub instrument_id: Option<String>,
}

impl OrderRequest {
    /// Create a new order request.
    pub fn new() -> Self {
        Self {
            symbol: String::new(),
            quantity: 0.0,
            price: None,
            stop_price: None,
            side: OrderSide::Buy,
            order_type: OrderType::Market,
            time_in_force: TimeInForce::Day,
            extended_hours: false,
            trailing_type: None,
            trailing_stop_step: None,
            client_order_id: None,
            instrument_id: None,
        }
    }

    /// Set the symbol.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = symbol.into();
        self
    }

    /// Set the quantity.
    pub fn quantity(mut self, quantity: f64) -> Self {
        self.quantity = quantity;
        self
    }

    /// Set the price.
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Set the stop price.
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }

    /// Set the order side.
    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = side;
        self
    }

    /// Set the order type.
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    /// Set the time in force.
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    /// Set whether the order is for extended hours trading.
    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    /// Set the trailing stop type.
    pub fn trailing_type(mut self, trailing_type: TrailingStopType) -> Self {
        self.trailing_type = Some(trailing_type);
        self
    }

    /// Set the trailing stop step.
    pub fn trailing_stop_step(mut self, trailing_stop_step: f64) -> Self {
        self.trailing_stop_step = Some(trailing_stop_step);
        self
    }

    /// Set the client order ID.
    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    /// Set the instrument ID.
    pub fn instrument_id(mut self, instrument_id: impl Into<String>) -> Self {
        self.instrument_id = Some(instrument_id.into());
        self
    }

    /// Create a market order.
    pub fn market() -> Self {
        Self::new().order_type(OrderType::Market)
    }

    /// Create a limit order.
    pub fn limit() -> Self {
        Self::new().order_type(OrderType::Limit)
    }

    /// Create a stop order.
    pub fn stop() -> Self {
        Self::new().order_type(OrderType::Stop)
    }

    /// Create a stop limit order.
    pub fn stop_limit() -> Self {
        Self::new().order_type(OrderType::StopLimit)
    }

    /// Create a trailing stop order.
    pub fn trailing_stop() -> Self {
        Self::new().order_type(OrderType::TrailingStop)
    }

    /// Create a trailing stop limit order.
    pub fn trailing_stop_limit() -> Self {
        Self::new().order_type(OrderType::TrailingStopLimit)
    }
}

impl Default for OrderRequest {
    fn default() -> Self {
        Self::new()
    }
}

/// Response from placing an order.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    /// Order ID
    pub id: String,

    /// Order status
    pub status: OrderStatus,

    /// Symbol of the security
    pub symbol: String,

    /// Quantity of shares
    pub quantity: f64,

    /// Price of the order (for limit orders)
    pub price: Option<f64>,

    /// Stop price (for stop orders)
    pub stop_price: Option<f64>,

    /// Order side (buy/sell)
    pub side: OrderSide,

    /// Order type
    pub order_type: OrderType,

    /// Time in force
    pub time_in_force: TimeInForce,

    /// Whether the order is for extended hours trading
    pub extended_hours: bool,

    /// When the order was created
    pub created_at: DateTime<Utc>,
}

/// Parameters for querying orders.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderQueryParams {
    /// Status of orders to query
    pub status: Option<OrderStatus>,

    /// Symbol to filter by
    pub symbol: Option<String>,

    /// Start date for the query
    pub start_date: Option<DateTime<Utc>>,

    /// End date for the query
    pub end_date: Option<DateTime<Utc>>,

    /// Maximum number of orders to return
    pub limit: Option<u32>,
}

impl OrderQueryParams {
    /// Create new order query parameters.
    pub fn new() -> Self {
        Self {
            status: None,
            symbol: None,
            start_date: None,
            end_date: None,
            limit: None,
        }
    }

    /// Set the status filter.
    pub fn status(mut self, status: OrderStatus) -> Self {
        self.status = Some(status);
        self
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

impl Default for OrderQueryParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Option order request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionOrderRequest {
    /// Client order ID
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,

    /// Option contract ID
    #[serde(rename = "contract_id")]
    pub contract_id: String,

    /// Quantity of contracts
    #[serde(rename = "qty")]
    pub quantity: f64,

    /// Order side (buy/sell)
    #[serde(rename = "side")]
    pub side: OrderSide,

    /// Order type
    #[serde(rename = "order_type")]
    pub order_type: OrderType,

    /// Time in force
    #[serde(rename = "tif")]
    pub time_in_force: TimeInForce,

    /// Whether the order is for extended hours trading
    #[serde(rename = "extended_hours_trading")]
    pub extended_hours: bool,

    /// Limit price (for limit orders)
    #[serde(rename = "limit_price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,

    /// Stop price (for stop orders)
    #[serde(rename = "stop_price", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
}

impl OptionOrderRequest {
    /// Create a new option order request.
    pub fn new(
        client_order_id: impl Into<String>,
        contract_id: impl Into<String>,
        quantity: f64,
    ) -> Self {
        Self {
            client_order_id: client_order_id.into(),
            contract_id: contract_id.into(),
            quantity,
            side: OrderSide::Buy,
            order_type: OrderType::Limit,
            time_in_force: TimeInForce::Day,
            extended_hours: false,
            price: None,
            stop_price: None,
        }
    }

    /// Set the order side.
    pub fn side(mut self, side: OrderSide) -> Self {
        self.side = side;
        self
    }

    /// Set the order type.
    pub fn order_type(mut self, order_type: OrderType) -> Self {
        self.order_type = order_type;
        self
    }

    /// Set the time in force.
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    /// Set whether the order is for extended hours trading.
    pub fn extended_hours(mut self, extended_hours: bool) -> Self {
        self.extended_hours = extended_hours;
        self
    }

    /// Set the price.
    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    /// Set the stop price.
    pub fn stop_price(mut self, stop_price: f64) -> Self {
        self.stop_price = Some(stop_price);
        self
    }
}

/// Option order preview request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionOrderPreviewRequest {
    /// Account ID
    #[serde(rename = "account_id")]
    pub account_id: String,

    /// New orders
    #[serde(rename = "new_orders")]
    pub new_orders: Vec<OptionOrderRequest>,
}

impl OptionOrderPreviewRequest {
    /// Create a new option order preview request.
    pub fn new(account_id: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            new_orders: Vec::new(),
        }
    }

    /// Add a new order to the preview request.
    pub fn add_order(mut self, order: OptionOrderRequest) -> Self {
        self.new_orders.push(order);
        self
    }
}

/// Option order preview response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionOrderPreviewResponse {
    /// Order ID
    pub id: String,

    /// Commission
    pub commission: f64,

    /// Estimated cost
    pub estimated_cost: f64,

    /// Estimated proceeds
    pub estimated_proceeds: f64,

    /// Buying power effect
    pub buying_power_effect: f64,

    /// Margin requirement
    pub margin_requirement: f64,

    /// Error message (if any)
    pub error_message: Option<String>,

    /// Warning message (if any)
    pub warning_message: Option<String>,
}
