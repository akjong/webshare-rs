//! Download token endpoints.
//!
//! Manage download tokens for proxy list, replaced proxy, and activity downloads.

use crate::{
    client::WebshareClient,
    error::Result,
    models::download::{DownloadToken, DownloadTokenScope},
};

impl WebshareClient {
    /// Get a download token for the given scope.
    ///
    /// Note: The API docs show `GET` but the example uses `POST`.
    /// This implementation uses `POST` to match the example behavior.
    ///
    /// `POST /api/v2/download_token/{scope}/`
    pub async fn get_download_token(&self, scope: DownloadTokenScope) -> Result<DownloadToken> {
        let req = self.post(&format!("/api/v2/download_token/{scope}/"));
        self.send_json(req).await
    }

    /// Reset a download token for the given scope.
    ///
    /// `POST /api/v2/download_token/{scope}/reset/`
    pub async fn reset_download_token(&self, scope: DownloadTokenScope) -> Result<DownloadToken> {
        let req = self.post(&format!("/api/v2/download_token/{scope}/reset/"));
        self.send_json(req).await
    }
}
