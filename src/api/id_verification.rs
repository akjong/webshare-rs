//! ID verification endpoints.
//!
//! Endpoints for retrieving, starting, and completing Stripe-based ID verification.

use crate::{client::WebshareClient, error::Result, models::id_verification::IdVerification};

impl WebshareClient {
    /// Get the current ID verification state.
    ///
    /// `GET /api/v2/idverification/`
    pub async fn get_id_verification(&self) -> Result<IdVerification> {
        let req = self.get("/api/v2/idverification/");
        self.send_json(req).await
    }

    /// Start an ID verification flow.
    ///
    /// The verification state must be `requested` or `failed` (with
    /// `verification_failure_times < max_verification_failure_times`).
    /// On success, state becomes `pending` and `client_secret` is populated.
    ///
    /// `POST /api/v2/idverification/start/`
    pub async fn start_id_verification(&self) -> Result<IdVerification> {
        let req = self
            .post("/api/v2/idverification/start/")
            .json(&serde_json::json!({}));
        self.send_json(req).await
    }

    /// Complete an ID verification flow.
    ///
    /// The verification state must be `pending`. On success, state becomes `processing`.
    /// Returns a 400 error if the Stripe JS verification is not complete.
    ///
    /// `POST /api/v2/idverification/complete/`
    pub async fn complete_id_verification(&self) -> Result<IdVerification> {
        let req = self
            .post("/api/v2/idverification/complete/")
            .json(&serde_json::json!({}));
        self.send_json(req).await
    }
}
