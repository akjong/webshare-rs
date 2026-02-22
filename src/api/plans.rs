//! Plan endpoints.
//!
//! Endpoints: List Plans, Get Plan, Update Plan, Upgrade Plan, Cancel Plan.

use crate::{
    client::WebshareClient,
    error::Result,
    models::plan::{
        CancelPlanResponse, Plan, UpdatePlanRequest, UpgradePlanRequest, UpgradePlanResponse,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List subscription plans (paginated).
    ///
    /// `GET /api/v2/subscription/plan/`
    pub async fn list_plans(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<Plan>> {
        let mut req = self.get("/api/v2/subscription/plan/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(ps) = page_size {
            req = req.query(&[("page_size", ps.to_string())]);
        }
        self.send_json(req).await
    }

    /// Retrieve a single plan by ID.
    ///
    /// `GET /api/v2/subscription/plan/{id}/`
    pub async fn get_plan(&self, id: i64) -> Result<Plan> {
        let req = self.get(&format!("/api/v2/subscription/plan/{id}/"));
        self.send_json(req).await
    }

    /// Update a plan.
    ///
    /// `PATCH /api/v2/subscription/plan/{id}/`
    pub async fn update_plan(&self, id: i64, request: &UpdatePlanRequest) -> Result<Plan> {
        let req = self
            .patch(&format!("/api/v2/subscription/plan/{id}/"))
            .json(request);
        self.send_json(req).await
    }

    /// Upgrade a plan.
    ///
    /// `POST /api/v2/subscription/plan/{id}/upgrade/`
    pub async fn upgrade_plan(
        &self,
        id: i64,
        request: &UpgradePlanRequest,
    ) -> Result<UpgradePlanResponse> {
        let req = self
            .post(&format!("/api/v2/subscription/plan/{id}/upgrade/"))
            .json(request);
        self.send_json(req).await
    }

    /// Cancel a plan.
    ///
    /// `POST /api/v2/subscription/plan/{id}/cancel/`
    pub async fn cancel_plan(&self, id: i64) -> Result<CancelPlanResponse> {
        let req = self
            .post(&format!("/api/v2/subscription/plan/{id}/cancel/"))
            .json(&serde_json::json!({}));
        self.send_json(req).await
    }
}
