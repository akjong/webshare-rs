//! Proxy replacement model types.

use serde::{Deserialize, Serialize};

/// A proxy replacement operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyReplacement {
    pub id: i64,
    /// Replacement target specification (typed as `serde_json::Value` because
    /// the shape varies by `type` field: `ip_range`, `ip_address`, `asn`,
    /// `country`, `any`).
    pub to_replace: serde_json::Value,
    /// Replacement source specifications.
    pub replace_with: Vec<serde_json::Value>,
    pub dry_run: bool,
    /// One of: `validating`, `validated`, `processing`, `completed`, `failed`.
    pub state: String,
    pub proxies_removed: Option<i64>,
    pub proxies_added: Option<i64>,
    pub reason: String,
    pub error_code: Option<String>,
    pub error: Option<String>,
    pub created_at: String,
    pub dry_run_completed_at: Option<String>,
    pub completed_at: Option<String>,
}

/// Request body for creating a proxy replacement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReplacementRequest {
    pub to_replace: serde_json::Value,
    pub replace_with: Vec<serde_json::Value>,
    pub dry_run: bool,
}

/// Query parameters for listing replacements.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListReplacementsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dry_run: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// A replaced proxy record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplacedProxy {
    pub id: i64,
    pub reason: String,
    pub proxy: String,
    pub proxy_port: u32,
    pub proxy_country_code: String,
    pub replaced_with: String,
    pub replaced_with_port: u32,
    pub replaced_with_country_code: String,
    pub created_at: String,
}

/// Query parameters for listing replaced proxies.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListReplacedProxiesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_list_replacement: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Query parameters for downloading replaced proxies.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadReplacedProxiesParams {
    pub download_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_codes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_list_replacement: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_proxy_replacement() {
        let json = r#"{
            "id": 1,
            "to_replace": {"type": "country", "country_code": "US"},
            "replace_with": [{"type": "country", "country_code": "GB"}],
            "dry_run": false,
            "state": "completed",
            "proxies_removed": 5,
            "proxies_added": 5,
            "reason": "",
            "error_code": null,
            "error": null,
            "created_at": "2024-01-01T00:00:00Z",
            "dry_run_completed_at": null,
            "completed_at": "2024-01-01T00:01:00Z"
        }"#;
        let r: ProxyReplacement = serde_json::from_str(json).unwrap();
        assert_eq!(r.state, "completed");
        assert_eq!(r.proxies_removed, Some(5));
    }

    #[test]
    fn deserialize_replaced_proxy() {
        let json = r#"{
            "id": 42,
            "reason": "auto_replace",
            "proxy": "1.2.3.4",
            "proxy_port": 8080,
            "proxy_country_code": "US",
            "replaced_with": "5.6.7.8",
            "replaced_with_port": 9090,
            "replaced_with_country_code": "US",
            "created_at": "2024-01-01T00:00:00Z"
        }"#;
        let rp: ReplacedProxy = serde_json::from_str(json).unwrap();
        assert_eq!(rp.id, 42);
    }
}
