//! Two-factor authentication model types.

use serde::{Deserialize, Serialize};

/// A two-factor authentication method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorMethod {
    pub id: i64,
    /// One of `email_code` or `device_totp`.
    #[serde(rename = "type")]
    pub type_: String,
    pub active: bool,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for entering a 2FA code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enter2faCodeRequest {
    pub code: String,
    pub recaptcha: String,
}

/// Request body for changing the 2FA method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change2faMethodRequest {
    /// One of `email_code` or `device_totp`.
    #[serde(rename = "type")]
    pub type_: String,
}

/// Request body for activating a device TOTP 2FA method.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activate2faMethodRequest {
    pub code_1: String,
    pub code_2: String,
}

/// Response from resending a 2FA email.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resend2faEmailResponse {
    pub email_sent: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_two_factor_method() {
        let json = r#"{
            "id": 1,
            "type": "email_code",
            "active": true,
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let m: TwoFactorMethod = serde_json::from_str(json).unwrap();
        assert_eq!(m.type_, "email_code");
        assert!(m.active);
    }
}
