//! Authentication model types (registration, login, password, email, account).

use serde::{Deserialize, Serialize};

/// Token response (returned by login and some auth flows).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub token: String,
}

/// Register response (includes login-existing-user flag).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterResponse {
    pub token: String,
    pub logged_in_existing_user: bool,
}

/// Request body for local registration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub recaptcha: String,
    pub tos_accepted: bool,
    pub marketing_email_accepted: bool,
}

/// Request body for local login.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub recaptcha: String,
}

/// Request body for social authentication (register or login).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialAuthRequest {
    pub provider: String,
    pub code: String,
    pub redirect_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_accepted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketing_email_accepted: Option<bool>,
}

/// Request body for changing password.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangePasswordRequest {
    pub password: String,
    pub new_password: String,
}

/// Request body for requesting a password reset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub recaptcha: String,
}

/// Request body for completing a password reset.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResetPasswordCompleteRequest {
    pub password: String,
    pub password_reset_token: String,
    pub recaptcha: String,
}

/// Request body for requesting an email change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEmailRequest {
    pub password: String,
    pub new_email: String,
}

/// Request body for completing an email change.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeEmailCompleteRequest {
    pub confirmation_code: String,
}

/// Account activation status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationStatus {
    pub email_is_verified: bool,
    pub last_time_email_verification_email_sent: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for completing account activation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivationCompleteRequest {
    pub activation_token: String,
}

/// Request body for deleting an account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteAccountRequest {
    pub password: String,
    pub recaptcha: String,
}

/// Request body for deleting a social account.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteSocialAccountRequest {
    pub provider: String,
    pub code: String,
    pub redirect_uri: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_token_response() {
        let json = r#"{"token": "abc123"}"#;
        let resp: TokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.token, "abc123");
    }

    #[test]
    fn deserialize_register_response() {
        let json = r#"{"token": "abc123", "logged_in_existing_user": false}"#;
        let resp: RegisterResponse = serde_json::from_str(json).unwrap();
        assert!(!resp.logged_in_existing_user);
    }

    #[test]
    fn serialize_register_request() {
        let req = RegisterRequest {
            email: "test@example.com".into(),
            password: "pass".into(),
            recaptcha: "token".into(),
            tos_accepted: true,
            marketing_email_accepted: false,
        };
        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("tos_accepted"));
    }
}
