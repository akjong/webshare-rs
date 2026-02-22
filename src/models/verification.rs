//! Account verification model types.

use serde::{Deserialize, Serialize};

/// A file attachment in evidence/answers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationFile {
    pub id: i64,
    pub file: String,
    pub created_at: String,
}

/// Evidence submitted for a verification flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: i64,
    pub explanation: String,
    pub created_at: String,
    pub updated_at: String,
    pub files: Vec<VerificationFile>,
}

/// A verification flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationFlow {
    pub id: i64,
    /// One of `acceptable_use_violation`, `abuse_report`, `fraudulent_payment`.
    #[serde(rename = "type")]
    pub type_: String,
    /// One of `inflow`, `successful_verification`, `failed_verification`.
    pub state: String,
    pub started_at: String,
    pub updated_at: String,
    pub needs_evidence: bool,
    pub evidence: Option<Evidence>,
    pub id_verification_restores_access: bool,
    pub id_verification_required: bool,
}

/// An answer to a verification question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAnswer {
    pub id: i64,
    pub answer: String,
    pub created_at: String,
    pub updated_at: String,
    pub files: Vec<VerificationFile>,
}

/// A verification question.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationQuestion {
    pub id: i64,
    pub question: String,
    pub created_at: String,
    pub updated_at: String,
    pub flow: i64,
    pub answer: Option<VerificationAnswer>,
}

/// Query parameters for listing verification questions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListQuestionsParams {
    #[serde(rename = "flow__type", skip_serializing_if = "Option::is_none")]
    pub flow_type: Option<String>,
    #[serde(rename = "flow__state", skip_serializing_if = "Option::is_none")]
    pub flow_state: Option<String>,
    #[serde(rename = "answer__isnull", skip_serializing_if = "Option::is_none")]
    pub answer_isnull: Option<bool>,
    #[serde(
        rename = "flow__started_at__gte",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_started_at_gte: Option<String>,
    #[serde(
        rename = "flow__started_at__lte",
        skip_serializing_if = "Option::is_none"
    )]
    pub flow_started_at_lte: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(rename = "answer__answer", skip_serializing_if = "Option::is_none")]
    pub answer_answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<u32>,
}

/// Verification limits response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationLimits {
    /// One of `active`, `limited`, `paused`.
    pub proxy_state: String,
}

/// Account suspension information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suspension {
    pub created_at: String,
    /// One of `acceptable_use_violation`, `abuse_report`, `fraudulent_payment`.
    pub reason: String,
}

/// An abuse report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbuseReport {
    pub id: i64,
    pub content: String,
    pub flow: i64,
    pub created_at: String,
    pub updated_at: String,
}

/// An appeal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appeal {
    pub id: i64,
    pub appeal: String,
    /// One of `submitted`, `approved`, `rejected`.
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
}

/// Request body for submitting an appeal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAppealRequest {
    pub appeal: String,
}

/// Request body for submitting a security code.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitSecurityCodeRequest {
    pub security_code: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_verification_flow() {
        let json = r#"{
            "id": 1,
            "type": "acceptable_use_violation",
            "state": "inflow",
            "started_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z",
            "needs_evidence": true,
            "evidence": null,
            "id_verification_restores_access": false,
            "id_verification_required": false
        }"#;
        let flow: VerificationFlow = serde_json::from_str(json).unwrap();
        assert_eq!(flow.type_, "acceptable_use_violation");
        assert!(flow.needs_evidence);
    }

    #[test]
    fn deserialize_appeal() {
        let json = r#"{
            "id": 1,
            "appeal": "I did nothing wrong",
            "state": "submitted",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }"#;
        let appeal: Appeal = serde_json::from_str(json).unwrap();
        assert_eq!(appeal.state, "submitted");
    }
}
