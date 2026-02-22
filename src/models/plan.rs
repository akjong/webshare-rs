//! Plan model types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A subscription plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plan {
    pub id: i64,
    pub status: String,
    pub bandwidth_limit: f64,
    pub monthly_price: f64,
    pub yearly_price: f64,
    pub proxy_type: String,
    pub proxy_subtype: String,
    pub proxy_count: i64,
    pub proxy_countries: HashMap<String, i64>,
    pub required_site_checks: Vec<String>,
    pub on_demand_refreshes_total: i64,
    pub on_demand_refreshes_used: i64,
    pub on_demand_refreshes_available: i64,
    pub automatic_refresh_frequency: i64,
    pub automatic_refresh_last_at: Option<String>,
    pub automatic_refresh_next_at: Option<String>,
    pub proxy_replacements_total: i64,
    pub proxy_replacements_used: i64,
    pub proxy_replacements_available: i64,
    pub subusers_total: i64,
    pub subusers_used: i64,
    pub subusers_available: i64,
    pub is_unlimited_ip_authorizations: bool,
    pub is_high_concurrency: bool,
    pub is_high_priority_network: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for updating a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePlanRequest {
    pub automatic_refresh_next_at: String,
}

/// Request body for upgrading a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradePlanRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_countries: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bandwidth_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_demand_refreshes_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_refresh_frequency: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_replacements_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subusers_total: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unlimited_ip_authorizations: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_high_concurrency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_high_priority_network: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_site_checks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recaptcha: Option<String>,
}

/// Response from upgrading a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradePlanResponse {
    pub payment_required: bool,
    pub plan: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_payment: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_payment_intent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_payment_method: Option<String>,
}

/// Response from cancelling a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancelPlanResponse {
    pub success: bool,
    pub transaction: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_cancel_response() {
        let json = r#"{"success": true, "transaction": 123}"#;
        let resp: CancelPlanResponse = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        assert_eq!(resp.transaction, 123);
    }
}
