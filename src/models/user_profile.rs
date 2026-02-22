//! User profile and preferences model types.

use serde::{Deserialize, Serialize};

/// User profile.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: i64,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub last_login: String,
    pub timezone: String,
    pub subscribed_bandwidth_usage_notifications: bool,
    pub subscribed_subscription_notifications: bool,
    pub subscribed_proxy_usage_statistics: bool,
    pub subscribed_usage_warnings: bool,
    pub subscribed_guides_and_tips: bool,
    pub subscribed_survey_emails: bool,
    pub tracking_id: String,
}

/// Request body for updating user profile.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateProfileRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_bandwidth_usage_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_subscription_notifications: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_proxy_usage_statistics: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_usage_warnings: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_guides_and_tips: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribed_survey_emails: Option<bool>,
}

/// User preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub id: i64,
    pub customer_satisfaction_survey_last_dismissed_at: Option<String>,
    pub customer_satisfaction_survey_last_completed_at: Option<String>,
    pub onboarding_activity_page_viewed_at: Option<String>,
}

/// Request body for updating user preferences.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdatePreferencesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_satisfaction_survey_last_dismissed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_satisfaction_survey_last_completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_activity_page_viewed_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_user_profile() {
        let json = r#"{
            "id": 1,
            "email": "test@example.com",
            "first_name": "John",
            "last_name": "Doe",
            "last_login": "2024-01-01T00:00:00Z",
            "timezone": "UTC",
            "subscribed_bandwidth_usage_notifications": true,
            "subscribed_subscription_notifications": true,
            "subscribed_proxy_usage_statistics": false,
            "subscribed_usage_warnings": true,
            "subscribed_guides_and_tips": false,
            "subscribed_survey_emails": false,
            "tracking_id": "abc123"
        }"#;
        let profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.email, "test@example.com");
    }
}
