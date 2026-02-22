//! ID verification model types.

use serde::{Deserialize, Serialize};

/// ID verification state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdVerification {
    /// The `id` field is absent in some API responses (e.g. free plans).
    #[serde(default)]
    pub id: Option<i64>,
    /// One of: `not-required`, `requested`, `pending`, `processing`, `verified`, `failed`.
    pub state: String,
    pub client_secret: Option<String>,
    pub verification_failure_times: u32,
    pub max_verification_failure_times: u32,
    pub created_at: String,
    pub updated_at: String,
    pub verified_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_id_verification() {
        let json = r#"{
            "id": 1,
            "state": "not-required",
            "client_secret": null,
            "verification_failure_times": 0,
            "max_verification_failure_times": 0,
            "created_at": "2019-05-09T23:34:00.095501-07:00",
            "updated_at": "2019-05-09T23:34:00.095501-07:00",
            "verified_at": null
        }"#;
        let v: IdVerification = serde_json::from_str(json).unwrap();
        assert_eq!(v.state, "not-required");
        assert!(v.client_secret.is_none());
    }

    #[test]
    fn deserialize_id_verification_pending() {
        let json = r#"{
            "id": 1,
            "state": "pending",
            "client_secret": "aabbccc...zz",
            "verification_failure_times": 0,
            "max_verification_failure_times": 0,
            "created_at": "2019-05-09T23:34:00.095501-07:00",
            "updated_at": "2019-05-09T23:34:00.095501-07:00",
            "verified_at": null
        }"#;
        let v: IdVerification = serde_json::from_str(json).unwrap();
        assert_eq!(v.state, "pending");
        assert_eq!(v.client_secret.as_deref(), Some("aabbccc...zz"));
    }
}
