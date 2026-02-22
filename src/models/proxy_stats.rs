//! Proxy statistics and activity model types.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Error reason breakdown entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReason {
    pub reason: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub how_to_fix: String,
    pub http_status: i64,
    pub count: i64,
}

/// A single stats time-bucket entry.
///
/// Note: the `list_stats` endpoint returns a plain `Vec<ProxyStat>`, **not** a
/// paginated response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStat {
    pub timestamp: String,
    pub is_projected: bool,
    pub bandwidth_total: f64,
    pub bandwidth_average: f64,
    pub requests_total: i64,
    pub requests_successful: i64,
    pub requests_failed: i64,
    pub error_reasons: Vec<ErrorReason>,
    pub countries_used: HashMap<String, i64>,
    pub number_of_proxies_used: i64,
    pub protocols_used: HashMap<String, i64>,
    pub average_concurrency: Option<f64>,
    pub average_rps: Option<f64>,
    pub last_request_sent_at: Option<String>,
}

/// Aggregate statistics across a time range.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregateStats {
    pub bandwidth_projected: f64,
    pub bandwidth_total: f64,
    pub bandwidth_average: f64,
    pub requests_total: i64,
    pub requests_successful: i64,
    pub requests_failed: i64,
    pub error_reasons: Vec<ErrorReason>,
    pub countries_used: HashMap<String, i64>,
    pub number_of_proxies_used: i64,
    pub protocols_used: HashMap<String, i64>,
    pub average_concurrency: Option<f64>,
    pub average_rps: Option<f64>,
    pub last_request_sent_at: Option<String>,
}

/// A single proxy activity record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyActivity {
    pub timestamp: String,
    pub protocol: String,
    pub request_duration: f64,
    pub handshake_duration: f64,
    pub tunnel_duration: Option<f64>,
    pub error_reason: Option<String>,
    pub error_reason_how_to_fix: Option<String>,
    pub auth_username: Option<String>,
    pub proxy_address: Option<String>,
    pub bytes: i64,
    pub client_address: String,
    pub ip_address: Option<String>,
    pub hostname: Option<String>,
    pub domain: Option<String>,
    pub port: Option<u32>,
    pub proxy_port: Option<u32>,
    pub listen_address: String,
    pub listen_port: u32,
}

/// Query parameters for listing stats.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListStatsParams {
    #[serde(rename = "timestamp__lte", skip_serializing_if = "Option::is_none")]
    pub timestamp_lte: Option<String>,
    #[serde(rename = "timestamp__gte", skip_serializing_if = "Option::is_none")]
    pub timestamp_gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

/// Query parameters for listing activities.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListActivitiesParams {
    #[serde(rename = "timestamp__lte", skip_serializing_if = "Option::is_none")]
    pub timestamp_lte: Option<String>,
    #[serde(rename = "timestamp__gte", skip_serializing_if = "Option::is_none")]
    pub timestamp_gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "bytes__gte", skip_serializing_if = "Option::is_none")]
    pub bytes_gte: Option<i64>,
    #[serde(rename = "bytes__lte", skip_serializing_if = "Option::is_none")]
    pub bytes_lte: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Query parameters for downloading activities (CSV).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DownloadActivitiesParams {
    pub download_token: String,
    #[serde(rename = "timestamp__lte", skip_serializing_if = "Option::is_none")]
    pub timestamp_lte: Option<String>,
    #[serde(rename = "timestamp__gte", skip_serializing_if = "Option::is_none")]
    pub timestamp_gte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
    #[serde(rename = "bytes__gte", skip_serializing_if = "Option::is_none")]
    pub bytes_gte: Option<i64>,
    #[serde(rename = "bytes__lte", skip_serializing_if = "Option::is_none")]
    pub bytes_lte: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_proxy_stat() {
        let json = r#"{
            "timestamp": "2024-01-01T00:00:00Z",
            "is_projected": false,
            "bandwidth_total": 1024.0,
            "bandwidth_average": 512.0,
            "requests_total": 100,
            "requests_successful": 95,
            "requests_failed": 5,
            "error_reasons": [],
            "countries_used": {"US": 50, "GB": 50},
            "number_of_proxies_used": 10,
            "protocols_used": {"http": 80, "socks": 20},
            "average_concurrency": 5.5,
            "average_rps": 10.0,
            "last_request_sent_at": "2024-01-01T00:00:00Z"
        }"#;
        let stat: ProxyStat = serde_json::from_str(json).unwrap();
        assert_eq!(stat.requests_total, 100);
        assert!(!stat.is_projected);
    }
}
