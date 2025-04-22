use crate::auth::AuthManager;
use crate::endpoints::base::BaseEndpoint;
use crate::error::WebullResult;
use crate::models::order::{
    OptionOrderPreviewRequest, OptionOrderPreviewResponse, OptionOrderRequest, Order,
    OrderQueryParams, OrderRequest, OrderResponse,
};
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
    pub async fn modify_order(
        &self,
        order_id: &str,
        order: &OrderRequest,
    ) -> WebullResult<OrderResponse> {
        let path = format!("/api/trade/modify/{}", order_id);
        self.base.put(&path, order).await
    }

    /// Get open orders for an account.
    pub async fn get_open_orders(&self, account_id: &str) -> WebullResult<Vec<Order>> {
        let path = format!("/api/trade/account/{}/orders/open", account_id);
        self.base.get(&path).await
    }

    /// Get open orders for an account with pagination.
    pub async fn get_open_orders_paged(
        &self,
        account_id: &str,
        page_size: u32,
        last_order_id: Option<&str>,
    ) -> WebullResult<Vec<Order>> {
        #[derive(serde::Serialize)]
        struct OpenOrdersRequest {
            account_id: String,
            page_size: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            last_client_order_id: Option<String>,
        }

        let mut request = OpenOrdersRequest {
            account_id: account_id.to_string(),
            page_size,
            last_client_order_id: None,
        };

        if let Some(order_id) = last_order_id {
            request.last_client_order_id = Some(order_id.to_string());
        }

        self.base.post("/api/trade/orders/open", &request).await
    }

    /// Get today's orders for an account.
    pub async fn get_today_orders(&self, account_id: &str) -> WebullResult<Vec<Order>> {
        let path = format!("/api/trade/account/{}/orders/today", account_id);
        self.base.get(&path).await
    }

    /// Get today's orders for an account with pagination.
    pub async fn get_today_orders_paged(
        &self,
        account_id: &str,
        page_size: u32,
        last_order_id: Option<&str>,
    ) -> WebullResult<Vec<Order>> {
        #[derive(serde::Serialize)]
        struct TodayOrdersRequest {
            account_id: String,
            page_size: u32,
            #[serde(skip_serializing_if = "Option::is_none")]
            last_client_order_id: Option<String>,
        }

        let mut request = TodayOrdersRequest {
            account_id: account_id.to_string(),
            page_size,
            last_client_order_id: None,
        };

        if let Some(order_id) = last_order_id {
            request.last_client_order_id = Some(order_id.to_string());
        }

        self.base.post("/api/trade/orders/today", &request).await
    }

    /// Preview an option order.
    pub async fn preview_option_order(
        &self,
        preview_request: &OptionOrderPreviewRequest,
    ) -> WebullResult<OptionOrderPreviewResponse> {
        self.base
            .post("/api/trade/option/preview", preview_request)
            .await
    }

    /// Place an option order.
    pub async fn place_option_order(
        &self,
        account_id: &str,
        orders: &[OptionOrderRequest],
    ) -> WebullResult<Vec<OrderResponse>> {
        #[derive(serde::Serialize)]
        struct PlaceOptionRequest<'a> {
            account_id: &'a str,
            new_orders: &'a [OptionOrderRequest],
        }

        let request = PlaceOptionRequest {
            account_id,
            new_orders: orders,
        };

        self.base.post("/api/trade/option/place", &request).await
    }

    /// Replace an option order.
    pub async fn replace_option_order(
        &self,
        account_id: &str,
        orders: &[OptionOrderRequest],
    ) -> WebullResult<Vec<OrderResponse>> {
        #[derive(serde::Serialize)]
        struct ReplaceOptionRequest<'a> {
            account_id: &'a str,
            modify_orders: &'a [OptionOrderRequest],
        }

        let request = ReplaceOptionRequest {
            account_id,
            modify_orders: orders,
        };

        self.base.post("/api/trade/option/replace", &request).await
    }

    /// Cancel an option order.
    pub async fn cancel_option_order(
        &self,
        account_id: &str,
        client_order_id: &str,
    ) -> WebullResult<()> {
        #[derive(serde::Serialize)]
        struct CancelOptionRequest<'a> {
            account_id: &'a str,
            client_order_id: &'a str,
        }

        let request = CancelOptionRequest {
            account_id,
            client_order_id,
        };

        self.base.post("/api/trade/option/cancel", &request).await
    }
}
