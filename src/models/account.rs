use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

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
}

/// Position in an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Symbol of the position
    pub symbol: String,
    
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
}
