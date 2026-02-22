//! Proxy list model types.

use serde::{Deserialize, Serialize};

/// A proxy resource from the proxy list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proxy {
    pub id: String,
    pub username: String,
    pub password: String,
    pub proxy_address: String,
    pub port: u32,
    pub valid: bool,
    pub last_verification: String,
    pub country_code: String,
    pub city_name: String,
    pub created_at: String,
}

/// Query parameters for listing proxies.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListProxiesParams {
    pub mode: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    #[serde(rename = "country_code__in", skip_serializing_if = "Option::is_none")]
    pub country_code_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_address: Option<String>,
    #[serde(rename = "proxy_address__in", skip_serializing_if = "Option::is_none")]
    pub proxy_address_in: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_name: Option<String>,
}

/// Parameters for downloading the proxy list (URL path segments).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyDownloadParams {
    pub token: String,
    /// Hyphen-separated country codes, or `-` for all.
    pub country_codes: String,
    /// `username` or `sourceip`.
    pub authentication_method: String,
    /// `backbone` or `direct`.
    pub endpoint_mode: String,
    /// URL-encoded search string, or `-` for none.
    pub search: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_proxy() {
        let json = r#"{
            "id": "d-10513",
            "username": "user",
            "password": "pass",
            "proxy_address": "1.2.3.4",
            "port": 8080,
            "valid": true,
            "last_verification": "2024-01-01T00:00:00Z",
            "country_code": "US",
            "city_name": "New York",
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let proxy: Proxy = serde_json::from_str(json).unwrap();
        assert_eq!(proxy.id, "d-10513");
        assert_eq!(proxy.port, 8080);
    }

    #[test]
    fn list_params_skip_none() {
        let params = ListProxiesParams {
            mode: "direct".into(),
            ..Default::default()
        };
        let json = serde_json::to_string(&params).unwrap();
        assert!(!json.contains("page"));
        assert!(json.contains("direct"));
    }
}
