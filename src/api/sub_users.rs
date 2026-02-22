//! Sub-user endpoints.
//!
//! Endpoints: List, Retrieve, Create, Update, Delete, Refresh Proxy List, Masquerade helper.

use crate::{
    client::WebshareClient,
    error::Result,
    models::sub_user::{CreateSubUserRequest, SubUser, UpdateSubUserRequest},
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List sub-users (paginated).
    ///
    /// `GET /api/v2/subuser/`
    pub async fn list_sub_users(
        &self,
        page: Option<u32>,
        plan_id: Option<i64>,
    ) -> Result<PaginatedResponse<SubUser>> {
        let mut req = self.get("/api/v2/subuser/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Retrieve a single sub-user by ID.
    ///
    /// `GET /api/v2/subuser/{id}/`
    pub async fn get_sub_user(&self, id: i64, plan_id: Option<i64>) -> Result<SubUser> {
        let mut req = self.get(&format!("/api/v2/subuser/{id}/"));
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Create a new sub-user.
    ///
    /// `POST /api/v2/subuser/`
    pub async fn create_sub_user(
        &self,
        request: &CreateSubUserRequest,
        plan_id: Option<i64>,
    ) -> Result<SubUser> {
        let mut req = self.post("/api/v2/subuser/").json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Update a sub-user.
    ///
    /// `PATCH /api/v2/subuser/{id}/`
    pub async fn update_sub_user(
        &self,
        id: i64,
        request: &UpdateSubUserRequest,
        plan_id: Option<i64>,
    ) -> Result<SubUser> {
        let mut req = self.patch(&format!("/api/v2/subuser/{id}/")).json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Delete a sub-user.
    ///
    /// `DELETE /api/v2/subuser/{id}/`
    pub async fn delete_sub_user(&self, id: i64, plan_id: Option<i64>) -> Result<()> {
        let mut req = self.delete(&format!("/api/v2/subuser/{id}/"));
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_no_content(req).await
    }

    /// Refresh a sub-user's proxy list.
    ///
    /// `POST /api/v2/subuser/{id}/refresh/`
    pub async fn refresh_sub_user_proxy_list(&self, id: i64) -> Result<SubUser> {
        let req = self.post(&format!("/api/v2/subuser/{id}/refresh/"));
        self.send_json(req).await
    }

    /// Generate the masquerade header for a sub-user.
    ///
    /// Add `X-Subuser: {sub_user_id}` to requests to act on behalf of a
    /// sub-user. Returns the header name-value pair for manual injection.
    pub fn masquerade_header(sub_user_id: i64) -> (&'static str, String) {
        ("X-Subuser", sub_user_id.to_string())
    }
}
