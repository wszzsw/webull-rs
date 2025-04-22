use crate::models::market::Quote;
use crate::models::order::Order;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Event types for WebSocket messages.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum EventType {
    /// Quote update event
    Quote,
    
    /// Order update event
    Order,
    
    /// Account update event
    Account,
    
    /// Trade update event
    Trade,
    
    /// Connection status event
    Connection,
    
    /// Subscription status event
    Subscription,
    
    /// Error event
    Error,
    
    /// Heartbeat event
    Heartbeat,
    
    /// Unknown event
    Unknown,
}

/// WebSocket event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type
    #[serde(rename = "type")]
    pub event_type: EventType,
    
    /// Event timestamp
    pub timestamp: DateTime<Utc>,
    
    /// Event data
    #[serde(flatten)]
    pub data: EventData,
}

/// WebSocket event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum EventData {
    /// Quote update event data
    Quote(Quote),
    
    /// Order update event data
    Order(Order),
    
    /// Connection status event data
    Connection(ConnectionStatus),
    
    /// Subscription status event data
    Subscription(SubscriptionStatus),
    
    /// Error event data
    Error(ErrorEvent),
    
    /// Heartbeat event data
    Heartbeat(HeartbeatEvent),
    
    /// Unknown event data
    Unknown(serde_json::Value),
}

/// Connection status event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    /// Connection status
    pub status: ConnectionState,
    
    /// Connection ID
    pub connection_id: Option<String>,
    
    /// Connection message
    pub message: Option<String>,
}

/// Connection state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum ConnectionState {
    /// Connected
    Connected,
    
    /// Disconnected
    Disconnected,
    
    /// Reconnecting
    Reconnecting,
    
    /// Failed
    Failed,
}

/// Subscription status event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStatus {
    /// Subscription ID
    pub subscription_id: String,
    
    /// Subscription status
    pub status: SubscriptionState,
    
    /// Subscription message
    pub message: Option<String>,
}

/// Subscription state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum SubscriptionState {
    /// Subscribed
    Subscribed,
    
    /// Unsubscribed
    Unsubscribed,
    
    /// Failed
    Failed,
}

/// Error event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    /// Error code
    pub code: String,
    
    /// Error message
    pub message: String,
}

/// Heartbeat event data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatEvent {
    /// Heartbeat ID
    pub id: String,
}
