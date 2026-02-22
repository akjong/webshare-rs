//! Subscription and checkout endpoints.
//!
//! Endpoints: Available Assets, Customize, Pricing, Purchase, Renew, Download Invoice.

use crate::{
    client::WebshareClient,
    error::Result,
    models::subscription::{
        AvailableAssets, CheckoutResponse, CustomizeQuery, CustomizeResponse, PricingQuery,
        PricingResponse, PurchaseRequest, RenewRequest,
    },
};

impl WebshareClient {
    /// Get available proxy assets.
    ///
    /// `GET /api/v2/subscription/available_assets/`
    pub async fn get_available_assets(&self) -> Result<AvailableAssets> {
        let req = self.get("/api/v2/subscription/available_assets/");
        self.send_json(req).await
    }

    /// Get customization options for a subscription.
    ///
    /// The query is JSON-encoded and passed as `?query={json}`.
    ///
    /// `GET /api/v2/subscription/customize/`
    pub async fn get_customization_options(
        &self,
        query: &CustomizeQuery,
    ) -> Result<CustomizeResponse> {
        let json_query = serde_json::to_string(query).unwrap_or_default();
        let mut req = self
            .get("/api/v2/subscription/customize/")
            .query(&[("query", &json_query)]);
        if let Some(plan_id) = query.plan_id {
            req = req.query(&[("plan_id", plan_id.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get pricing for a subscription configuration.
    ///
    /// The query is JSON-encoded and passed as `?query={json}`.
    ///
    /// `GET /api/v2/subscription/pricing/`
    pub async fn get_pricing(&self, query: &PricingQuery) -> Result<PricingResponse> {
        let json_query = serde_json::to_string(query).unwrap_or_default();
        let req = self
            .get("/api/v2/subscription/pricing/")
            .query(&[("query", &json_query)]);
        self.send_json(req).await
    }

    /// Purchase a new plan.
    ///
    /// `POST /api/v2/subscription/checkout/purchase/`
    pub async fn purchase_plan(&self, request: &PurchaseRequest) -> Result<CheckoutResponse> {
        let req = self
            .post("/api/v2/subscription/checkout/purchase/")
            .json(request);
        self.send_json(req).await
    }

    /// Renew an existing subscription.
    ///
    /// `POST /api/v2/subscription/checkout/renew/`
    pub async fn renew_subscription(&self, request: &RenewRequest) -> Result<CheckoutResponse> {
        let req = self
            .post("/api/v2/subscription/checkout/renew/")
            .json(request);
        self.send_json(req).await
    }

    /// Download an invoice as PDF.
    ///
    /// `GET /api/v2/invoices/download?subscription_transaction_id={id}`
    pub async fn download_invoice(&self, subscription_transaction_id: i64) -> Result<Vec<u8>> {
        let req = self.get("/api/v2/invoices/download").query(&[(
            "subscription_transaction_id",
            subscription_transaction_id.to_string(),
        )]);
        self.send_bytes(req).await
    }
}
