//! API Key endpoints.
//!
//! Endpoints: Create, List, Retrieve, Update, Delete.
//! Base path: `/api/v2/apikey/`

use crate::{
    client::WebshareClient,
    error::Result,
    models::api_key::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest},
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// Create a new API key.
    ///
    /// `POST /api/v2/apikey/`
    pub async fn create_api_key(&self, request: &CreateApiKeyRequest) -> Result<ApiKey> {
        let req = self.post("/api/v2/apikey/").json(request);
        self.send_json(req).await
    }

    /// List all API keys (paginated).
    ///
    /// `GET /api/v2/apikey/`
    pub async fn list_api_keys(
        &self,
        page: Option<u32>,
        page_size: Option<u32>,
    ) -> Result<PaginatedResponse<ApiKey>> {
        let mut req = self.get("/api/v2/apikey/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        if let Some(ps) = page_size {
            req = req.query(&[("page_size", ps.to_string())]);
        }
        self.send_json(req).await
    }

    /// Retrieve a single API key by ID.
    ///
    /// `GET /api/v2/apikey/{id}/`
    pub async fn get_api_key(&self, id: i64) -> Result<ApiKey> {
        let req = self.get(&format!("/api/v2/apikey/{id}/"));
        self.send_json(req).await
    }

    /// Update an API key.
    ///
    /// `PATCH /api/v2/apikey/{id}/`
    pub async fn update_api_key(&self, id: i64, request: &UpdateApiKeyRequest) -> Result<ApiKey> {
        let req = self.patch(&format!("/api/v2/apikey/{id}/")).json(request);
        self.send_json(req).await
    }

    /// Delete an API key.
    ///
    /// `DELETE /api/v2/apikey/{id}/`
    pub async fn delete_api_key(&self, id: i64) -> Result<()> {
        let req = self.delete(&format!("/api/v2/apikey/{id}/"));
        self.send_no_content(req).await
    }
}
