//! Account verification endpoints.
//!
//! Includes verification flows, evidence submission, questions/answers,
//! categories, limits, thresholds, suspension, abuse reports, and appeals.

use crate::{
    client::WebshareClient,
    error::Result,
    models::verification::{
        AbuseReport, Appeal, ListQuestionsParams, SubmitAppealRequest, SubmitSecurityCodeRequest,
        Suspension, VerificationAnswer, VerificationFlow, VerificationLimits, VerificationQuestion,
    },
    pagination::PaginatedResponse,
};

impl WebshareClient {
    /// List verification flows (paginated).
    ///
    /// `GET /api/v2/verification/flow/`
    pub async fn list_verification_flows(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<VerificationFlow>> {
        let mut req = self.get("/api/v2/verification/flow/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Retrieve a single verification flow.
    ///
    /// `GET /api/v2/verification/flow/{id}/`
    pub async fn get_verification_flow(&self, id: i64) -> Result<VerificationFlow> {
        let req = self.get(&format!("/api/v2/verification/flow/{id}/"));
        self.send_json(req).await
    }

    /// Submit evidence for a verification flow (multipart form).
    ///
    /// `POST /api/v2/verification/flow/{flow_id}/submit_evidence/`
    pub async fn submit_evidence(
        &self,
        flow_id: i64,
        explanation: &str,
        files: Vec<(String, Vec<u8>)>,
    ) -> Result<VerificationFlow> {
        let mut form = hpx::multipart::Form::new().text("explanation", explanation.to_owned());
        for (filename, data) in files {
            let part = hpx::multipart::Part::bytes(data).file_name(filename);
            form = form.part("files", part);
        }
        let req = self
            .post(&format!(
                "/api/v2/verification/flow/{flow_id}/submit_evidence/"
            ))
            .multipart(form);
        self.send_json(req).await
    }

    /// Submit a security code for a verification flow.
    ///
    /// `POST /api/v2/verification/flow/{flow_id}/submit_verification_code/`
    pub async fn submit_security_code(
        &self,
        flow_id: i64,
        request: &SubmitSecurityCodeRequest,
    ) -> Result<VerificationFlow> {
        let req = self
            .post(&format!(
                "/api/v2/verification/flow/{flow_id}/submit_verification_code/"
            ))
            .json(request);
        self.send_json(req).await
    }

    /// Get verification categories.
    ///
    /// Returns a dictionary keyed by category name.
    ///
    /// `GET /api/v2/verification/categories/`
    pub async fn get_verification_categories(&self) -> Result<serde_json::Value> {
        let req = self.get("/api/v2/verification/categories/");
        self.send_json(req).await
    }

    /// Get verification limits.
    ///
    /// `GET /api/v2/verification/limits/`
    pub async fn get_verification_limits(&self) -> Result<VerificationLimits> {
        let req = self.get("/api/v2/verification/limits/");
        self.send_json(req).await
    }

    /// Get verification thresholds.
    ///
    /// Returns a dictionary keyed by category name.
    ///
    /// `GET /api/v2/verification/thresholds/`
    pub async fn get_verification_thresholds(&self) -> Result<serde_json::Value> {
        let req = self.get("/api/v2/verification/thresholds/");
        self.send_json(req).await
    }

    /// Get account suspension information.
    ///
    /// Returns the suspension reason even if the account is suspended.
    ///
    /// `GET /api/v2/verification/suspension/`
    pub async fn get_suspension(&self) -> Result<Suspension> {
        let req = self.get("/api/v2/verification/suspension/");
        self.send_json(req).await
    }

    /// List verification questions (paginated).
    ///
    /// `GET /api/v2/verification/question/`
    pub async fn list_verification_questions(
        &self,
        params: &ListQuestionsParams,
    ) -> Result<PaginatedResponse<VerificationQuestion>> {
        let req = self.get("/api/v2/verification/question/").query(params);
        self.send_json(req).await
    }

    /// Submit an answer to a verification question (multipart form).
    ///
    /// `POST /api/v2/verification/question/{question_id}/answer/`
    pub async fn submit_answer(
        &self,
        question_id: i64,
        answer: &str,
        files: Option<Vec<(String, Vec<u8>)>>,
    ) -> Result<VerificationAnswer> {
        let mut form = hpx::multipart::Form::new().text("answer", answer.to_owned());
        if let Some(file_list) = files {
            for (filename, data) in file_list {
                let part = hpx::multipart::Part::bytes(data).file_name(filename);
                form = form.part("files", part);
            }
        }
        let req = self
            .post(&format!(
                "/api/v2/verification/question/{question_id}/answer/"
            ))
            .multipart(form);
        self.send_json(req).await
    }

    /// List abuse reports (paginated).
    ///
    /// `GET /api/v2/verification/abuse_report/`
    pub async fn list_abuse_reports(
        &self,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<AbuseReport>> {
        let mut req = self.get("/api/v2/verification/abuse_report/");
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// List appeals (paginated).
    ///
    /// `GET /api/v2/verification/appeal/`
    pub async fn list_appeals(
        &self,
        state: Option<&str>,
        page: Option<u32>,
    ) -> Result<PaginatedResponse<Appeal>> {
        let mut req = self.get("/api/v2/verification/appeal/");
        if let Some(s) = state {
            req = req.query(&[("state", s)]);
        }
        if let Some(p) = page {
            req = req.query(&[("page", p.to_string())]);
        }
        self.send_json(req).await
    }

    /// Submit an appeal.
    ///
    /// `POST /api/v2/verification/appeal/`
    pub async fn submit_appeal(&self, request: &SubmitAppealRequest) -> Result<Appeal> {
        let req = self.post("/api/v2/verification/appeal/").json(request);
        self.send_json(req).await
    }
}
