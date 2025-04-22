use crate::auth::AuthManager;
use crate::endpoints::orders::OrderEndpoints;
use crate::models::order::{OrderRequest, OrderSide, OrderType, TimeInForce};
use crate::auth::TokenStore;
use crate::error::WebullResult;
use reqwest::Client;
use std::sync::Arc;
use mockito::{mock, server_url};

struct MockTokenStore;

impl TokenStore for MockTokenStore {
    fn get_token(&self) -> WebullResult<Option<crate::auth::Token>> {
        Ok(Some(crate::auth::Token {
            access_token: "test-token".to_string(),
            refresh_token: "test-refresh-token".to_string(),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(1),
        }))
    }

    fn store_token(&self, _token: crate::auth::Token) -> WebullResult<()> {
        Ok(())
    }

    fn clear_token(&self) -> WebullResult<()> {
        Ok(())
    }
}

#[tokio::test]
async fn test_place_order() {
    let mock_server = mock("POST", "/api/trade/order")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        {
            "id": "test-order-id",
            "status": "NEW",
            "symbol": "AAPL",
            "quantity": 1.0,
            "price": 150.0,
            "side": "BUY",
            "order_type": "LIMIT",
            "time_in_force": "DAY",
            "extended_hours": false,
            "created_at": "2023-01-01T00:00:00Z"
        }
        "#)
        .create();

    let client = Client::new();
    let auth_manager = Arc::new(AuthManager::new(
        crate::WebullConfig::default(),
        Box::new(MockTokenStore),
        client.clone(),
    ));
    let orders = OrderEndpoints::new(client, server_url(), auth_manager);

    let order = OrderRequest::limit()
        .symbol("AAPL")
        .quantity(1.0)
        .price(150.0)
        .side(OrderSide::Buy)
        .time_in_force(TimeInForce::Day);

    let result = orders.place_order(&order).await;
    assert!(result.is_ok());

    let order_response = result.unwrap();
    assert_eq!(order_response.id, "test-order-id");
    assert_eq!(order_response.symbol, "AAPL");
    assert_eq!(order_response.quantity, 1.0);
    assert_eq!(order_response.price, Some(150.0));

    mock_server.assert();
}

#[tokio::test]
async fn test_cancel_order() {
    let mock_server = mock("DELETE", "/api/trade/cancel/test-order-id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body("{}")
        .create();

    let client = Client::new();
    let auth_manager = Arc::new(AuthManager::new(
        crate::WebullConfig::default(),
        Box::new(MockTokenStore),
        client.clone(),
    ));
    let orders = OrderEndpoints::new(client, server_url(), auth_manager);

    let result = orders.cancel_order("test-order-id").await;
    assert!(result.is_ok());

    mock_server.assert();
}

#[tokio::test]
async fn test_get_order() {
    let mock_server = mock("GET", "/api/trade/order/test-order-id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        {
            "id": "test-order-id",
            "symbol": "AAPL",
            "quantity": 1.0,
            "filled_quantity": 0.0,
            "price": 150.0,
            "status": "NEW",
            "side": "BUY",
            "order_type": "LIMIT",
            "time_in_force": "DAY",
            "extended_hours": false,
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z",
            "commission": 0.0
        }
        "#)
        .create();

    let client = Client::new();
    let auth_manager = Arc::new(AuthManager::new(
        crate::WebullConfig::default(),
        Box::new(MockTokenStore),
        client.clone(),
    ));
    let orders = OrderEndpoints::new(client, server_url(), auth_manager);

    let result = orders.get_order("test-order-id").await;
    assert!(result.is_ok());

    let order = result.unwrap();
    assert_eq!(order.id, "test-order-id");
    assert_eq!(order.symbol, "AAPL");
    assert_eq!(order.quantity, 1.0);
    assert_eq!(order.price, Some(150.0));

    mock_server.assert();
}

#[tokio::test]
async fn test_get_open_orders() {
    let mock_server = mock("GET", "/api/trade/account/test-account-id/orders/open")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        [
            {
                "id": "test-order-id-1",
                "symbol": "AAPL",
                "quantity": 1.0,
                "filled_quantity": 0.0,
                "price": 150.0,
                "status": "NEW",
                "side": "BUY",
                "order_type": "LIMIT",
                "time_in_force": "DAY",
                "extended_hours": false,
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z",
                "commission": 0.0
            },
            {
                "id": "test-order-id-2",
                "symbol": "MSFT",
                "quantity": 2.0,
                "filled_quantity": 0.0,
                "price": 250.0,
                "status": "NEW",
                "side": "BUY",
                "order_type": "LIMIT",
                "time_in_force": "DAY",
                "extended_hours": false,
                "created_at": "2023-01-01T00:00:00Z",
                "updated_at": "2023-01-01T00:00:00Z",
                "commission": 0.0
            }
        ]
        "#)
        .create();

    let client = Client::new();
    let auth_manager = Arc::new(AuthManager::new(
        crate::WebullConfig::default(),
        Box::new(MockTokenStore),
        client.clone(),
    ));
    let orders = OrderEndpoints::new(client, server_url(), auth_manager);

    let result = orders.get_open_orders("test-account-id").await;
    assert!(result.is_ok());

    let orders_list = result.unwrap();
    assert_eq!(orders_list.len(), 2);
    assert_eq!(orders_list[0].id, "test-order-id-1");
    assert_eq!(orders_list[0].symbol, "AAPL");
    assert_eq!(orders_list[1].id, "test-order-id-2");
    assert_eq!(orders_list[1].symbol, "MSFT");

    mock_server.assert();
}

#[tokio::test]
async fn test_modify_order() {
    let mock_server = mock("PUT", "/api/trade/modify/test-order-id")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        {
            "id": "test-order-id-modified",
            "status": "REPLACED",
            "symbol": "AAPL",
            "quantity": 2.0,
            "price": 155.0,
            "side": "BUY",
            "order_type": "LIMIT",
            "time_in_force": "DAY",
            "extended_hours": false,
            "created_at": "2023-01-01T00:00:00Z"
        }
        "#)
        .create();

    let client = Client::new();
    let auth_manager = Arc::new(AuthManager::new(
        crate::WebullConfig::default(),
        Box::new(MockTokenStore),
        client.clone(),
    ));
    let orders = OrderEndpoints::new(client, server_url(), auth_manager);

    let order = OrderRequest::limit()
        .symbol("AAPL")
        .quantity(2.0)
        .price(155.0)
        .side(OrderSide::Buy)
        .time_in_force(TimeInForce::Day);

    let result = orders.modify_order("test-order-id", &order).await;
    assert!(result.is_ok());

    let order_response = result.unwrap();
    assert_eq!(order_response.id, "test-order-id-modified");
    assert_eq!(order_response.symbol, "AAPL");
    assert_eq!(order_response.quantity, 2.0);
    assert_eq!(order_response.price, Some(155.0));

    mock_server.assert();
}
