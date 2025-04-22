use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Account information from Webull.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Account ID
    pub id: String,

    /// Account number
    pub account_number: String,

    /// Account type
    pub account_type: AccountType,

    /// Account status
    pub status: AccountStatus,

    /// When the account was created
    pub created_at: DateTime<Utc>,

    /// Currency of the account
    pub currency: String,

    /// Whether the account is a paper trading account
    pub paper_trading: bool,

    /// Account region
    pub region: Option<String>,

    /// Account name
    pub name: Option<String>,

    /// Account email
    pub email: Option<String>,

    /// Account phone number
    pub phone: Option<String>,
}

/// Type of account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountType {
    /// Cash account
    Cash,

    /// Margin account
    Margin,

    /// IRA account
    Ira,

    /// Other account type
    Other,
}

/// Status of an account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AccountStatus {
    /// Account is active
    Active,

    /// Account is closed
    Closed,

    /// Account is pending approval
    Pending,

    /// Account is suspended
    Suspended,
}

/// Account balance information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    /// Cash balance
    pub cash: f64,

    /// Buying power
    pub buying_power: f64,

    /// Market value of holdings
    pub market_value: f64,

    /// Total account value
    pub total_value: f64,

    /// Unrealized profit/loss
    pub unrealized_profit_loss: f64,

    /// Unrealized profit/loss percentage
    pub unrealized_profit_loss_percentage: f64,

    /// Currency of the balance
    pub currency: String,

    /// Settled cash
    pub settled_cash: Option<f64>,

    /// Unsettled cash
    pub unsettled_cash: Option<f64>,

    /// Cash available for withdrawal
    pub withdrawable_cash: Option<f64>,

    /// Cash available for trading
    pub tradable_cash: Option<f64>,

    /// Margin buying power
    pub margin_buying_power: Option<f64>,

    /// Option buying power
    pub option_buying_power: Option<f64>,

    /// Day trading buying power
    pub day_trading_buying_power: Option<f64>,
}

/// Position in an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Symbol of the position
    pub symbol: String,

    /// Instrument ID
    pub instrument_id: String,

    /// Quantity of shares
    pub quantity: f64,

    /// Average cost basis
    pub cost_basis: f64,

    /// Current market value
    pub market_value: f64,

    /// Unrealized profit/loss
    pub unrealized_profit_loss: f64,

    /// Unrealized profit/loss percentage
    pub unrealized_profit_loss_percentage: f64,

    /// Current price
    pub current_price: f64,

    /// When the position was opened
    pub opened_at: DateTime<Utc>,

    /// Security name
    pub name: Option<String>,

    /// Security type
    pub security_type: Option<String>,

    /// Exchange
    pub exchange: Option<String>,

    /// Currency
    pub currency: Option<String>,

    /// Position side (LONG or SHORT)
    pub side: Option<String>,

    /// Position status
    pub status: Option<String>,

    /// Quantity available for trading
    pub tradable_quantity: Option<f64>,

    /// Quantity not yet settled
    pub unsettled_quantity: Option<f64>,
}

/// Account profile information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountProfile {
    /// Account ID
    pub id: String,

    /// Account number
    pub account_number: String,

    /// Account type
    pub account_type: AccountType,

    /// Account status
    pub status: AccountStatus,

    /// Account region
    pub region: String,

    /// Account name
    pub name: String,

    /// Account email
    pub email: Option<String>,

    /// Account phone number
    pub phone: Option<String>,

    /// Account currency
    pub currency: String,

    /// Whether the account is a paper trading account
    pub paper_trading: bool,

    /// Account creation date
    pub created_at: DateTime<Utc>,

    /// Account KYC status
    pub kyc_status: Option<String>,

    /// Account risk level
    pub risk_level: Option<String>,

    /// Account permissions
    pub permissions: Option<Vec<String>>,
}

/// Trade history entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeHistory {
    /// Trade ID
    pub id: String,

    /// Symbol of the security
    pub symbol: String,

    /// Instrument ID
    pub instrument_id: String,

    /// Security name
    pub name: Option<String>,

    /// Trade action (BUY or SELL)
    pub action: String,

    /// Quantity of shares
    pub quantity: f64,

    /// Price per share
    pub price: f64,

    /// Total amount of the trade
    pub amount: f64,

    /// Trade fees
    pub fees: Option<f64>,

    /// Trade date and time
    pub trade_time: DateTime<Utc>,

    /// Trade status
    pub status: String,

    /// Order ID associated with the trade
    pub order_id: Option<String>,

    /// Trade currency
    pub currency: Option<String>,

    /// Exchange where the trade was executed
    pub exchange: Option<String>,

    /// Security type
    pub security_type: Option<String>,
}

/// Parameters for querying account positions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PositionParams {
    /// Account ID
    #[serde(rename = "account_id")]
    pub account_id: String,

    /// Page size (max 100)
    #[serde(rename = "page_size")]
    pub page_size: u32,

    /// Last instrument ID for pagination
    #[serde(rename = "last_instrument_id", skip_serializing_if = "Option::is_none")]
    pub last_instrument_id: Option<String>,
}

impl PositionParams {
    /// Create new position query parameters.
    pub fn new(account_id: impl Into<String>, page_size: u32) -> Self {
        Self {
            account_id: account_id.into(),
            page_size,
            last_instrument_id: None,
        }
    }

    /// Set the last instrument ID for pagination.
    pub fn last_instrument_id(mut self, last_instrument_id: impl Into<String>) -> Self {
        self.last_instrument_id = Some(last_instrument_id.into());
        self
    }
}

/// Parameters for querying account balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceParams {
    /// Account ID
    #[serde(rename = "account_id")]
    pub account_id: String,

    /// Currency for total assets
    #[serde(rename = "total_asset_currency")]
    pub total_asset_currency: String,
}

impl BalanceParams {
    /// Create new balance query parameters.
    pub fn new(account_id: impl Into<String>, total_asset_currency: impl Into<String>) -> Self {
        Self {
            account_id: account_id.into(),
            total_asset_currency: total_asset_currency.into(),
        }
    }

    /// Create new balance query parameters with default currency (HKD).
    pub fn new_with_default_currency(account_id: impl Into<String>) -> Self {
        Self::new(account_id, "HKD")
    }
}
