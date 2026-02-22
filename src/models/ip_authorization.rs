//! IP authorization model types.

use serde::{Deserialize, Serialize};

/// An IP authorization entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpAuthorization {
    pub id: i64,
    pub ip_address: String,
    pub created_at: String,
    pub last_used_at: Option<String>,
}

/// Request body for creating an IP authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIpAuthRequest {
    pub ip_address: String,
}

/// Response from the "What's My IP" endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatsMyIpResponse {
    pub ip_address: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_ip_authorization() {
        let json = r#"{
            "id": 1,
            "ip_address": "1.2.3.4",
            "created_at": "2024-01-01T00:00:00Z",
            "last_used_at": null
        }"#;
        let ip: IpAuthorization = serde_json::from_str(json).unwrap();
        assert_eq!(ip.ip_address, "1.2.3.4");
        assert!(ip.last_used_at.is_none());
    }
}
