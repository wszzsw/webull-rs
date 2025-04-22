use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::order::{Order, OrderQueryParams, OrderRequest, OrderResponse};
use reqwest::Client;
use std::sync::Arc;

/// Endpoints for order operations.
pub struct OrderEndpoints {
    /// Base endpoint
    base: BaseEndpoint,
}

impl OrderEndpoints {
    /// Create new order endpoints.
    pub fn new(client: Client, base_url: String, auth_manager: Arc<AuthManager>) -> Self {
        Self {
            base: BaseEndpoint::new(client, base_url, auth_manager),
        }
    }
    
    /// Place an order.
    pub async fn place_order(&self, order: &OrderRequest) -> WebullResult<OrderResponse> {
        self.base.post("/api/trade/order", order).await
    }
    
    /// Cancel an order.
    pub async fn cancel_order(&self, order_id: &str) -> WebullResult<()> {
        let path = format!("/api/trade/cancel/{}", order_id);
        self.base.delete(&path).await
    }
    
    /// Get an order by ID.
    pub async fn get_order(&self, order_id: &str) -> WebullResult<Order> {
        let path = format!("/api/trade/order/{}", order_id);
        self.base.get(&path).await
    }
    
    /// Get orders based on query parameters.
    pub async fn get_orders(&self, params: &OrderQueryParams) -> WebullResult<Vec<Order>> {
        self.base.post("/api/trade/orders", params).await
    }
    
    /// Get active orders.
    pub async fn get_active_orders(&self) -> WebullResult<Vec<Order>> {
        self.base.get("/api/trade/active").await
    }
    
    /// Get filled orders.
    pub async fn get_filled_orders(&self) -> WebullResult<Vec<Order>> {
        self.base.get("/api/trade/filled").await
    }
    
    /// Modify an existing order.
    pub async fn modify_order(&self, order_id: &str, order: &OrderRequest) -> WebullResult<OrderResponse> {
        let path = format!("/api/trade/modify/{}", order_id);
        self.base.put(&path, order).await
    }
}
