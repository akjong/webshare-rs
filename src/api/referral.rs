//! Referral and affiliate endpoints.
//!
//! Includes referral configuration, public referral code info,
//! referral credits, and earn-outs.

use crate::{
    client::WebshareClient,
    error::Result,
    models::referral::{
        ReferralCodeInfo, ReferralConfig, ReferralCredit, ReferralEarnOut,
        UpdateReferralConfigRequest,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    // ── Referral Config ─────────────────────────────────────────────

    /// Get the referral configuration.
    ///
    /// `GET /api/v2/referral/config/`
    pub async fn get_referral_config(&self) -> Result<ReferralConfig> {
        let req = self.get("/api/v2/referral/config/");
        self.send_json(req).await
    }

    /// Update referral configuration.
    ///
    /// Only `mode` and `paypal_payout_email` are writable.
    ///
    /// `PATCH /api/v2/referral/config/`
    pub async fn update_referral_config(
        &self,
        request: &UpdateReferralConfigRequest,
    ) -> Result<ReferralConfig> {
        let req = self.patch("/api/v2/referral/config/").json(request);
        self.send_json(req).await
    }

    // ── Referral Code Info (Public / Unauthenticated) ───────────────

    /// Get public information about a referral code.
    ///
    /// This endpoint does **not** require authentication.
    ///
    /// `GET /api/v2/referral/code/info/?referral_code={code}`
    pub async fn get_referral_code_info(&self, referral_code: &str) -> Result<ReferralCodeInfo> {
        let req = self
            .get_unauthed("/api/v2/referral/code/info/")
            .query(&[("referral_code", referral_code)]);
        self.send_json(req).await
    }

    // ── Referral Credits ────────────────────────────────────────────

    /// List referral credits (paginated).
    ///
    /// `GET /api/v2/referral/credit/`
    pub async fn list_referral_credits(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<ReferralCredit>> {
        let mut req = self.get("/api/v2/referral/credit/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get a referral credit by ID.
    ///
    /// `GET /api/v2/referral/credit/{id}/`
    pub async fn get_referral_credit(&self, id: i64) -> Result<ReferralCredit> {
        let req = self.get(&format!("/api/v2/referral/credit/{id}/"));
        self.send_json(req).await
    }

    // ── Referral Earn-Outs ──────────────────────────────────────────

    /// List referral earn-outs (paginated).
    ///
    /// `GET /api/v2/referral/earnout/`
    pub async fn list_referral_earnouts(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<ReferralEarnOut>> {
        let mut req = self.get("/api/v2/referral/earnout/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Get a referral earn-out by ID.
    ///
    /// `GET /api/v2/referral/earnout/{id}/`
    pub async fn get_referral_earnout(&self, id: i64) -> Result<ReferralEarnOut> {
        let req = self.get(&format!("/api/v2/referral/earnout/{id}/"));
        self.send_json(req).await
    }
}
