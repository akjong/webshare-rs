//! Subscription, pricing, and checkout model types.

use serde::{Deserialize, Serialize};

/// Available proxy assets breakdown.
///
/// The response is a nested object keyed by proxy category and subtype.
/// We represent it as `serde_json::Value` for maximum flexibility.
pub type AvailableAssets = serde_json::Value;

/// Query parameters for the customization endpoint.
///
/// Serialized to JSON and passed as `?query={json}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizeQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_subtype: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_countries: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_site_checks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

/// Feature entry in customization/pricing responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feature {
    pub feature: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_selected: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
}

/// Term option in customization response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TermOption {
    pub term: String,
    pub renewals_paid: i64,
}

/// Site check entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteCheck {
    pub name: String,
}

/// Customization options response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizeResponse {
    pub proxy_type: String,
    pub proxy_subtype: String,
    pub proxy_count_max: i64,
    pub proxy_count_min: i64,
    pub available_countries: serde_json::Value,
    pub on_demand_refreshes_max: i64,
    pub on_demand_refreshes_min: i64,
    pub automatic_refresh_frequency_max: i64,
    pub automatic_refresh_frequency_min: i64,
    pub proxy_replacements_max: i64,
    pub proxy_replacements_min: i64,
    pub bandwidth_limit_max: i64,
    pub bandwidth_limit_min: i64,
    pub subusers_max: i64,
    pub subusers_min: i64,
    pub available_features: Vec<Feature>,
    pub available_site_checks: Vec<SiteCheck>,
    pub terms: Vec<TermOption>,
}

/// Query parameters for the pricing endpoint.
///
/// Serialized to JSON and passed as `?query={json}`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
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
    pub with_tax: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

/// Proxy count discount tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyCountDiscountTier {
    pub from: i64,
    pub to: Option<i64>,
    pub discount_percentage: i64,
    pub per_proxy_price: f64,
}

/// Bandwidth discount tier.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BandwidthDiscountTier {
    pub from: i64,
    pub to: Option<i64>,
    pub per_gb_price: Option<f64>,
}

/// Tax breakdown entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxBreakdown {
    pub amount: String,
    pub tax_rate_details: TaxRateDetails,
    pub taxable_amount: String,
}

/// Tax rate details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRateDetails {
    pub percentage_decimal: String,
    pub tax_type: String,
}

/// Pricing response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PricingResponse {
    pub discount_percentage: i64,
    pub non_discounted_price: f64,
    pub price: f64,
    pub paid_today: f64,
    pub credits_added: f64,
    pub credits_used: f64,
    pub proxy_count_discount_tiers: Vec<ProxyCountDiscountTier>,
    pub bandwidth_discount_tiers: Vec<BandwidthDiscountTier>,
    pub features: Vec<Feature>,
    #[serde(default)]
    pub tax_breakdown: Vec<TaxBreakdown>,
}

/// Request body for purchasing a plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PurchaseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behavior: Option<String>,
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

/// Purchase or renew response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutResponse {
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

/// Request body for renewing a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<serde_json::Value>,
    pub term: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recaptcha: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_checkout_response() {
        let json = r#"{
            "payment_required": false,
            "plan": 42
        }"#;
        let resp: CheckoutResponse = serde_json::from_str(json).unwrap();
        assert!(!resp.payment_required);
        assert_eq!(resp.plan, 42);
    }
}
