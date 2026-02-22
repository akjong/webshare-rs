//! API Key model types.

use serde::{Deserialize, Serialize};

/// An API key resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for creating an API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateApiKeyRequest {
    pub label: String,
}

/// Request body for updating an API key.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateApiKeyRequest {
    pub label: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_api_key() {
        let json = r#"{
            "id": 1,
            "key": "abc123",
            "label": "my-key",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let key: ApiKey = serde_json::from_str(json).unwrap();
        assert_eq!(key.id, 1);
        assert_eq!(key.label, "my-key");
    }

    #[test]
    fn serialize_create_request() {
        let req = CreateApiKeyRequest {
            label: "test".into(),
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"label\":\"test\""));
    }
}
