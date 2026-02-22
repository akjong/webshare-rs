//! Billing and payment endpoints.
//!
//! Includes billing information, payment methods, transactions,
//! pending payments, and payment method updates.

use crate::{
    client::WebshareClient,
    error::Result,
    models::billing::{
        BillingInfo, PaymentMethod, PendingPayment, Transaction, UpdateBillingInfoRequest,
        UpdatePaymentMethodRequest, UpdatePaymentMethodResponse,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    // ── Billing Information ─────────────────────────────────────────

    /// Get billing information.
    ///
    /// `GET /api/v2/subscription/billing_info/`
    pub async fn get_billing_info(&self) -> Result<BillingInfo> {
        let req = self.get("/api/v2/subscription/billing_info/");
        self.send_json(req).await
    }

    /// Update billing information.
    ///
    /// `PATCH /api/v2/subscription/billing_info/`
    pub async fn update_billing_info(
        &self,
        request: &UpdateBillingInfoRequest,
    ) -> Result<BillingInfo> {
        let req = self
            .patch("/api/v2/subscription/billing_info/")
            .json(request);
        self.send_json(req).await
    }

    // ── Payment Methods ─────────────────────────────────────────────

    /// List payment methods (paginated).
    ///
    /// `GET /api/v2/payment/method/`
    pub async fn list_payment_methods(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<PaymentMethod>> {
        let mut req = self.get("/api/v2/payment/method/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get a payment method by ID.
    ///
    /// `GET /api/v2/payment/method/{id}/`
    pub async fn get_payment_method(&self, id: i64) -> Result<PaymentMethod> {
        let req = self.get(&format!("/api/v2/payment/method/{id}/"));
        self.send_json(req).await
    }

    /// Update / create a payment method (requires recaptcha token).
    ///
    /// Returns Stripe client secret and setup intent for frontend integration.
    ///
    /// `POST /api/v2/payment/method/`
    pub async fn update_payment_method(
        &self,
        request: &UpdatePaymentMethodRequest,
    ) -> Result<UpdatePaymentMethodResponse> {
        let req = self.post("/api/v2/payment/method/").json(request);
        self.send_json(req).await
    }

    // ── Transactions ────────────────────────────────────────────────

    /// List transactions (paginated).
    ///
    /// `GET /api/v2/payment/transaction/`
    pub async fn list_transactions(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<Transaction>> {
        let mut req = self.get("/api/v2/payment/transaction/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get a transaction by ID.
    ///
    /// `GET /api/v2/payment/transaction/{id}/`
    pub async fn get_transaction(&self, id: i64) -> Result<Transaction> {
        let req = self.get(&format!("/api/v2/payment/transaction/{id}/"));
        self.send_json(req).await
    }

    // ── Pending Payments ────────────────────────────────────────────

    /// List pending payments (paginated).
    ///
    /// `GET /api/v2/payment/pending/`
    pub async fn list_pending_payments(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<PendingPayment>> {
        let mut req = self.get("/api/v2/payment/pending/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get a pending payment by ID.
    ///
    /// `GET /api/v2/payment/pending/{id}/`
    pub async fn get_pending_payment(&self, id: i64) -> Result<PendingPayment> {
        let req = self.get(&format!("/api/v2/payment/pending/{id}/"));
        self.send_json(req).await
    }
}
