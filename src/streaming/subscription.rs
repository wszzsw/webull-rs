use serde::{Deserialize, Serialize};

/// Subscription type for WebSocket messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscriptionType {
    /// Quote subscription
    Quote,
    
    /// Order subscription
    Order,
    
    /// Account subscription
    Account,
    
    /// Trade subscription
    Trade,
}

/// Subscription request for WebSocket messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionRequest {
    /// Subscription type
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    
    /// Symbols to subscribe to (for quote subscriptions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    
    /// Account ID to subscribe to (for order, account, and trade subscriptions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl SubscriptionRequest {
    /// Create a new quote subscription request.
    pub fn new_quote(symbols: Vec<String>) -> Self {
        Self {
            subscription_type: SubscriptionType::Quote,
            symbols: Some(symbols),
            account_id: None,
        }
    }
    
    /// Create a new order subscription request.
    pub fn new_order(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Order,
            symbols: None,
            account_id: Some(account_id),
        }
    }
    
    /// Create a new account subscription request.
    pub fn new_account(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Account,
            symbols: None,
            account_id: Some(account_id),
        }
    }
    
    /// Create a new trade subscription request.
    pub fn new_trade(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Trade,
            symbols: None,
            account_id: Some(account_id),
        }
    }
}

/// Unsubscription request for WebSocket messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnsubscriptionRequest {
    /// Subscription type
    #[serde(rename = "type")]
    pub subscription_type: SubscriptionType,
    
    /// Symbols to unsubscribe from (for quote subscriptions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbols: Option<Vec<String>>,
    
    /// Account ID to unsubscribe from (for order, account, and trade subscriptions)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl UnsubscriptionRequest {
    /// Create a new quote unsubscription request.
    pub fn new_quote(symbols: Vec<String>) -> Self {
        Self {
            subscription_type: SubscriptionType::Quote,
            symbols: Some(symbols),
            account_id: None,
        }
    }
    
    /// Create a new order unsubscription request.
    pub fn new_order(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Order,
            symbols: None,
            account_id: Some(account_id),
        }
    }
    
    /// Create a new account unsubscription request.
    pub fn new_account(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Account,
            symbols: None,
            account_id: Some(account_id),
        }
    }
    
    /// Create a new trade unsubscription request.
    pub fn new_trade(account_id: String) -> Self {
        Self {
            subscription_type: SubscriptionType::Trade,
            symbols: None,
            account_id: Some(account_id),
        }
    }
}
