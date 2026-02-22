//! IP authorization endpoints.
//!
//! Endpoints: Create, List, Retrieve, Delete, What's My IP.

use crate::{
    client::WebshareClient,
    error::Result,
    models::ip_authorization::{CreateIpAuthRequest, IpAuthorization, WhatsMyIpResponse},
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// Create a new IP authorization.
    ///
    /// `POST /api/v2/proxy/ipauthorization/`
    pub async fn create_ip_authorization(
        &self,
        request: &CreateIpAuthRequest,
        plan_id: Option<i64>,
    ) -> Result<IpAuthorization> {
        let mut req = self.post("/api/v2/proxy/ipauthorization/").json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// List IP authorizations (paginated).
    ///
    /// `GET /api/v2/proxy/ipauthorization/`
    pub async fn list_ip_authorizations(
        &self,
        plan_id: Option<i64>,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<IpAuthorization>> {
        let mut req = self.get("/api/v2/proxy/ipauthorization/");
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Retrieve a single IP authorization.
    ///
    /// `GET /api/v2/proxy/ipauthorization/{id}/`
    pub async fn get_ip_authorization(
        &self,
        id: i64,
        plan_id: Option<i64>,
    ) -> Result<IpAuthorization> {
        let mut req = self.get(&format!("/api/v2/proxy/ipauthorization/{id}/"));
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Delete an IP authorization.
    ///
    /// `DELETE /api/v2/proxy/ipauthorization/{id}/`
    pub async fn delete_ip_authorization(&self, id: i64, plan_id: Option<i64>) -> Result<()> {
        let mut req = self.delete(&format!("/api/v2/proxy/ipauthorization/{id}/"));
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_no_content(req).await
    }

    /// Get the current public IP address.
    ///
    /// `GET /api/v2/proxy/ipauthorization/whatsmyip/`
    pub async fn whats_my_ip(&self) -> Result<WhatsMyIpResponse> {
        let req = self.get("/api/v2/proxy/ipauthorization/whatsmyip/");
        self.send_json(req).await
    }
}
