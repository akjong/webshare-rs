//! Two-factor authentication endpoints.
//!
//! Endpoints: Enter Code, Get Method, Change Method, Activate Method, Resend Email.

use crate::{
    client::WebshareClient,
    error::Result,
    models::two_factor_auth::{
        Activate2faMethodRequest, Change2faMethodRequest, Enter2faCodeRequest,
        Resend2faEmailResponse, TwoFactorMethod,
    },
};

impl WebshareClient {
    /// Enter a 2FA code for authentication.
    ///
    /// `POST /api/v2/twofactorauth/codeauth/`
    pub async fn enter_2fa_code(&self, request: &Enter2faCodeRequest) -> Result<()> {
        let req = self.post("/api/v2/twofactorauth/codeauth/").json(request);
        self.send_no_content(req).await
    }

    /// Get the current 2FA method.
    ///
    /// `GET /api/v2/twofactorauth/method/current/`
    pub async fn get_2fa_method(&self) -> Result<TwoFactorMethod> {
        let req = self.get("/api/v2/twofactorauth/method/current/");
        self.send_json(req).await
    }

    /// Change the 2FA method.
    ///
    /// `POST /api/v2/twofactorauth/method/`
    pub async fn change_2fa_method(
        &self,
        request: &Change2faMethodRequest,
    ) -> Result<TwoFactorMethod> {
        let req = self.post("/api/v2/twofactorauth/method/").json(request);
        self.send_json(req).await
    }

    /// Activate a device TOTP 2FA method with two consecutive codes.
    ///
    /// `POST /api/v2/twofactorauth/method/{id}/activate/`
    pub async fn activate_2fa_method(
        &self,
        id: i64,
        request: &Activate2faMethodRequest,
    ) -> Result<TwoFactorMethod> {
        let req = self
            .post(&format!("/api/v2/twofactorauth/method/{id}/activate/"))
            .json(request);
        self.send_json(req).await
    }

    /// Resend 2FA email code.
    ///
    /// `POST /api/v2/twofactorauth/email/resend/`
    pub async fn resend_2fa_email(&self) -> Result<Resend2faEmailResponse> {
        let req = self.post("/api/v2/twofactorauth/email/resend/");
        self.send_json(req).await
    }
}
