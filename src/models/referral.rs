//! Referral and affiliate model types.

use serde::{Deserialize, Serialize};

/// Referral configuration object.
///
/// Describes the current state of the user's referral/affiliate system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralConfig {
    pub id: i64,
    /// `payout` or `credits`.
    pub mode: String,
    pub paypal_payout_email: Option<String>,
    pub id_verification_required: bool,
    pub credits_earned: f64,
    pub payouts_earned: f64,
    pub total_credits_earned: f64,
    pub total_payouts_earned: f64,
    pub number_of_users_referred: u64,
    pub number_of_users_upgraded: u64,
    /// Frequency of earn-outs. Format: `[DD] [HH:MM:SS]`.
    pub earn_out_frequency: String,
    pub next_earn_out_date: Option<String>,
    pub minimum_earn_out_amount: f64,
    pub referral_code: String,
    /// One of: `first_time_value_off`, `first_time_percent_off`,
    /// `always_value_off`, `always_percent_off`, or `null`.
    pub promo_type: Option<String>,
    pub promo_value: Option<i32>,
    pub referral_url: String,
    pub referral_maximum_credits: f64,
    pub referral_credit_ratio: f64,
    /// Grace period. Format: `[DD] [HH:MM:SS]`.
    pub referral_payment_pending_days: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request to update referral config.
///
/// Only `mode` and `paypal_payout_email` are writable.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateReferralConfigRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal_payout_email: Option<String>,
}

/// Public information about a referral code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCodeInfo {
    pub referral_code: String,
    pub promo_type: Option<String>,
    pub promo_value: Option<i32>,
}

/// A referral credit earned when a referred user spends money.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralCredit {
    pub id: i64,
    pub user_id: i64,
    /// `payout` or `credit`.
    pub mode: String,
    pub amount: f64,
    /// `pending`, `available`, or `reverted`.
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub reverted_at: Option<String>,
}

/// A referral earn-out record.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralEarnOut {
    pub id: i64,
    /// `payout` or `credit`.
    pub mode: String,
    pub paypal_payout_email: Option<String>,
    pub amount: f64,
    /// `processing`, `completed`, or `failed`.
    pub status: String,
    pub error_reason: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_referral_config() {
        let json = r#"{
            "id": 1,
            "mode": "payout",
            "paypal_payout_email": "paypal@webshare.io",
            "id_verification_required": false,
            "credits_earned": 0.0,
            "payouts_earned": 0.0,
            "total_credits_earned": 0.0,
            "total_payouts_earned": 0.0,
            "number_of_users_referred": 0,
            "number_of_users_upgraded": 0,
            "earn_out_frequency": "7 00:00:00",
            "next_earn_out_date": "2022-06-14T11:58:10.246406-07:00",
            "minimum_earn_out_amount": 10,
            "referral_code": "78saf89712",
            "referral_url": "https://www.webshare.io/?referral_code=78saf89712",
            "referral_maximum_credits": 100,
            "referral_credit_ratio": 0.25,
            "referral_payment_pending_days": "30 00:00:00",
            "promo_type": "first_time_value_off",
            "promo_value": 10,
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-15T11:58:10.246406-07:00"
        }"#;
        let cfg: ReferralConfig = serde_json::from_str(json).unwrap();
        assert_eq!(cfg.mode, "payout");
        assert_eq!(cfg.referral_code, "78saf89712");
    }

    #[test]
    fn deserialize_referral_code_info() {
        let json = r#"{
            "promo_type": "first_time_value_off",
            "promo_value": 10,
            "referral_code": "a8b192klkwncvk"
        }"#;
        let info: ReferralCodeInfo = serde_json::from_str(json).unwrap();
        assert_eq!(info.referral_code, "a8b192klkwncvk");
    }

    #[test]
    fn deserialize_referral_credit() {
        let json = r#"{
            "id": 1,
            "user_id": 6124,
            "mode": "credits",
            "amount": 2.50,
            "status": "pending",
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00",
            "reverted_at": null
        }"#;
        let credit: ReferralCredit = serde_json::from_str(json).unwrap();
        assert_eq!(credit.status, "pending");
        assert!(credit.reverted_at.is_none());
    }

    #[test]
    fn deserialize_referral_earnout() {
        let json = r#"{
            "id": 1,
            "mode": "credits",
            "paypal_payout_email": null,
            "amount": 2.50,
            "status": "completed",
            "error_reason": null,
            "created_at": "2022-06-14T11:58:10.246406-07:00",
            "updated_at": "2022-06-14T11:58:10.246406-07:00"
        }"#;
        let eo: ReferralEarnOut = serde_json::from_str(json).unwrap();
        assert_eq!(eo.status, "completed");
    }
}
