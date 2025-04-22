use crate::auth::{AccessToken, AuthManager};
use crate::error::{WebullError, WebullResult};
use crate::streaming::events::{
    ConnectionState, ConnectionStatus, ErrorEvent, Event, EventType, HeartbeatEvent,
};
use crate::streaming::subscription::{SubscriptionRequest, UnsubscriptionRequest};
use crate::utils::serialization::{from_json, to_json};
use futures_util::{SinkExt, StreamExt};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::net::TcpStream;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio::time::sleep;
use tokio_tungstenite::{
    connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream,
};
use url::Url;
use uuid::Uuid;

/// WebSocket client for streaming data from Webull.
pub struct WebSocketClient {
    /// Base URL for WebSocket connections
    base_url: String,

    /// Authentication manager
    auth_manager: Arc<AuthManager>,

    /// Connection state
    connection_state: Arc<Mutex<ConnectionState>>,

    /// Event sender
    event_sender: Option<Sender<Event>>,

    /// Last heartbeat time
    last_heartbeat: Arc<Mutex<Instant>>,

    /// Heartbeat interval in seconds
    heartbeat_interval: u64,

    /// Reconnect attempts
    reconnect_attempts: Arc<Mutex<u32>>,

    /// Maximum reconnect attempts
    max_reconnect_attempts: u32,

    /// Reconnect delay in seconds
    reconnect_delay: u64,
}

impl WebSocketClient {
    /// Create a new WebSocket client.
    pub fn new(base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base_url,
            auth_manager,
            connection_state: Arc::new(Mutex::new(ConnectionState::Disconnected)),
            event_sender: None,
            last_heartbeat: Arc::new(Mutex::new(Instant::now())),
            heartbeat_interval: 30,
            reconnect_attempts: Arc::new(Mutex::new(0)),
            max_reconnect_attempts: 5,
            reconnect_delay: 5,
        }
    }

    /// Connect to the WebSocket server.
    pub async fn connect(&mut self) -> WebullResult<Receiver<Event>> {
        // Create a channel for events
        let (tx, rx) = mpsc::channel(100);
        self.event_sender = Some(tx.clone());

        // Set the connection state to reconnecting
        *self.connection_state.lock().unwrap() = ConnectionState::Reconnecting;

        // Reset reconnect attempts
        *self.reconnect_attempts.lock().unwrap() = 0;

        // Start the connection task
        let base_url = self.base_url.clone();
        let auth_manager = self.auth_manager.clone();
        let connection_state = self.connection_state.clone();
        let last_heartbeat = self.last_heartbeat.clone();
        let heartbeat_interval = self.heartbeat_interval;
        let reconnect_attempts = self.reconnect_attempts.clone();
        let max_reconnect_attempts = self.max_reconnect_attempts;
        let reconnect_delay = self.reconnect_delay;

        tokio::spawn(async move {
            loop {
                // Check if we've exceeded the maximum reconnect attempts
                let attempts = *reconnect_attempts.lock().unwrap();
                if attempts > max_reconnect_attempts {
                    // Send a connection failed event
                    let event = Event {
                        event_type: EventType::Connection,
                        timestamp: chrono::Utc::now(),
                        data: crate::streaming::events::EventData::Connection(ConnectionStatus {
                            status: ConnectionState::Failed,
                            connection_id: None,
                            message: Some("Maximum reconnect attempts exceeded".to_string()),
                        }),
                    };

                    let _ = tx.send(event).await;

                    // Set the connection state to failed
                    *connection_state.lock().unwrap() = ConnectionState::Failed;

                    break;
                }

                // Increment reconnect attempts
                *reconnect_attempts.lock().unwrap() = attempts + 1;

                // Get the authentication token
                let token = match auth_manager.get_token().await {
                    Ok(token) => token,
                    Err(e) => {
                        // Send an error event
                        let event = Event {
                            event_type: EventType::Error,
                            timestamp: chrono::Utc::now(),
                            data: crate::streaming::events::EventData::Error(ErrorEvent {
                                code: "AUTH_ERROR".to_string(),
                                message: format!("Authentication error: {}", e),
                            }),
                        };

                        let _ = tx.send(event).await;

                        // Wait before retrying
                        sleep(Duration::from_secs(reconnect_delay)).await;
                        continue;
                    }
                };

                // Connect to the WebSocket server
                match Self::connect_websocket(&base_url, &token).await {
                    Ok(ws_stream) => {
                        // Set the connection state to connected
                        *connection_state.lock().unwrap() = ConnectionState::Connected;

                        // Reset reconnect attempts
                        *reconnect_attempts.lock().unwrap() = 0;

                        // Send a connection established event
                        let connection_id = Uuid::new_v4().to_string();
                        let event = Event {
                            event_type: EventType::Connection,
                            timestamp: chrono::Utc::now(),
                            data: crate::streaming::events::EventData::Connection(
                                ConnectionStatus {
                                    status: ConnectionState::Connected,
                                    connection_id: Some(connection_id.clone()),
                                    message: Some("Connection established".to_string()),
                                },
                            ),
                        };

                        let _ = tx.send(event).await;

                        // Handle the WebSocket connection
                        if let Err(e) = Self::handle_websocket(
                            ws_stream,
                            tx.clone(),
                            last_heartbeat.clone(),
                            heartbeat_interval,
                        )
                        .await
                        {
                            // Send an error event
                            let event = Event {
                                event_type: EventType::Error,
                                timestamp: chrono::Utc::now(),
                                data: crate::streaming::events::EventData::Error(ErrorEvent {
                                    code: "WS_ERROR".to_string(),
                                    message: format!("WebSocket error: {}", e),
                                }),
                            };

                            let _ = tx.send(event).await;
                        }

                        // Set the connection state to disconnected
                        *connection_state.lock().unwrap() = ConnectionState::Disconnected;

                        // Send a disconnection event
                        let event = Event {
                            event_type: EventType::Connection,
                            timestamp: chrono::Utc::now(),
                            data: crate::streaming::events::EventData::Connection(
                                ConnectionStatus {
                                    status: ConnectionState::Disconnected,
                                    connection_id: Some(connection_id),
                                    message: Some("Connection closed".to_string()),
                                },
                            ),
                        };

                        let _ = tx.send(event).await;
                    }
                    Err(e) => {
                        // Send an error event
                        let event = Event {
                            event_type: EventType::Error,
                            timestamp: chrono::Utc::now(),
                            data: crate::streaming::events::EventData::Error(ErrorEvent {
                                code: "WS_CONNECT_ERROR".to_string(),
                                message: format!("WebSocket connection error: {}", e),
                            }),
                        };

                        let _ = tx.send(event).await;
                    }
                }

                // Wait before reconnecting
                sleep(Duration::from_secs(reconnect_delay)).await;

                // Set the connection state to reconnecting
                *connection_state.lock().unwrap() = ConnectionState::Reconnecting;

                // Send a reconnecting event
                let event = Event {
                    event_type: EventType::Connection,
                    timestamp: chrono::Utc::now(),
                    data: crate::streaming::events::EventData::Connection(ConnectionStatus {
                        status: ConnectionState::Reconnecting,
                        connection_id: None,
                        message: Some("Reconnecting...".to_string()),
                    }),
                };

                let _ = tx.send(event).await;
            }
        });

        Ok(rx)
    }

    /// Disconnect from the WebSocket server.
    pub async fn disconnect(&mut self) -> WebullResult<()> {
        // Set the connection state to disconnected
        *self.connection_state.lock().unwrap() = ConnectionState::Disconnected;

        // Reset reconnect attempts
        *self.reconnect_attempts.lock().unwrap() = self.max_reconnect_attempts + 1;

        Ok(())
    }

    /// Subscribe to a topic.
    pub async fn subscribe(&self, request: SubscriptionRequest) -> WebullResult<()> {
        // Check if we're connected
        if *self.connection_state.lock().unwrap() != ConnectionState::Connected {
            return Err(WebullError::InvalidRequest(
                "Not connected to WebSocket server".to_string(),
            ));
        }

        // Send the subscription request
        let message = json!({
            "action": "SUBSCRIBE",
            "request": request,
        });

        // Send the message
        if let Some(tx) = &self.event_sender {
            let _message_str = to_json(&message)?;

            // Create a heartbeat event
            let event = Event {
                event_type: EventType::Heartbeat,
                timestamp: chrono::Utc::now(),
                data: crate::streaming::events::EventData::Heartbeat(HeartbeatEvent {
                    id: Uuid::new_v4().to_string(),
                }),
            };

            tx.send(event).await.map_err(|e| {
                WebullError::InvalidRequest(format!("Failed to send message: {}", e))
            })?;
        }

        Ok(())
    }

    /// Unsubscribe from a topic.
    pub async fn unsubscribe(&self, request: UnsubscriptionRequest) -> WebullResult<()> {
        // Check if we're connected
        if *self.connection_state.lock().unwrap() != ConnectionState::Connected {
            return Err(WebullError::InvalidRequest(
                "Not connected to WebSocket server".to_string(),
            ));
        }

        // Send the unsubscription request
        let message = json!({
            "action": "UNSUBSCRIBE",
            "request": request,
        });

        // Send the message
        if let Some(tx) = &self.event_sender {
            let _message_str = to_json(&message)?;

            // Create a heartbeat event
            let event = Event {
                event_type: EventType::Heartbeat,
                timestamp: chrono::Utc::now(),
                data: crate::streaming::events::EventData::Heartbeat(HeartbeatEvent {
                    id: Uuid::new_v4().to_string(),
                }),
            };

            tx.send(event).await.map_err(|e| {
                WebullError::InvalidRequest(format!("Failed to send message: {}", e))
            })?;
        }

        Ok(())
    }

    /// Connect to the WebSocket server.
    async fn connect_websocket(
        base_url: &str,
        token: &AccessToken,
    ) -> WebullResult<WebSocketStream<MaybeTlsStream<TcpStream>>> {
        // Create the WebSocket URL
        let ws_url = format!("{}/ws", base_url.replace("http", "ws"));
        let url = Url::parse(&ws_url)
            .map_err(|e| WebullError::InvalidRequest(format!("Invalid WebSocket URL: {}", e)))?;

        // Create the request headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token.token)).unwrap(),
        );

        // Connect to the WebSocket server
        let (ws_stream, _) = connect_async(url).await.map_err(|e| {
            WebullError::InvalidRequest(format!("WebSocket connection error: {}", e))
        })?;

        Ok(ws_stream)
    }

    /// Handle the WebSocket connection.
    async fn handle_websocket(
        mut ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
        tx: Sender<Event>,
        last_heartbeat: Arc<Mutex<Instant>>,
        heartbeat_interval: u64,
    ) -> WebullResult<()> {
        // Start the heartbeat task
        let tx_clone = tx.clone();
        let last_heartbeat_clone = last_heartbeat.clone();

        tokio::spawn(async move {
            loop {
                // Sleep for the heartbeat interval
                sleep(Duration::from_secs(heartbeat_interval)).await;

                // Check if we need to send a heartbeat
                let now = Instant::now();
                let last = *last_heartbeat_clone.lock().unwrap();

                if now.duration_since(last).as_secs() >= heartbeat_interval {
                    // Create a heartbeat message
                    let heartbeat = json!({
                        "type": "HEARTBEAT",
                        "id": Uuid::new_v4().to_string(),
                    });

                    // Send the heartbeat message
                    let _message = Message::Text(to_json(&heartbeat).unwrap());

                    // Create a heartbeat event
                    let event = Event {
                        event_type: EventType::Heartbeat,
                        timestamp: chrono::Utc::now(),
                        data: crate::streaming::events::EventData::Heartbeat(HeartbeatEvent {
                            id: Uuid::new_v4().to_string(),
                        }),
                    };

                    // Send the heartbeat event
                    if tx_clone.send(event).await.is_err() {
                        // Channel closed, exit the task
                        break;
                    }

                    // Update the last heartbeat time
                    *last_heartbeat_clone.lock().unwrap() = now;
                }
            }
        });

        // Handle incoming messages
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    // Parse the message
                    match from_json::<Event>(&text) {
                        Ok(event) => {
                            // Send the event
                            if tx.send(event).await.is_err() {
                                // Channel closed, exit the loop
                                break;
                            }
                        }
                        Err(e) => {
                            // Send an error event
                            let event = Event {
                                event_type: EventType::Error,
                                timestamp: chrono::Utc::now(),
                                data: crate::streaming::events::EventData::Error(ErrorEvent {
                                    code: "PARSE_ERROR".to_string(),
                                    message: format!("Failed to parse message: {}", e),
                                }),
                            };

                            if tx.send(event).await.is_err() {
                                // Channel closed, exit the loop
                                break;
                            }
                        }
                    }
                }
                Ok(Message::Binary(_)) => {
                    // Ignore binary messages
                }
                Ok(Message::Ping(data)) => {
                    // Respond with a pong
                    if let Err(e) = ws_stream.send(Message::Pong(data)).await {
                        // Send an error event
                        let event = Event {
                            event_type: EventType::Error,
                            timestamp: chrono::Utc::now(),
                            data: crate::streaming::events::EventData::Error(ErrorEvent {
                                code: "PONG_ERROR".to_string(),
                                message: format!("Failed to send pong: {}", e),
                            }),
                        };

                        if tx.send(event).await.is_err() {
                            // Channel closed, exit the loop
                            break;
                        }
                    }

                    // Update the last heartbeat time
                    *last_heartbeat.lock().unwrap() = Instant::now();
                }
                Ok(Message::Pong(_)) => {
                    // Update the last heartbeat time
                    *last_heartbeat.lock().unwrap() = Instant::now();
                }
                Ok(Message::Close(_)) => {
                    // Connection closed
                    break;
                }
                Ok(Message::Frame(_)) => {
                    // Ignore frame messages
                }
                Err(e) => {
                    // Send an error event
                    let event = Event {
                        event_type: EventType::Error,
                        timestamp: chrono::Utc::now(),
                        data: crate::streaming::events::EventData::Error(ErrorEvent {
                            code: "WS_ERROR".to_string(),
                            message: format!("WebSocket error: {}", e),
                        }),
                    };

                    if tx.send(event).await.is_err() {
                        // Channel closed, exit the loop
                        break;
                    }

                    // Exit the loop on error
                    break;
                }
            }
        }

        Ok(())
    }
}
