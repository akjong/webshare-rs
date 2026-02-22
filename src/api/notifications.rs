//! Notification endpoints.
//!
//! Endpoints: List, Retrieve, Dismiss, Restore.

use crate::{
    client::WebshareClient,
    error::Result,
    models::notification::{ListNotificationsParams, Notification},
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List notifications (paginated).
    ///
    /// `GET /api/v2/notification/`
    pub async fn list_notifications(
        &self,
        params: &ListNotificationsParams,
    ) -> Result<PaginatedResponse<Notification>> {
        let req = self.get("/api/v2/notification/").query(params);
        self.send_json(req).await
    }

    /// Retrieve a single notification.
    ///
    /// `GET /api/v2/notification/{id}/`
    pub async fn get_notification(&self, id: i64) -> Result<Notification> {
        let req = self.get(&format!("/api/v2/notification/{id}/"));
        self.send_json(req).await
    }

    /// Dismiss a notification.
    ///
    /// `POST /api/v2/notification/{id}/dismiss/`
    pub async fn dismiss_notification(&self, id: i64) -> Result<Notification> {
        let req = self.post(&format!("/api/v2/notification/{id}/dismiss/"));
        self.send_json(req).await
    }

    /// Restore a dismissed notification.
    ///
    /// `POST /api/v2/notification/{id}/restore/`
    pub async fn restore_notification(&self, id: i64) -> Result<Notification> {
        let req = self.post(&format!("/api/v2/notification/{id}/restore/"));
        self.send_json(req).await
    }
}
