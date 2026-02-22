//! Proxy configuration model types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Proxy configuration (v3 endpoint response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub request_timeout: i64,
    pub request_idle_timeout: i64,
    pub ip_authorization_country_codes: Option<Vec<String>>,
    pub auto_replace_invalid_proxies: bool,
    pub auto_replace_low_country_confidence_proxies: bool,
    pub auto_replace_out_of_rotation_proxies: bool,
    pub auto_replace_failed_site_check_proxies: bool,
    pub proxy_list_download_token: String,
}

/// Full proxy configuration (v2 update response), with additional fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfigFull {
    pub id: i64,
    pub state: String,
    pub countries: HashMap<String, serde_json::Value>,
    pub available_countries: HashMap<String, serde_json::Value>,
    pub unallocated_countries: serde_json::Value,
    pub ip_ranges_24: HashMap<String, serde_json::Value>,
    pub ip_ranges_16: HashMap<String, serde_json::Value>,
    pub ip_ranges_8: HashMap<String, serde_json::Value>,
    pub available_ip_ranges_24: HashMap<String, serde_json::Value>,
    pub available_ip_ranges_16: HashMap<String, serde_json::Value>,
    pub available_ip_ranges_8: HashMap<String, serde_json::Value>,
    pub asns: HashMap<String, serde_json::Value>,
    pub available_asns: HashMap<String, serde_json::Value>,
    pub username: String,
    pub password: String,
    pub request_timeout: i64,
    pub request_idle_timeout: i64,
    pub ip_authorization_country_codes: Option<Vec<String>>,
    pub auto_replace_invalid_proxies: bool,
    pub auto_replace_low_country_confidence_proxies: bool,
    pub auto_replace_out_of_rotation_proxies: bool,
    pub auto_replace_failed_site_check_proxies: bool,
    pub proxy_list_download_token: String,
    pub is_proxy_used: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for updating proxy configuration.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProxyConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_idle_timeout: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_authorization_country_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replace_invalid_proxies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replace_low_country_confidence_proxies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replace_out_of_rotation_proxies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_replace_failed_site_check_proxies: Option<bool>,
}

/// Proxy list statistics (v3 endpoint response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyListStats {
    pub available_countries: HashMap<String, i64>,
    pub ip_ranges_24: HashMap<String, i64>,
    pub ip_ranges_16: HashMap<String, i64>,
    pub ip_ranges_8: HashMap<String, i64>,
    pub available_ip_ranges_24: HashMap<String, i64>,
    pub available_ip_ranges_16: HashMap<String, i64>,
    pub available_ip_ranges_8: HashMap<String, i64>,
    pub asns: HashMap<String, serde_json::Value>,
    pub available_asns: HashMap<String, serde_json::Value>,
}

/// Proxy list status (v3 endpoint response).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyListStatus {
    pub state: String,
    pub countries: HashMap<String, i64>,
    pub unallocated_countries: serde_json::Value,
    pub username: String,
    pub password: String,
    pub is_proxy_used: bool,
}

/// Request body for allocating unallocated countries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocateCountriesRequest {
    pub new_countries: HashMap<String, u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_proxy_config() {
        let json = r#"{
            "request_timeout": 30,
            "request_idle_timeout": 30,
            "ip_authorization_country_codes": ["US"],
            "auto_replace_invalid_proxies": true,
            "auto_replace_low_country_confidence_proxies": false,
            "auto_replace_out_of_rotation_proxies": false,
            "auto_replace_failed_site_check_proxies": false,
            "proxy_list_download_token": "abc"
        }"#;
        let config: ProxyConfig = serde_json::from_str(json).unwrap();
        assert_eq!(config.request_timeout, 30);
    }

    #[test]
    fn update_request_skip_none() {
        let req = UpdateProxyConfigRequest {
            username: Some("user".into()),
            ..Default::default()
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("username"));
        assert!(!json.contains("password"));
    }
}
