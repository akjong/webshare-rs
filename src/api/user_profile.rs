//! User profile and preferences endpoints.
//!
//! Endpoints: Get Profile, Update Profile, Get Preferences, Update Preferences.

use crate::{
    client::WebshareClient,
    error::Result,
    models::user_profile::{
        UpdatePreferencesRequest, UpdateProfileRequest, UserPreferences, UserProfile,
    },
};

impl WebshareClient {
    /// Get the current user's profile.
    ///
    /// `GET /api/v2/profile/`
    pub async fn get_profile(&self) -> Result<UserProfile> {
        let req = self.get("/api/v2/profile/");
        self.send_json(req).await
    }

    /// Update the current user's profile.
    ///
    /// `PATCH /api/v2/profile/`
    pub async fn update_profile(&self, request: &UpdateProfileRequest) -> Result<UserProfile> {
        let req = self.patch("/api/v2/profile/").json(request);
        self.send_json(req).await
    }

    /// Get the current user's preferences.
    ///
    /// `GET /api/v2/profile/preferences/`
    pub async fn get_preferences(&self) -> Result<UserPreferences> {
        let req = self.get("/api/v2/profile/preferences/");
        self.send_json(req).await
    }

    /// Update the current user's preferences.
    ///
    /// `PATCH /api/v2/profile/preferences/`
    pub async fn update_preferences(
        &self,
        request: &UpdatePreferencesRequest,
    ) -> Result<UserPreferences> {
        let req = self.patch("/api/v2/profile/preferences/").json(request);
        self.send_json(req).await
    }
}
