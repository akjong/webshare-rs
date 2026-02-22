# Design Document: Webshare API SDK Rewrite

| Metadata | Details |
| :--- | :--- |
| **Author** | pb-plan agent |
| **Status** | Draft |
| **Created** | 2026-02-22 |
| **Reviewers** | akagi201 |
| **Related Issues** | N/A |

## 1. Executive Summary

**Problem:** The current `webshare-rs` SDK only implements 1 of ~108 Webshare API endpoints (proxy list), uses the forbidden `reqwest` HTTP client, has outdated dependency versions, and follows a suboptimal Rust SDK design pattern (traits-on-client, manual request building, no proper error types).

**Solution:** Complete rewrite of the SDK from scratch using `hpx` (with `rustls` TLS backend) as the HTTP client, implementing all ~108 Webshare API endpoints across 18 API groups. The rewrite follows idiomatic Rust SDK design: builder-pattern client, typed request/response models, proper error hierarchy with `thiserror`, modular API organization, and pagination support.

---

## 2. Requirements & Goals

### 2.1 Problem Statement

- Only 1 endpoint (proxy list) is currently implemented out of ~108 total endpoints.
- Uses `reqwest` which is forbidden per project guidelines — must use `hpx` with `rustls`.
- Dependency versions are outdated (e.g., `serde 1.0.216`, `reqwest 0.12.9`).
- No proper error handling — uses `eyre::Result` at library level (should use `thiserror`).
- No pagination abstractions — consumers must manually handle paginated responses.
- No typed error responses from the API.
- `#[allow(non_snake_case)]` suggests rushed implementation.
- No builder pattern for complex query parameters.
- Trait-per-API-group pattern (`ProxyListApi`) is unusual for Rust SDKs; method-based approach on client is more idiomatic.

### 2.2 Functional Goals

1. **Complete API coverage:** Implement all ~108 endpoints across 18 API groups (API Keys, Proxy List, Proxy Config, Proxy Replacement, Proxy Stats, Subscription, Plans, Sub-Users, IP Authorization, Notifications, User Profile, Registration & Login, Two-Factor Auth, Account Verification, ID Verification, Billing & Payments, Referral & Affiliate, Downloads).
2. **Type-safe models:** Every request and response has a strongly-typed Rust struct with proper `serde` derive macros.
3. **Ergonomic client:** Builder-pattern `WebshareClient` with configurable base URL, timeout, and auth token.
4. **Pagination support:** Generic `PaginatedResponse<T>` and a `PaginatedStream` / iterator for automatic page traversal.
5. **hpx HTTP backend:** Use `hpx` with `rustls-tls` feature — zero `reqwest` dependency.
6. **Proper error hierarchy:** `thiserror`-based `WebshareError` enum with API error response parsing.
7. **Optional plan_id:** Many endpoints accept `plan_id` — provide ergonomic optional parameter passing.
8. **Download support:** Handle plain-text and CSV download endpoints that return non-JSON responses.
9. **Multipart upload support:** Verification evidence submission requires multipart form data.

### 2.3 Non-Functional Goals

- **Performance:** Connection pooling via `hpx::Client` reuse; zero unnecessary allocations; `Cow<'_, str>` for borrowed-or-owned strings where beneficial.
- **Reliability:** Comprehensive error handling; API error response deserialization; timeout configuration.
- **Security:** TLS via `rustls` only; no `native-tls`; token never logged.
- **Ergonomics:** Fluent builder APIs; sensible defaults; comprehensive doc comments on every public item.
- **Testability:** All API methods accept `&self` on the client; base URL configurable for mock server testing.

### 2.4 Out of Scope

- WebSocket or streaming connections (Webshare API is REST-only).
- Rate limiting / retry logic (can be added as middleware later).
- CLI tool or binary crate changes (this is SDK library only).
- Leptos frontend integration (separate concern).
- OAuth / social login flow orchestration (we expose the endpoints but don't manage browser redirects).
- `utoipa` / OpenAPI generation for the SDK itself (this is a client SDK, not a server).

### 2.5 Assumptions

- `hpx` API is compatible with `reqwest`-style builder pattern (confirmed via docs: `.get()`, `.post()`, `.json()`, `.query()`, `.send()`, `.error_for_status()`, `.json().await`).
- `hpx` version `2.1.0` with features `rustls-tls`, `json`, `multipart`, `http1`, `http2` provides all needed functionality.
- Webshare API base URL remains `https://proxy.webshare.io`.
- Auth is always `Authorization: Token <TOKEN>` header (except unauthenticated endpoints).
- API versioning (v2/v3) is stable — we hardcode paths per endpoint.
- Multipart file upload for verification evidence can use `hpx::multipart::Form`.

---

## 3. Architecture Overview

### 3.1 System Context

```text
┌─────────────────────────────────────────────────┐
│                  webshare-rs (SDK)               │
│                                                   │
│  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
│  │  Client   │  │  Models  │  │    Error       │  │
│  │ (hpx)    │  │ (serde)  │  │  (thiserror)   │  │
│  └────┬─────┘  └──────────┘  └───────────────┘  │
│       │                                           │
│  ┌────┴──────────────────────────────────────┐   │
│  │              API Modules                   │   │
│  │  api_keys | proxy_list | proxy_config     │   │
│  │  proxy_replacement | proxy_stats          │   │
│  │  subscription | plans | sub_users         │   │
│  │  ip_authorization | notifications         │   │
│  │  user_profile | auth | two_factor_auth    │   │
│  │  verification | id_verification           │   │
│  │  billing | referral | downloads           │   │
│  └───────────────────────────────────────────┘   │
└─────────────────────────────────────────────────┘
                      │
                      ▼
          ┌──────────────────────┐
          │  Webshare REST API   │
          │  proxy.webshare.io   │
          └──────────────────────┘
```

The SDK is a Rust library crate (`src/`) that consumers add as a dependency. It exposes:

- `WebshareClient` — the main entry point, holds `hpx::Client` and auth config.
- Per-domain API methods directly on `WebshareClient` (e.g., `client.list_api_keys()`, `client.get_proxy_list()`).
- Typed request/response models in `src/models/`.
- Error types in `src/error.rs`.

### 3.2 Key Design Principles

1. **Method-on-client pattern:** All API methods are `impl WebshareClient` methods organized via `mod` files, not separate traits. This is more idiomatic for Rust HTTP SDKs (similar to `octocrab`, `aws-sdk-*`, `stripe-rust`).
2. **One model file per API group:** Each API group (e.g., `api_keys`, `proxy_list`) has its own model file with request/response structs.
3. **Builder pattern for complex requests:** Endpoints with many optional parameters use builder structs (e.g., `ListProxiesRequest`).
4. **Generic pagination:** `PaginatedResponse<T>` handles all paginated list endpoints uniformly.
5. **Error as enum:** `WebshareError` covers transport errors, deserialization errors, and API-level errors with status codes.
6. **`plan_id` as first-class optional:** Endpoints accepting `plan_id` include it as `Option<i64>` in their request types.

### 3.3 Existing Components to Reuse

| Component | Location | How to Reuse |
| :--- | :--- | :--- |
| `common` crate | `crates/common/` | Place shared error types or utility traits here if needed cross-crate; currently empty — we may keep SDK self-contained in `src/` |
| `Justfile` recipes | `Justfile` | Use existing `format`, `lint`, `test` recipes for verification |
| `Cargo.toml` workspace | Root `Cargo.toml` | SDK library is the root package — update dependencies in place |

> Note: The existing `src/rest.rs`, `src/apis/`, and `src/models/` code will be **entirely replaced**, not extended. The current implementation patterns (trait-per-group, `reqwest` usage, `eyre` at lib level) are incompatible with the new design.

---

## 4. Detailed Design

### 4.1 Module Structure

```text
src/
├── lib.rs                    # Public API re-exports, crate docs
├── client.rs                 # WebshareClient struct + builder
├── error.rs                  # WebshareError enum (thiserror)
├── pagination.rs             # PaginatedResponse<T>, pagination helpers
├── models/
│   ├── mod.rs                # Re-exports all model modules
│   ├── api_key.rs            # ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest
│   ├── proxy.rs              # Proxy, ListProxiesParams, ProxyDownloadParams
│   ├── proxy_config.rs       # ProxyConfig, UpdateProxyConfigRequest, ProxyStats, ProxyStatus, AllocateCountriesRequest
│   ├── proxy_replacement.rs  # ProxyReplacement, CreateReplacementRequest, ReplacedProxy
│   ├── proxy_stats.rs        # ProxyStat, AggregateStats, ProxyActivity
│   ├── subscription.rs       # Subscription, AvailableAssets, CustomizeQuery, PricingQuery, PurchaseRequest, RenewRequest
│   ├── plan.rs               # Plan, UpdatePlanRequest, UpgradePlanRequest, CancelPlanResponse
│   ├── sub_user.rs           # SubUser, CreateSubUserRequest, UpdateSubUserRequest
│   ├── ip_authorization.rs   # IpAuthorization, CreateIpAuthRequest, WhatsMyIpResponse
│   ├── notification.rs       # Notification
│   ├── user_profile.rs       # UserProfile, UpdateProfileRequest, UserPreferences, UpdatePreferencesRequest
│   ├── auth.rs               # RegisterRequest, LoginRequest, SocialAuthRequest, TokenResponse, ActivationStatus, etc.
│   ├── two_factor_auth.rs    # TwoFactorMethod, Enter2faRequest, Change2faRequest, Activate2faRequest
│   ├── verification.rs       # VerificationFlow, VerificationQuestion, VerificationAnswer, VerificationCategory, etc.
│   ├── id_verification.rs    # IdVerification
│   ├── billing.rs            # BillingInfo, UpdateBillingInfoRequest, PaymentMethod, Transaction, PendingPayment
│   ├── referral.rs           # ReferralConfig, UpdateReferralConfigRequest, ReferralCredit, ReferralEarnOut, ReferralCodeInfo
│   └── download_token.rs     # DownloadToken, DownloadTokenScope
├── api/
│   ├── mod.rs                # Re-exports all API modules  
│   ├── api_keys.rs           # impl WebshareClient { create/list/get/update/delete api_key }
│   ├── proxy_list.rs         # impl WebshareClient { list/download/refresh proxies }
│   ├── proxy_config.rs       # impl WebshareClient { get_config/get_stats/get_status/update_config/allocate_countries }
│   ├── proxy_replacement.rs  # impl WebshareClient { create/list/get replacement, list/download replaced_proxies }
│   ├── proxy_stats.rs        # impl WebshareClient { list_stats/aggregate_stats/list_activities/download_activities }
│   ├── subscription.rs       # impl WebshareClient { get_subscription/available_assets/customize/pricing/purchase/renew/download_invoice }
│   ├── plans.rs              # impl WebshareClient { list/get/update/upgrade/cancel plan }
│   ├── sub_users.rs          # impl WebshareClient { list/get/create/update/delete/refresh sub_user, masquerade_header }
│   ├── ip_authorization.rs   # impl WebshareClient { create/list/get/delete ip_auth, whats_my_ip }
│   ├── notifications.rs      # impl WebshareClient { list/get/dismiss/restore notification }
│   ├── user_profile.rs       # impl WebshareClient { get/update profile, get/update preferences }
│   ├── auth.rs               # impl WebshareClient { register/login/social_register/social_login/change_password/reset_password/... }
│   ├── two_factor_auth.rs    # impl WebshareClient { enter_2fa_code/get_2fa_method/change_2fa_method/activate_2fa_method/resend_2fa_email }
│   ├── verification.rs       # impl WebshareClient { list/get flows, submit_evidence, submit_code, categories, limits, thresholds, suspension, questions, answers, abuse_reports, appeals }
│   ├── id_verification.rs    # impl WebshareClient { get/start/complete id_verification }
│   ├── billing.rs            # impl WebshareClient { get/update billing_info, list/get payment_methods, update_payment_method, list/get transactions, list/get pending_payments }
│   ├── referral.rs           # impl WebshareClient { get/update referral_config, list/get credits, list/get earnouts, get_referral_code_info }
│   └── downloads.rs          # impl WebshareClient { get/reset download_token }
```

### 4.2 Data Structures & Types

#### Core Client

```rust
// src/client.rs
pub struct WebshareClient {
    http: hpx::Client,
    base_url: String,
    token: Option<String>,
}

pub struct WebshareClientBuilder {
    base_url: String,
    token: Option<String>,
    timeout: Duration,
}

impl WebshareClientBuilder {
    pub fn new() -> Self { /* defaults */ }
    pub fn token(mut self, token: impl Into<String>) -> Self { /* ... */ }
    pub fn base_url(mut self, url: impl Into<String>) -> Self { /* ... */ }
    pub fn timeout(mut self, timeout: Duration) -> Self { /* ... */ }
    pub fn build(self) -> Result<WebshareClient, WebshareError> { /* ... */ }
}

impl WebshareClient {
    pub fn new(token: impl Into<String>) -> Result<Self, WebshareError> { /* ... */ }
    pub fn builder() -> WebshareClientBuilder { /* ... */ }

    // Internal helpers
    fn get(&self, path: &str) -> hpx::RequestBuilder { /* ... */ }
    fn post(&self, path: &str) -> hpx::RequestBuilder { /* ... */ }
    fn patch(&self, path: &str) -> hpx::RequestBuilder { /* ... */ }
    fn delete(&self, path: &str) -> hpx::RequestBuilder { /* ... */ }
    fn request(&self, method: hpx::Method, path: &str) -> hpx::RequestBuilder { /* ... */ }
}
```

#### Error Type

```rust
// src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum WebshareError {
    #[error("HTTP transport error: {0}")]
    Transport(#[from] hpx::Error),

    #[error("JSON serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("API error (HTTP {status}): {message}")]
    Api {
        status: u16,
        message: String,
        details: Option<serde_json::Value>,
    },

    #[error("Client configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, WebshareError>;
```

#### Pagination

```rust
// src/pagination.rs
#[derive(Debug, Deserialize, Serialize)]
pub struct PaginatedResponse<T> {
    pub count: u64,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}
```

#### Example Model (API Keys)

```rust
// src/models/api_key.rs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub label: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
pub struct CreateApiKeyRequest {
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct UpdateApiKeyRequest {
    pub label: String,
}
```

### 4.3 Interface Design

#### API Method Signature Patterns

```rust
// Pattern 1: Simple GET (no params)
pub async fn get_subscription(&self) -> Result<Subscription>

// Pattern 2: GET with path param
pub async fn get_api_key(&self, id: i64) -> Result<ApiKey>

// Pattern 3: Paginated GET
pub async fn list_api_keys(&self, page: Option<u32>, page_size: Option<u32>) -> Result<PaginatedResponse<ApiKey>>

// Pattern 4: GET with query params struct
pub async fn list_proxies(&self, params: &ListProxiesParams) -> Result<PaginatedResponse<Proxy>>

// Pattern 5: POST with JSON body
pub async fn create_api_key(&self, request: &CreateApiKeyRequest) -> Result<ApiKey>

// Pattern 6: PATCH with JSON body
pub async fn update_api_key(&self, id: i64, request: &UpdateApiKeyRequest) -> Result<ApiKey>

// Pattern 7: DELETE (returns unit)
pub async fn delete_api_key(&self, id: &str) -> Result<()>

// Pattern 8: POST returning 204 (no content)
pub async fn refresh_proxy_list(&self, plan_id: Option<i64>) -> Result<()>

// Pattern 9: Download (returns bytes/string)
pub async fn download_proxy_list(&self, params: &ProxyDownloadParams) -> Result<String>

// Pattern 10: Unauthenticated endpoint
pub async fn get_referral_code_info(referral_code: &str) -> Result<ReferralCodeInfo>  // static method
```

### 4.4 Logic Flow

#### Request Flow

```text
User calls client.list_api_keys(page, page_size)
  → Build URL: "{base_url}/api/v2/apikey/"
  → Add query params (page, page_size) if Some
  → Add Authorization header: "Token {token}"
  → Send GET request via hpx
  → Check response status:
      - 2xx → Deserialize JSON body into PaginatedResponse<ApiKey>
      - 4xx/5xx → Parse error response body → return WebshareError::Api
      - Transport error → return WebshareError::Transport
  → Return Ok(result)
```

#### Internal Request Helper

```rust
impl WebshareClient {
    async fn send_json<T: DeserializeOwned>(&self, request: hpx::RequestBuilder) -> Result<T> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();
        if status.is_success() {
            response.json::<T>().await.map_err(WebshareError::Transport)
        } else {
            let body = response.text().await.unwrap_or_default();
            let details = serde_json::from_str(&body).ok();
            Err(WebshareError::Api {
                status: status.as_u16(),
                message: body,
                details,
            })
        }
    }

    async fn send_no_content(&self, request: hpx::RequestBuilder) -> Result<()> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();
        if status.is_success() {
            Ok(())
        } else {
            // ... same error handling
        }
    }

    async fn send_raw(&self, request: hpx::RequestBuilder) -> Result<Vec<u8>> {
        let response = request.send().await.map_err(WebshareError::Transport)?;
        let status = response.status();
        if status.is_success() {
            response.bytes().await.map(|b| b.to_vec()).map_err(WebshareError::Transport)
        } else {
            // ... same error handling
        }
    }
}
```

### 4.5 Configuration

```toml
# Cargo.toml dependencies (all latest versions)
[dependencies]
hpx = { version = "2.1", default-features = false, features = ["rustls-tls", "json", "multipart", "http1", "http2"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
url = "2"
```

- **Base URL:** Default `https://proxy.webshare.io`, configurable via builder.
- **Timeout:** Default 30 seconds, configurable via builder.
- **Auth Token:** Set via `WebshareClient::new(token)` or builder. `None` for unauthenticated-only usage.

### 4.6 Error Handling

All API methods return `Result<T, WebshareError>` where:

- `WebshareError::Transport` — network issues, DNS failures, timeouts, TLS errors (wraps `hpx::Error`).
- `WebshareError::Api` — Webshare API returned a non-2xx status. Includes HTTP status code, raw message, and parsed JSON details if available.
- `WebshareError::Serialization` — Failed to serialize request or deserialize response (wraps `serde_json::Error`).
- `WebshareError::Config` — Client misconfiguration (e.g., invalid base URL).

For 204 No Content responses, the API methods return `Result<()>`.
For download endpoints, the API methods return `Result<String>` (text) or `Result<Vec<u8>>` (binary/PDF).

---

## 5. Verification & Testing Strategy

### 5.1 Unit Testing

- Test model serialization/deserialization with known JSON fixtures for each API group.
- Test error type construction and `Display` formatting.
- Test URL construction for each endpoint path.
- Test query parameter serialization (especially `skip_serializing_if` for None fields).

### 5.2 Integration Testing

- Use a configurable `base_url` to point at a mock HTTP server (via `httpmock` or similar) in `tests/`.
- Each API group gets an integration test file that verifies:
  - Correct HTTP method is used.
  - Correct URL path is constructed.
  - Auth header is sent.
  - Request body matches expected JSON.
  - Response is deserialized correctly.
  - Error responses produce `WebshareError::Api`.

### 5.3 Critical Path Verification (The "Harness")

| Verification Step | Command | Success Criteria |
| :--- | :--- | :--- |
| **VP-01** | `cargo check --all-targets --all-features` | Compiles with zero errors |
| **VP-02** | `cargo +nightly clippy --all -- -D warnings` | Zero warnings |
| **VP-03** | `cargo +nightly fmt --all -- --check` | Properly formatted |
| **VP-04** | `cargo test --all-features` | All tests pass |
| **VP-05** | `cargo machete` | No unused dependencies |
| **VP-06** | `cargo doc --no-deps` | Documentation builds without warnings |

### 5.4 Validation Rules

| Test Case ID | Action | Expected Outcome | Verification Method |
| :--- | :--- | :--- | :--- |
| **TC-01** | Create `WebshareClient::new("test-token")` | Client created with default base URL and token | Unit test |
| **TC-02** | Serialize `ListProxiesParams { mode: "direct" }` to query string | `mode=direct` with no None fields | Unit test |
| **TC-03** | Deserialize paginated API key response JSON | `PaginatedResponse<ApiKey>` with correct count and results | Unit test |
| **TC-04** | Call `list_api_keys()` against mock server | GET to `/api/v2/apikey/` with `Authorization: Token ...` header | Integration test |
| **TC-05** | API returns 401 | `WebshareError::Api { status: 401, .. }` | Integration test |
| **TC-06** | API returns invalid JSON | `WebshareError::Transport` (deserialization failure) | Integration test |
| **TC-07** | All 18 API groups have at least one working endpoint test | Full coverage of API surface | Integration test suite |

---

## 6. Implementation Plan

- [ ] **Phase 1: Foundation** — Client, error types, pagination, Cargo.toml setup
- [ ] **Phase 2: Core Logic** — Implement all 18 API group models and endpoint methods
- [ ] **Phase 3: Integration** — Wire `lib.rs` re-exports, clean up module structure, remove old code
- [ ] **Phase 4: Polish** — Unit tests, doc comments, clippy/fmt, CI verification

---

## 7. Cross-Functional Concerns

### Backward Compatibility

**None.** Per requirements, this is a clean rewrite with no backward compatibility concerns. The existing `rest.rs`, `apis/`, and `models/` code is entirely replaced. Consumers must update their code to use the new API.

### Security

- Auth tokens are stored in the client struct but never included in `Debug` output (we'll use a custom `Debug` impl or wrap in a newtype).
- TLS is enforced via `rustls` — no `native-tls` fallback.
- `https_only(true)` can be set on the hpx client builder to prevent accidental HTTP usage.

### Migration

Old code in `src/rest.rs`, `src/apis/`, `src/models/` is deleted. The `bin/leptos-csr-app/` and `crates/common/` code is not affected (the front-end app may need updates if it imports from the SDK, but that's out of scope for this task).

### Documentation

Every public type, method, and field will have `///` doc comments. Module-level docs describe the API group. The crate-level doc comment in `lib.rs` provides a quick-start example.
