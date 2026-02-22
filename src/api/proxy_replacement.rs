//! Proxy replacement endpoints.
//!
//! Endpoints: Create Replacement, List Replacements, Get Replacement,
//! List Replaced Proxies, Download Replaced Proxies.

use crate::{
    client::WebshareClient,
    error::Result,
    models::proxy_replacement::{
        CreateReplacementRequest, DownloadReplacedProxiesParams, ListReplacedProxiesParams,
        ListReplacementsParams, ProxyReplacement, ReplacedProxy,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// Create a proxy replacement operation.
    ///
    /// `POST /api/v3/proxy/replace/`
    pub async fn create_replacement(
        &self,
        request: &CreateReplacementRequest,
        plan_id: Option<i64>,
    ) -> Result<ProxyReplacement> {
        let mut req = self.post("/api/v3/proxy/replace/").json(request);
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// List proxy replacement operations (paginated).
    ///
    /// `GET /api/v3/proxy/replace/`
    pub async fn list_replacements(
        &self,
        params: &ListReplacementsParams,
    ) -> Result<PaginatedResponse<ProxyReplacement>> {
        let req = self.get("/api/v3/proxy/replace/").query(params);
        self.send_json(req).await
    }

    /// Retrieve a single proxy replacement by ID.
    ///
    /// `GET /api/v3/proxy/replace/{id}/`
    pub async fn get_replacement(&self, id: i64, plan_id: Option<i64>) -> Result<ProxyReplacement> {
        let mut req = self.get(&format!("/api/v3/proxy/replace/{id}/"));
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_json(req).await
    }

    /// List replaced proxies (paginated).
    ///
    /// `GET /api/v2/proxy/list/replaced/`
    pub async fn list_replaced_proxies(
        &self,
        params: &ListReplacedProxiesParams,
    ) -> Result<PaginatedResponse<ReplacedProxy>> {
        let req = self.get("/api/v2/proxy/list/replaced/").query(params);
        self.send_json(req).await
    }

    /// Download replaced proxy list as plain text.
    ///
    /// This endpoint is **unauthenticated** — the download token acts as the
    /// credential.
    ///
    /// `GET /api/v2/proxy/list/replaced/download/`
    pub async fn download_replaced_proxies(
        &self,
        params: &DownloadReplacedProxiesParams,
    ) -> Result<String> {
        let req = self
            .get_unauthed("/api/v2/proxy/list/replaced/download/")
            .query(params);
        self.send_text(req).await
    }
}
