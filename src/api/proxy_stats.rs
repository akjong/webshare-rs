//! Proxy statistics and activity endpoints.
//!
//! Endpoints: List Stats, Aggregate Stats, List Activities, Download Activities.

use crate::{
    client::WebshareClient,
    error::Result,
    models::proxy_stats::{
        AggregateStats, DownloadActivitiesParams, ListActivitiesParams, ListStatsParams,
        ProxyActivity, ProxyStat,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List proxy statistics.
    ///
    /// **Note:** This endpoint returns a plain array, not a paginated response.
    ///
    /// `GET /api/v2/stats/`
    pub async fn list_stats(&self, params: &ListStatsParams) -> Result<Vec<ProxyStat>> {
        let req = self.get("/api/v2/stats/").query(params);
        self.send_json(req).await
    }

    /// Get aggregate proxy statistics.
    ///
    /// `GET /api/v2/stats/aggregate/`
    pub async fn aggregate_stats(&self, params: &ListStatsParams) -> Result<AggregateStats> {
        let req = self.get("/api/v2/stats/aggregate/").query(params);
        self.send_json(req).await
    }

    /// List proxy activity records (paginated).
    ///
    /// `GET /api/v2/proxy/activity/`
    pub async fn list_activities(
        &self,
        params: &ListActivitiesParams,
    ) -> Result<PaginatedResponse<ProxyActivity>> {
        let req = self.get("/api/v2/proxy/activity/").query(params);
        self.send_json(req).await
    }

    /// Download proxy activity as CSV.
    ///
    /// This endpoint is **unauthenticated** — the download token acts as the
    /// credential.
    ///
    /// `GET /api/v2/proxy/activity/download/`
    pub async fn download_activities(&self, params: &DownloadActivitiesParams) -> Result<String> {
        let req = self
            .get_unauthed("/api/v2/proxy/activity/download/")
            .query(params);
        self.send_text(req).await
    }
}
