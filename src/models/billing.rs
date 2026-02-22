//! Billing and payment model types.

use serde::{Deserialize, Serialize};

/// Billing information for invoices.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingInfo {
    pub id: i64,
    pub name: String,
    pub address: String,
    pub billing_email: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to update billing information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateBillingInfoRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<String>,
}

/// A payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub expiration_year: Option<i32>,
    pub expiration_month: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
    /// Street address line.
    pub line: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
}

/// Embedded payment method in transactions (fewer fields).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionPaymentMethod {
    pub id: i64,
    pub brand: Option<String>,
    pub last4: Option<String>,
    pub name: Option<String>,
    pub expiration_year: Option<i32>,
    pub expiration_month: Option<i32>,
    pub created_at: String,
    pub updated_at: String,
}

/// A payment transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i64,
    pub status: String,
    pub payment_method: Option<TransactionPaymentMethod>,
    pub reason: Option<String>,
    pub amount: f64,
    pub credits_used: f64,
    pub credits_gained: f64,
    pub refund_amount: f64,
    pub refund_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// A pending payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingPayment {
    pub id: i64,
    pub status: String,
    pub failure_reason: Option<String>,
    pub payment_method: Option<serde_json::Value>,
    pub plan: Option<serde_json::Value>,
    pub transaction: Option<serde_json::Value>,
    pub is_renewal: bool,
    pub term: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub completed_at: Option<String>,
}

/// Request to update/create a payment method (requires recaptcha).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentMethodRequest {
    pub recaptcha: String,
}

/// Response from updating a payment method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePaymentMethodResponse {
    pub pending_payment: i64,
    pub stripe_client_secret: String,
    pub stripe_payment_intent: Option<String>,
    pub stripe_setup_intent: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_billing_info() {
        let json = r#"{
            "id": 1,
            "name": "Webshare Software",
            "address": "Lemon Ave",
            "billing_email": "incomingbills@webshare.io",
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00"
        }"#;
        let info: BillingInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.name, "Webshare Software");
    }

    #[test]
    fn deserialize_payment_method() {
        let json = r#"{
            "id": 1,
            "type": "StripeCard",
            "brand": "visa",
            "last4": "4242",
            "name": null,
            "expiration_year": 2023,
            "expiration_month": 6,
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00",
            "line": "123 George Street",
            "city": "Sydney",
            "state": null,
            "postal_code": "2000",
            "country": "AU"
        }"#;
        let pm: PaymentMethod = serde_json::from_str(json).unwrap();
        assert_eq!(pm.brand.as_deref(), Some("visa"));
        assert_eq!(pm.country.as_deref(), Some("AU"));
    }

    #[test]
    fn deserialize_transaction() {
        let json = r#"{
            "id": 1,
            "status": "completed",
            "payment_method": {
                "id": 1,
                "brand": "visa",
                "last4": "4242",
                "name": null,
                "expiration_year": 2023,
                "expiration_month": 6,
                "created_at": "2022-06-14T11:58:10.246406-07:00",
                "updated_at": "2022-06-14T11:58:10.246406-07:00"
            },
            "reason": "Upgraded from Free Plan to 100 Proxies",
            "amount": 1.0,
            "credits_used": 0.0,
            "credits_gained": 0.0,
            "refund_amount": 0.0,
            "refund_date": null,
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00"
        }"#;
        let tx: Transaction = serde_json::from_str(json).unwrap();
        assert_eq!(tx.status, "completed");
        assert!(tx.payment_method.is_some());
    }

    #[test]
    fn deserialize_pending_payment() {
        let json = r#"{
            "id": 1,
            "status": "pending",
            "failure_reason": null,
            "payment_method": 2,
            "plan": 2,
            "transaction": null,
            "is_renewal": false,
            "term": "monthly",
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00",
            "completed_at": null
        }"#;
        let pp: PendingPayment = serde_json::from_str(json).unwrap();
        assert_eq!(pp.status, "pending");
        assert!(!pp.is_renewal);
    }
}
