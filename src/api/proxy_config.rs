//! Proxy configuration endpoints.
//!
//! Endpoints: Get Config (v3), Get Stats (v3), Get Status (v3), Update (v2), Allocate Countries (v2).

use crate::{
    client::WebshareClient,
    error::Result,
    models::proxy_config::{
        AllocateCountriesRequest, ProxyConfig, ProxyConfigFull, ProxyListStats, ProxyListStatus,
        UpdateProxyConfigRequest,
    },
};

impl WebshareClient {
    /// Get proxy configuration for a plan.
    ///
    /// `GET /api/v3/proxy/config?plan_id={plan_id}`
    pub async fn get_proxy_config(&self, plan_id: i64) -> Result<ProxyConfig> {
        let req = self
            .get("/api/v3/proxy/config")
            .query(&[("plan_id", plan_id.to_string())]);
        self.send_json(req).await
    }

    /// Get proxy list statistics for a plan.
    ///
    /// `GET /api/v3/proxy/list/stats?plan_id={plan_id}`
    pub async fn get_proxy_list_stats(&self, plan_id: i64) -> Result<ProxyListStats> {
        let req = self
            .get("/api/v3/proxy/list/stats")
            .query(&[("plan_id", plan_id.to_string())]);
        self.send_json(req).await
    }

    /// Get proxy list status for a plan.
    ///
    /// `GET /api/v3/proxy/list/status?plan_id={plan_id}`
    pub async fn get_proxy_list_status(&self, plan_id: i64) -> Result<ProxyListStatus> {
        let req = self
            .get("/api/v3/proxy/list/status")
            .query(&[("plan_id", plan_id.to_string())]);
        self.send_json(req).await
    }

    /// Update proxy configuration.
    ///
    /// `PATCH /api/v2/proxy/config/`
    pub async fn update_proxy_config(
        &self,
        request: &UpdateProxyConfigRequest,
        plan_id: Option<i64>,
    ) -> Result<ProxyConfigFull> {
        let mut req = self.patch("/api/v2/proxy/config/").json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// Allocate unallocated countries to proxy configuration.
    ///
    /// `POST /api/v2/proxy/config/allocate_unallocated_countries/`
    pub async fn allocate_unallocated_countries(
        &self,
        request: &AllocateCountriesRequest,
        plan_id: Option<i64>,
    ) -> Result<ProxyConfigFull> {
        let mut req = self
            .post("/api/v2/proxy/config/allocate_unallocated_countries/")
            .json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }
}
