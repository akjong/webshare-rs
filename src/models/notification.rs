//! Notification model types.

use serde::{Deserialize, Serialize};

/// A notification resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub id: i64,
    #[serde(rename = "type")]
    pub type_: String,
    pub is_dismissable: bool,
    pub context: serde_json::Value,
    pub created_at: String,
    pub updated_at: String,
    pub dismissed_at: Option<String>,
}

/// Query parameters for listing notifications.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListNotificationsParams {
    #[serde(
        rename = "dismissed_at__isnull",
        skip_serializing_if = "Option::is_none"
    )]
    pub dismissed_at_isnull: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_notification() {
        let json = r#"{
            "id": 1,
            "type": "plan_expiring",
            "is_dismissable": true,
            "context": {"plan": 42},
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "dismissed_at": null
        }"#;
        let n: Notification = serde_json::from_str(json).unwrap();
        assert_eq!(n.type_, "plan_expiring");
        assert!(n.dismissed_at.is_none());
    }
}
