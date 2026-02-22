//! Integration tests for the Webshare API.
//!
//! These tests make real HTTP requests against the Webshare API.
//! The API key is read from the `WEBSHARE_API_KEY` environment variable.
//!
//! Run with:
//!   WEBSHARE_API_KEY=<your-key> cargo test --test integration
//!
//! Tests are grouped by safety:
//! - Read-only tests (list/get) run unconditionally.
//! - Mutating tests (create/update/delete) are gated behind `#[ignore]` so they
//!   don't accidentally modify account state. Run them with:
//!     WEBSHARE_API_KEY=<key> cargo test --test integration -- --ignored

use std::env;

use webshare_rs::{
    WebshareClient,
    models::{
        api_key::{CreateApiKeyRequest, UpdateApiKeyRequest},
        billing::UpdateBillingInfoRequest,
        download::DownloadTokenScope,
        notification::ListNotificationsParams,
        proxy::ListProxiesParams,
        proxy_stats::{ListActivitiesParams, ListStatsParams},
        verification::ListQuestionsParams,
    },
};

/// Build an authenticated client from the environment variable.
fn client() -> WebshareClient {
    let token = env::var("WEBSHARE_API_KEY")
        .expect("WEBSHARE_API_KEY environment variable must be set to run integration tests");
    WebshareClient::new(&token)
}

// ═══════════════════════════════════════════════════════════════════════════
// User Profile
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_profile() {
    let c = client();
    let profile = c.get_profile().await.expect("get_profile failed");
    assert!(
        !profile.email.is_empty(),
        "profile email should be non-empty"
    );
    assert!(profile.id > 0, "profile id should be positive");
}

#[tokio::test]
async fn test_get_preferences() {
    let c = client();
    let _prefs = c.get_preferences().await.expect("get_preferences failed");
}

// ═══════════════════════════════════════════════════════════════════════════
// API Keys
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_api_keys() {
    let c = client();
    // API keys cannot query the API key list endpoint, so expect 403.
    let result = c.list_api_keys(None, None).await;
    assert!(
        result.is_err(),
        "list_api_keys should fail with API key auth (403)"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// API Key CRUD (mutating — ignored by default)
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
#[ignore]
async fn test_api_key_crud() {
    let c = client();

    // Create
    let created = c
        .create_api_key(&CreateApiKeyRequest {
            label: "integration-test-key".into(),
        })
        .await
        .expect("create_api_key failed");
    assert_eq!(created.label, "integration-test-key");
    let key_id = created.id;

    // Get
    let fetched = c.get_api_key(key_id).await.expect("get_api_key failed");
    assert_eq!(fetched.id, key_id);
    assert_eq!(fetched.label, "integration-test-key");

    // Update
    let updated = c
        .update_api_key(
            key_id,
            &UpdateApiKeyRequest {
                label: "renamed-test-key".into(),
            },
        )
        .await
        .expect("update_api_key failed");
    assert_eq!(updated.label, "renamed-test-key");

    // Delete
    c.delete_api_key(key_id)
        .await
        .expect("delete_api_key failed");
}

// ═══════════════════════════════════════════════════════════════════════════
// Proxy List
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_proxies() {
    let c = client();
    let params = ListProxiesParams {
        mode: "direct".into(),
        page: Some(1),
        page_size: Some(5),
        ..Default::default()
    };
    let page = c.list_proxies(&params).await.expect("list_proxies failed");
    // Free plan may have 0 proxies, that's okay — we just verify the call succeeds.
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Proxy Configuration
// ═══════════════════════════════════════════════════════════════════════════

// Note: proxy_config endpoints require a plan_id. We fetch it from plans list.
async fn get_first_plan_id(c: &WebshareClient) -> i64 {
    let plans = c
        .list_plans(Some(1), Some(1))
        .await
        .expect("list_plans failed");
    assert!(!plans.results.is_empty(), "no plans found");
    plans.results[0].id
}

#[tokio::test]
async fn test_get_proxy_config() {
    let c = client();
    let plan_id = get_first_plan_id(&c).await;
    let config = c
        .get_proxy_config(plan_id)
        .await
        .expect("get_proxy_config failed");
    // Just verify it deserialized; check a field exists.
    let _ = &config;
}

#[tokio::test]
async fn test_get_proxy_list_stats() {
    let c = client();
    let plan_id = get_first_plan_id(&c).await;
    let _stats = c
        .get_proxy_list_stats(plan_id)
        .await
        .expect("get_proxy_list_stats failed");
}

#[tokio::test]
async fn test_get_proxy_list_status() {
    let c = client();
    let plan_id = get_first_plan_id(&c).await;
    let _status = c
        .get_proxy_list_status(plan_id)
        .await
        .expect("get_proxy_list_status failed");
}

// ═══════════════════════════════════════════════════════════════════════════
// Plans
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_plans() {
    let c = client();
    let page = c
        .list_plans(Some(1), Some(5))
        .await
        .expect("list_plans failed");
    assert!(page.count >= 1, "account should have at least 1 plan");
    let plan = &page.results[0];
    assert!(plan.id > 0);
}

#[tokio::test]
async fn test_get_plan() {
    let c = client();
    let plan_id = get_first_plan_id(&c).await;
    let plan = c.get_plan(plan_id).await.expect("get_plan failed");
    assert_eq!(plan.id, plan_id);
}

// ═══════════════════════════════════════════════════════════════════════════
// IP Authorization
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_whats_my_ip() {
    let c = client();
    let resp = c.whats_my_ip().await.expect("whats_my_ip failed");
    assert!(!resp.ip_address.is_empty(), "IP should not be empty");
}

#[tokio::test]
async fn test_list_ip_authorizations() {
    let c = client();
    let page = c
        .list_ip_authorizations(None, None)
        .await
        .expect("list_ip_authorizations failed");
    // Count can be 0 — just verify the call succeeds.
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Notifications
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_notifications() {
    let c = client();
    let page = c
        .list_notifications(&ListNotificationsParams::default())
        .await
        .expect("list_notifications failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Billing
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_billing_info() {
    let c = client();
    let info = c.get_billing_info().await.expect("get_billing_info failed");
    // BillingInfo always exists; id must be positive.
    assert!(info.id > 0);
}

#[tokio::test]
#[ignore]
async fn test_update_billing_info() {
    let c = client();
    let updated = c
        .update_billing_info(&UpdateBillingInfoRequest {
            name: Some("Integration Test".into()),
            ..Default::default()
        })
        .await
        .expect("update_billing_info failed");
    assert_eq!(updated.name, "Integration Test");

    // Revert
    c.update_billing_info(&UpdateBillingInfoRequest {
        name: Some(String::new()),
        ..Default::default()
    })
    .await
    .expect("revert billing_info failed");
}

#[tokio::test]
async fn test_list_payment_methods() {
    let c = client();
    let page = c
        .list_payment_methods(None)
        .await
        .expect("list_payment_methods failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_transactions() {
    let c = client();
    let page = c
        .list_transactions(None)
        .await
        .expect("list_transactions failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_pending_payments() {
    let c = client();
    let page = c
        .list_pending_payments(None)
        .await
        .expect("list_pending_payments failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Proxy Stats
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_stats() {
    let c = client();
    let stats = c
        .list_stats(&ListStatsParams::default())
        .await
        .expect("list_stats failed");
    // stats is a Vec, can be empty.
    let _ = stats;
}

#[tokio::test]
async fn test_aggregate_stats() {
    let c = client();
    let agg = c
        .aggregate_stats(&ListStatsParams::default())
        .await
        .expect("aggregate_stats failed");
    // Just verify it deserialized successfully.
    let _ = agg;
}

#[tokio::test]
async fn test_list_activities() {
    let c = client();
    let page = c
        .list_activities(&ListActivitiesParams::default())
        .await
        .expect("list_activities failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Proxy Replacement
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_replacements() {
    let c = client();
    let page = c
        .list_replacements(
            &webshare_rs::models::proxy_replacement::ListReplacementsParams::default(),
        )
        .await
        .expect("list_replacements failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_replaced_proxies() {
    let c = client();
    let page = c
        .list_replaced_proxies(
            &webshare_rs::models::proxy_replacement::ListReplacedProxiesParams::default(),
        )
        .await
        .expect("list_replaced_proxies failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Subscription
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_available_assets() {
    let c = client();
    let assets = c
        .get_available_assets()
        .await
        .expect("get_available_assets failed");
    // AvailableAssets is serde_json::Value, should be non-null.
    assert!(!assets.is_null());
}

// ═══════════════════════════════════════════════════════════════════════════
// Sub-Users
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_sub_users() {
    let c = client();
    let page = c
        .list_sub_users(None, None)
        .await
        .expect("list_sub_users failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Referral
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_referral_config() {
    let c = client();
    let config = c
        .get_referral_config()
        .await
        .expect("get_referral_config failed");
    assert!(!config.referral_code.is_empty());
    assert!(config.id > 0);
}

#[tokio::test]
async fn test_list_referral_credits() {
    let c = client();
    let page = c
        .list_referral_credits(None)
        .await
        .expect("list_referral_credits failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_referral_earnouts() {
    let c = client();
    let page = c
        .list_referral_earnouts(None)
        .await
        .expect("list_referral_earnouts failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Two-Factor Auth
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_2fa_method() {
    let c = client();
    // API keys cannot query 2FA endpoint, so expect 403.
    let result = c.get_2fa_method().await;
    assert!(
        result.is_err(),
        "get_2fa_method should fail with API key auth (403)"
    );
}

// ═══════════════════════════════════════════════════════════════════════════
// ID Verification
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_id_verification() {
    let c = client();
    let verification = c
        .get_id_verification()
        .await
        .expect("get_id_verification failed");
    assert!(!verification.state.is_empty());
}

// ═══════════════════════════════════════════════════════════════════════════
// Account Verification
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_list_verification_flows() {
    let c = client();
    let page = c
        .list_verification_flows(None)
        .await
        .expect("list_verification_flows failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_get_verification_categories() {
    let c = client();
    let cats = c
        .get_verification_categories()
        .await
        .expect("get_verification_categories failed");
    // Should be a JSON object or array.
    assert!(!cats.is_null());
}

#[tokio::test]
async fn test_get_verification_limits() {
    let c = client();
    let limits = c
        .get_verification_limits()
        .await
        .expect("get_verification_limits failed");
    assert!(!limits.proxy_state.is_empty());
}

#[tokio::test]
async fn test_get_verification_thresholds() {
    let c = client();
    let thresholds = c
        .get_verification_thresholds()
        .await
        .expect("get_verification_thresholds failed");
    assert!(!thresholds.is_null());
}

#[tokio::test]
async fn test_list_verification_questions() {
    let c = client();
    let page = c
        .list_verification_questions(&ListQuestionsParams::default())
        .await
        .expect("list_verification_questions failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_abuse_reports() {
    let c = client();
    let page = c
        .list_abuse_reports(None)
        .await
        .expect("list_abuse_reports failed");
    let _ = page.count; // verified call succeeded
}

#[tokio::test]
async fn test_list_appeals() {
    let c = client();
    let page = c
        .list_appeals(None, None)
        .await
        .expect("list_appeals failed");
    let _ = page.count; // verified call succeeded
}

// ═══════════════════════════════════════════════════════════════════════════
// Downloads
// ═══════════════════════════════════════════════════════════════════════════

#[tokio::test]
async fn test_get_download_token() {
    let c = client();
    let token = c
        .get_download_token(DownloadTokenScope::ProxyList)
        .await
        .expect("get_download_token failed");
    assert!(!token.key.is_empty());
    assert_eq!(token.scope, DownloadTokenScope::ProxyList);
}
