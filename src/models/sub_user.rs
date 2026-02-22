//! Sub-user model types.

use serde::{Deserialize, Serialize};

/// A sub-user resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubUser {
    pub id: i64,
    pub label: String,
    pub proxy_countries: Option<serde_json::Value>,
    pub proxy_limit: f64,
    pub max_thread_count: i64,
    /// Aggregate statistics for this sub-user (same shape as aggregate stats).
    pub aggregate_stats: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    pub bandwidth_use_start_date: String,
    pub bandwidth_use_end_date: String,
}

/// Request body for creating a sub-user.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubUserRequest {
    pub label: String,
    pub proxy_limit: f64,
    pub max_thread_count: i64,
}

/// Request body for updating a sub-user.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateSubUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_limit: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_thread_count: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_sub_user() {
        let json = r#"{
            "id": 1,
            "label": "test-user",
            "proxy_countries": null,
            "proxy_limit": 100.0,
            "max_thread_count": 10,
            "aggregate_stats": {},
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "bandwidth_use_start_date": "2024-01-01T00:00:00Z",
            "bandwidth_use_end_date": "2024-02-01T00:00:00Z"
        }"#;
        let user: SubUser = serde_json::from_str(json).unwrap();
        assert_eq!(user.label, "test-user");
    }
}
