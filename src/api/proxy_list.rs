//! Proxy list endpoints.
//!
//! Endpoints: List, Download, Refresh.
//! Base path: `/api/v2/proxy/list/`

use crate::{
    client::WebshareClient,
    error::Result,
    models::proxy::{ListProxiesParams, Proxy, ProxyDownloadParams},
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List proxies (paginated).
    ///
    /// `GET /api/v2/proxy/list/`
    pub async fn list_proxies(
        &self,
        params: &ListProxiesParams,
    ) -> Result<PaginatedResponse<Proxy>> {
        let req = self.get("/api/v2/proxy/list/").query(params);
        self.send_json(req).await
    }

    /// Download the proxy list as plain text.
    ///
    /// This endpoint is **unauthenticated** — the download token in the URL
    /// acts as the credential.
    ///
    /// `GET /api/v2/proxy/list/download/{token}/{country_codes}/any/{auth_method}/{endpoint_mode}/{search}/`
    pub async fn download_proxy_list(&self, params: &ProxyDownloadParams) -> Result<String> {
        let path = format!(
            "/api/v2/proxy/list/download/{}/{}/any/{}/{}/{}/",
            params.token,
            params.country_codes,
            params.authentication_method,
            params.endpoint_mode,
            params.search,
        );
        let mut req = self.get_unauthed(&path);
        if let Some(plan_id) = params.plan_id {
            req = req.query(&[("plan_id", plan_id.to_string())]);
        }
        self.send_text(req).await
    }

    /// Trigger an on-demand proxy list refresh.
    ///
    /// `POST /api/v2/proxy/list/refresh/`
    pub async fn refresh_proxy_list(&self, plan_id: Option<i64>) -> Result<()> {
        let mut req = self.post("/api/v2/proxy/list/refresh/");
        if let Some(pid) = plan_id {
            req = req.query(&[("plan_id", pid.to_string())]);
        }
        self.send_no_content(req).await
    }
}
