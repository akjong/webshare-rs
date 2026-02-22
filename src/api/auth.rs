//! Registration, login, and account management endpoints.
//!
//! Several endpoints in this group are **unauthenticated** (register, login,
//! reset password, activation complete). These use the `post_unauthed` /
//! `get_unauthed` helpers.

use crate::{
    client::WebshareClient,
    error::Result,
    models::auth::{
        ActivationCompleteRequest, ActivationStatus, ChangeEmailCompleteRequest,
        ChangeEmailRequest, ChangePasswordRequest, DeleteAccountRequest,
        DeleteSocialAccountRequest, LoginRequest, RegisterRequest, RegisterResponse,
        ResetPasswordCompleteRequest, ResetPasswordRequest, SocialAuthRequest, TokenResponse,
    },
};

impl WebshareClient {
    /// Register a new local account.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/register/`
    pub async fn register(&self, request: &RegisterRequest) -> Result<RegisterResponse> {
        let req = self.post_unauthed("/api/v2/register/").json(request);
        self.send_json(req).await
    }

    /// Login with a local account.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/login/`
    pub async fn login(&self, request: &LoginRequest) -> Result<TokenResponse> {
        let req = self.post_unauthed("/api/v2/login/").json(request);
        self.send_json(req).await
    }

    /// Register via a social provider (e.g. Google).
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/register/social/`
    pub async fn register_social(&self, request: &SocialAuthRequest) -> Result<RegisterResponse> {
        let req = self.post_unauthed("/api/v2/register/social/").json(request);
        self.send_json(req).await
    }

    /// Login via a social provider.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/login/social/`
    pub async fn login_social(&self, request: &SocialAuthRequest) -> Result<TokenResponse> {
        let req = self.post_unauthed("/api/v2/login/social/").json(request);
        self.send_json(req).await
    }

    /// Change the current user's password.
    ///
    /// `POST /api/v2/changepassword/`
    pub async fn change_password(&self, request: &ChangePasswordRequest) -> Result<()> {
        let req = self.post("/api/v2/changepassword/").json(request);
        self.send_no_content(req).await
    }

    /// Request a password reset email.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/resetpassword/`
    pub async fn reset_password(&self, request: &ResetPasswordRequest) -> Result<()> {
        let req = self.post_unauthed("/api/v2/resetpassword/").json(request);
        self.send_no_content(req).await
    }

    /// Complete a password reset with the token from email.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/resetpassword/complete/`
    pub async fn reset_password_complete(
        &self,
        request: &ResetPasswordCompleteRequest,
    ) -> Result<TokenResponse> {
        let req = self
            .post_unauthed("/api/v2/resetpassword/complete/")
            .json(request);
        self.send_json(req).await
    }

    /// Request an email change.
    ///
    /// `POST /api/v2/changeemail/`
    pub async fn change_email(&self, request: &ChangeEmailRequest) -> Result<()> {
        let req = self.post("/api/v2/changeemail/").json(request);
        self.send_no_content(req).await
    }

    /// Complete an email change with the confirmation code.
    ///
    /// `POST /api/v2/changeemail/complete/`
    pub async fn change_email_complete(&self, request: &ChangeEmailCompleteRequest) -> Result<()> {
        let req = self.post("/api/v2/changeemail/complete/").json(request);
        self.send_no_content(req).await
    }

    /// Get the current account activation status.
    ///
    /// `GET /api/v2/activation/`
    pub async fn get_activation_status(&self) -> Result<ActivationStatus> {
        let req = self.get("/api/v2/activation/");
        self.send_json(req).await
    }

    /// Resend the account activation email.
    ///
    /// `POST /api/v2/activation/resend/`
    pub async fn resend_activation_email(&self) -> Result<ActivationStatus> {
        let req = self
            .post("/api/v2/activation/resend/")
            .json(&serde_json::json!({}));
        self.send_json(req).await
    }

    /// Complete account activation with the token from email.
    ///
    /// **Unauthenticated.**
    ///
    /// `POST /api/v2/activation/complete/`
    pub async fn complete_activation(
        &self,
        request: &ActivationCompleteRequest,
    ) -> Result<TokenResponse> {
        let req = self
            .post_unauthed("/api/v2/activation/complete/")
            .json(request);
        self.send_json(req).await
    }

    /// Delete the current account.
    ///
    /// `POST /api/v2/deleteaccount/`
    pub async fn delete_account(&self, request: &DeleteAccountRequest) -> Result<()> {
        let req = self.post("/api/v2/deleteaccount/").json(request);
        self.send_no_content(req).await
    }

    /// Delete a social account connection.
    ///
    /// `POST /api/v2/deleteaccount/social/`
    pub async fn delete_social_account(&self, request: &DeleteSocialAccountRequest) -> Result<()> {
        let req = self.post("/api/v2/deleteaccount/social/").json(request);
        self.send_no_content(req).await
    }

    /// Logout the current session.
    ///
    /// `POST /api/v2/logout/`
    pub async fn logout(&self) -> Result<()> {
        let req = self.post("/api/v2/logout/");
        self.send_no_content(req).await
    }
}
