use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
    Market,
    
    /// Limit order
    Limit,
    
    /// Stop order
    Stop,
    
    /// Stop limit order
    StopLimit,
    
    /// Trailing stop order
    TrailingStop,
    
    /// Trailing stop limit order
    TrailingStopLimit,
}

/// Time in force for an order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    /// Day order
    Day,
    
    /// Good till canceled order
    Gtc,
    
    /// Good till date order
    Gtd,
    
    /// Immediate or cancel order
    Ioc,
    
    /// Fill or kill order
    Fok,
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
