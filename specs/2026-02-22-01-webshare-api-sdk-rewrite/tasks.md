# Webshare API SDK Rewrite — Implementation Tasks

| Metadata | Details |
| :--- | :--- |
| **Design Doc** | specs/2026-02-22-01-webshare-api-sdk-rewrite/design.md |
| **Owner** | akagi201 |
| **Start Date** | 2026-02-22 |
| **Target Date** | 2026-03-08 |
| **Status** | Planning |

## Summary & Phasing

Complete rewrite of the `webshare-rs` SDK implementing all ~108 Webshare API endpoints using `hpx` with `rustls`. The work is organized in 4 phases:

- **Phase 1: Foundation & Scaffolding** — Replace dependencies, build client, error types, and pagination primitives
- **Phase 2: Core Logic** — Implement all 18 API groups (models + endpoint methods)
- **Phase 3: Integration & Features** — Wire everything together, clean up old code, re-exports
- **Phase 4: Polish, QA & Docs** — Tests, documentation, linting, final verification

---

## Phase 1: Foundation & Scaffolding

### Task 1.1: Update Cargo.toml Dependencies

> **Context:** The root `Cargo.toml` currently depends on `reqwest` which is forbidden. We need to replace it with `hpx` and update all dependencies to latest versions. This is the foundation everything else builds on.
> **Verification:** `cargo check` compiles without errors after dependency change.

- **Priority:** P0
- **Scope:** Dependency management
- **Status:** � DONE

- [x] **Step 1:** Remove `reqwest`, `eyre`, `serde_derive`, `serde_with`, `url` from `[dependencies]`.
- [x] **Step 2:** Add `hpx = { version = "2.1", default-features = false, features = ["rustls-tls", "json", "multipart", "http1", "http2"] }`.
- [x] **Step 3:** Add/update: `serde = { version = "1", features = ["derive"] }`, `serde_json = "1"`, `thiserror = "2"`.
- [x] **Step 4:** Keep `url = "2"` if needed, or remove if not used.
- [x] **Step 5:** Verify `cargo check` passes (existing code will break — that's expected; we're replacing it in Task 1.3).
- [x] **Verification:** `cargo check` compiles with the new dependency set (may have errors in old src files — acceptable at this stage as they're being rewritten).

---

### Task 1.2: Implement Error Types (`src/error.rs`)

> **Context:** The SDK needs a proper error hierarchy using `thiserror` (not `eyre`, which is for app-level code). Every API method will return `Result<T, WebshareError>`. This must exist before any API implementation.
> **Verification:** Error types compile and derive correct traits (`Debug`, `Display`, `Error`).

- **Priority:** P0
- **Scope:** Error handling foundation
- **Status:** � DONE

- [x] **Step 1:** Create `src/error.rs` with `WebshareError` enum.
- [x] **Step 2:** Add `pub type Result<T> = std::result::Result<T, WebshareError>;`
- [x] **Step 3:** Implement `From<hpx::Error>` and `From<serde_json::Error>` via `#[from]`.
- [x] **Verification:** `cargo check` on `src/error.rs`; `WebshareError` implements `std::error::Error + Send + Sync`.

---

### Task 1.3: Implement Core Client (`src/client.rs`)

> **Context:** The `WebshareClient` is the central struct that all API methods are implemented on. It wraps `hpx::Client`, stores the base URL and auth token, and provides internal helpers for making requests. Replaces the old `src/rest.rs`.
> **Verification:** `WebshareClient::new("token")` and `WebshareClient::builder().token("x").build()` both produce valid clients.

- **Priority:** P0
- **Scope:** HTTP client core
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/client.rs` with `WebshareClient` struct (fields: `http: hpx::Client`, `base_url: String`, `token: Option<String>`).
- [ ] **Step 2:** Implement `WebshareClientBuilder` with methods: `new()`, `token()`, `base_url()`, `timeout()`, `build()`.
- [ ] **Step 3:** Implement `WebshareClient::new(token)` as shorthand constructor with defaults.
- [ ] **Step 4:** Implement internal request helpers:
  - `fn request(&self, method: hpx::Method, path: &str) -> hpx::RequestBuilder` — builds URL and adds auth header.
  - `fn get(&self, path: &str) -> hpx::RequestBuilder`
  - `fn post(&self, path: &str) -> hpx::RequestBuilder`
  - `fn patch(&self, path: &str) -> hpx::RequestBuilder`
  - `fn delete(&self, path: &str) -> hpx::RequestBuilder`
- [ ] **Step 5:** Implement response helpers:
  - `async fn send_json<T: DeserializeOwned>(&self, req: hpx::RequestBuilder) -> Result<T>`
  - `async fn send_no_content(&self, req: hpx::RequestBuilder) -> Result<()>`
  - `async fn send_text(&self, req: hpx::RequestBuilder) -> Result<String>`
  - `async fn send_bytes(&self, req: hpx::RequestBuilder) -> Result<Vec<u8>>`
- [ ] **Step 6:** Implement custom `Debug` for `WebshareClient` that redacts the token.
- [ ] **Verification:** Unit test: create client with builder, verify base_url and token are set. `Debug` output does not contain the token value.

---

### Task 1.4: Implement Pagination Types (`src/pagination.rs`)

> **Context:** Most list endpoints return paginated responses with `{count, next, previous, results}`. A generic type avoids duplication across all API groups.
> **Verification:** Can deserialize a sample paginated JSON response into `PaginatedResponse<serde_json::Value>`.

- **Priority:** P0
- **Scope:** Shared data types
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/pagination.rs` with `PaginatedResponse<T>` struct: `count: u64`, `next: Option<String>`, `previous: Option<String>`, `results: Vec<T>`.
- [ ] **Step 2:** Derive `Debug, Clone, Serialize, Deserialize` on `PaginatedResponse<T>`.
- [ ] **Step 3:** Add helper methods: `fn is_empty(&self) -> bool`, `fn has_next(&self) -> bool`.
- [ ] **Verification:** Unit test: deserialize `{"count":1,"next":null,"previous":null,"results":[{"id":1}]}` into `PaginatedResponse<serde_json::Value>`.

---

### Task 1.5: Set Up `lib.rs` Module Structure

> **Context:** Replace the old `lib.rs` (which declares `apis`, `models`, `rest`) with the new module structure. This wires up error, client, pagination, and placeholder `api`/`models` modules.
> **Verification:** `cargo check` passes with the new module structure.

- **Priority:** P0
- **Scope:** Module wiring
- **Status:** 🔴 TODO

- [ ] **Step 1:** Delete old `src/rest.rs`, `src/apis/mod.rs`, `src/apis/proxy_list_api.rs`, `src/models/mod.rs`, `src/models/proxy_list.rs`.
- [ ] **Step 2:** Rewrite `src/lib.rs`:
  ```rust
  //! Webshare API client for Rust
  //!
  //! # Example
  //! ```no_run
  //! use webshare_rs::WebshareClient;
  //!
  //! #[tokio::main]
  //! async fn main() -> webshare_rs::Result<()> {
  //!     let client = WebshareClient::new("your-api-token")?;
  //!     let keys = client.list_api_keys(None, None).await?;
  //!     println!("{:?}", keys);
  //!     Ok(())
  //! }
  //! ```
  
  pub mod client;
  pub mod error;
  pub mod pagination;
  pub mod models;
  pub mod api;
  
  pub use client::{WebshareClient, WebshareClientBuilder};
  pub use error::{WebshareError, Result};
  pub use pagination::PaginatedResponse;
  ```
- [ ] **Step 3:** Create empty `src/models/mod.rs` and `src/api/mod.rs` with placeholder comments.
- [ ] **Verification:** `cargo check` passes with no errors.

---

## Phase 2: Core Logic

### Task 2.1: API Keys — Models & Endpoints (5 endpoints)

> **Context:** API Keys are a simple CRUD resource — good first API group to implement as a pattern template. Endpoints: Create, List, Retrieve, Update, Delete.
> **Verification:** Compiles; model serialization round-trips correctly.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/api_key.rs` with:
  - `ApiKey` (response): `id: i64, key: String, label: String, created_at: String, updated_at: String`
  - `CreateApiKeyRequest`: `label: String`
  - `UpdateApiKeyRequest`: `label: String`
- [ ] **Step 2:** Create `src/api/api_keys.rs` with `impl WebshareClient`:
  - `create_api_key(&self, request: &CreateApiKeyRequest) -> Result<ApiKey>` — POST `/api/v2/apikey/`
  - `list_api_keys(&self, page: Option<u32>, page_size: Option<u32>) -> Result<PaginatedResponse<ApiKey>>` — GET `/api/v2/apikey/`
  - `get_api_key(&self, id: i64) -> Result<ApiKey>` — GET `/api/v2/apikey/{id}/`
  - `update_api_key(&self, id: i64, request: &UpdateApiKeyRequest) -> Result<ApiKey>` — PATCH `/api/v2/apikey/{id}/`
  - `delete_api_key(&self, id: &str) -> Result<()>` — DELETE `/api/v2/apikey/{id}/`
- [ ] **Step 3:** Register in `src/models/mod.rs` and `src/api/mod.rs`.
- [ ] **Verification:** `cargo check`; serde round-trip test for `ApiKey`.

---

### Task 2.2: Proxy List — Models & Endpoints (3 endpoints)

> **Context:** Core proxy listing functionality. The list endpoint has many optional query parameters that need a builder struct. Download endpoint is unauthenticated and returns plain text.
> **Verification:** Compiles; `ListProxiesParams` serializes only non-None fields.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/proxy.rs` with:
  - `Proxy` (response): `id: String, username: String, password: String, proxy_address: String, port: u32, valid: bool, last_verification: String, country_code: String, city_name: String, created_at: String`
  - `ListProxiesParams`: `mode: String` + many optional fields with `#[serde(skip_serializing_if = "Option::is_none")]`
  - `ProxyDownloadParams`: `token: String, country_codes: String, auth_method: String, endpoint_mode: String, search: String, plan_id: Option<i64>`
- [ ] **Step 2:** Create `src/api/proxy_list.rs` with `impl WebshareClient`:
  - `list_proxies(&self, params: &ListProxiesParams) -> Result<PaginatedResponse<Proxy>>` — GET `/api/v2/proxy/list/`
  - `download_proxy_list(&self, params: &ProxyDownloadParams) -> Result<String>` — GET download URL (unauthenticated)
  - `refresh_proxy_list(&self, plan_id: Option<i64>) -> Result<()>` — POST `/api/v2/proxy/list/refresh/`
- [ ] **Step 3:** Register in `src/models/mod.rs` and `src/api/mod.rs`.
- [ ] **Verification:** `cargo check`; serde test that None fields are omitted.

---

### Task 2.3: Proxy Configuration — Models & Endpoints (5 endpoints)

> **Context:** Proxy configuration management including v3 endpoints for stats and status. The `allocate_unallocated_countries` endpoint takes a dict (HashMap) as body.
> **Verification:** Compiles; all 5 endpoints have methods.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/proxy_config.rs` with:
  - `ProxyConfig` (response): all config fields (request_timeout, ip_authorization_country_codes, auto_replace flags, download_token, username, password, etc.)
  - `UpdateProxyConfigRequest`: writable fields as `Option<T>` with skip_serializing_if
  - `ProxyStats` (response): available_countries, ip_ranges, asns
  - `ProxyStatus` (response): state, countries, unallocated_countries, username, password, is_proxy_used
  - `AllocateCountriesRequest`: `new_countries: HashMap<String, u32>`
- [ ] **Step 2:** Create `src/api/proxy_config.rs` with `impl WebshareClient`:
  - `get_proxy_config(&self, plan_id: i64) -> Result<ProxyConfig>` — GET `/api/v3/proxy/config`
  - `get_proxy_stats(&self, plan_id: i64) -> Result<ProxyStats>` — GET `/api/v3/proxy/list/stats`
  - `get_proxy_status(&self, plan_id: i64) -> Result<ProxyStatus>` — GET `/api/v3/proxy/list/status`
  - `update_proxy_config(&self, request: &UpdateProxyConfigRequest, plan_id: Option<i64>) -> Result<ProxyConfig>` — PATCH `/api/v2/proxy/config/`
  - `allocate_unallocated_countries(&self, request: &AllocateCountriesRequest, plan_id: Option<i64>) -> Result<ProxyConfig>` — POST `/api/v2/proxy/config/allocate_unallocated_countries/`
- [ ] **Step 3:** Register in `src/models/mod.rs` and `src/api/mod.rs`.
- [ ] **Verification:** `cargo check`.

---

### Task 2.4: Proxy Replacement — Models & Endpoints (5 endpoints)

> **Context:** Proxy replacement operations and history. Replacement create accepts complex nested body (`to_replace`, `replace_with` dicts). Replaced proxy download is unauthenticated.
> **Verification:** Compiles; `CreateReplacementRequest` serializes nested structures correctly.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/proxy_replacement.rs` with:
  - `ProxyReplacement` (response): id, to_replace, replace_with, dry_run, state, proxies_removed, proxies_added, reason, error, error_code, timestamps
  - `CreateReplacementRequest`: to_replace (serde_json::Value or typed), replace_with (Vec), dry_run (bool)
  - `ListReplacementsParams`: ordering, dry_run, state, plan_id
  - `ReplacedProxy` (response): id, reason, proxy, proxy_port, proxy_country_code, replaced_with, replaced_with_port, replaced_with_country_code, created_at
  - `ListReplacedProxiesParams`: proxy_list_replacement (Option<i64>)
  - `DownloadReplacedProxiesParams`: download_token, country_codes, authentication_type, mode, search, proxy_list_replacement
- [ ] **Step 2:** Create `src/api/proxy_replacement.rs` with `impl WebshareClient`:
  - `create_replacement(&self, request: &CreateReplacementRequest, plan_id: Option<i64>) -> Result<ProxyReplacement>` — POST `/api/v3/proxy/replace/`
  - `list_replacements(&self, params: &ListReplacementsParams) -> Result<PaginatedResponse<ProxyReplacement>>` — GET `/api/v3/proxy/replace/`
  - `get_replacement(&self, id: i64, plan_id: Option<i64>) -> Result<ProxyReplacement>` — GET `/api/v3/proxy/replace/{id}/`
  - `list_replaced_proxies(&self, params: &ListReplacedProxiesParams) -> Result<PaginatedResponse<ReplacedProxy>>` — GET `/api/v2/proxy/list/replaced/`
  - `download_replaced_proxies(&self, params: &DownloadReplacedProxiesParams) -> Result<String>` — GET download URL
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.5: Proxy Statistics — Models & Endpoints (4 endpoints)

> **Context:** Statistics and activity tracking. Note: `list_stats` returns an **array** not a paginated response. Activity download endpoint returns CSV and is unauthenticated.
> **Verification:** Compiles; `list_stats` returns `Vec<ProxyStat>` (not paginated).

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/proxy_stats.rs` with:
  - `ProxyStat` (response): timestamp, is_projected, bandwidth_total/average, requests_total/successful/failed, error_reasons, countries_used, number_of_proxies_used, protocols_used, average_concurrency, average_rps, last_request_sent_at
  - `AggregateStats` (response): bandwidth_projected, bandwidth_total, bandwidth_average, requests fields, etc.
  - `ProxyActivity` (response): timestamp, protocol, durations, error fields, addresses, bytes, etc.
  - `ListStatsParams`: timestamp__lte, timestamp__gte, plan_id
  - `ListActivitiesParams`: all filter params + starting_after
  - `DownloadActivitiesParams`: download_token, timestamp filters, etc.
- [ ] **Step 2:** Create `src/api/proxy_stats.rs`:
  - `list_stats(&self, params: &ListStatsParams) -> Result<Vec<ProxyStat>>` — GET `/api/v2/stats/` (returns array, NOT paginated)
  - `aggregate_stats(&self, params: &ListStatsParams) -> Result<AggregateStats>` — GET `/api/v2/stats/aggregate/`
  - `list_activities(&self, params: &ListActivitiesParams) -> Result<PaginatedResponse<ProxyActivity>>` — GET `/api/v2/proxy/activity/`
  - `download_activities(&self, params: &DownloadActivitiesParams) -> Result<String>` — GET download URL (CSV)
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.6: Subscription — Models & Endpoints (7 endpoints)

> **Context:** Subscription management including checkout flows. The `customize` and `pricing` endpoints take JSON-encoded query parameters (`?query={json}`). Purchase/renew return complex payment response objects.
> **Verification:** Compiles; `CustomizeQuery` and `PricingQuery` serialize to JSON for query param.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/subscription.rs` with:
  - `Subscription` (response): id, plan, payment_method, free_credits, term, dates, promotion fields, etc.
  - `AvailableAssets` (response): nested dict by proxy type — use `serde_json::Value` or typed map
  - `CustomizeQuery`: proxy_type, proxy_subtype, proxy_countries, required_site_checks, plan_id
  - `CustomizeResponse`: proxy_type, proxy_subtype, counts, available_countries, features, terms, etc.
  - `PricingQuery`: behavior, proxy_type, proxy_subtype, many configuration fields, plan_id
  - `PricingResponse`: discount_percentage, prices, credits, tiers, features, tax_breakdown
  - `PurchaseRequest`: extends PricingQuery + payment_method, recaptcha, behavior
  - `PurchaseResponse`: payment_required, plan, pending_payment, stripe fields
  - `RenewRequest`: payment_method, term, recaptcha
  - `RenewResponse`: payment_required, plan, pending_payment, stripe fields
- [ ] **Step 2:** Create `src/api/subscription.rs`:
  - `get_subscription(&self) -> Result<Subscription>`
  - `get_available_assets(&self) -> Result<AvailableAssets>`
  - `get_customization_options(&self, query: &CustomizeQuery) -> Result<CustomizeResponse>` — serialize query to JSON, pass as `?query=` param
  - `get_pricing(&self, query: &PricingQuery) -> Result<PricingResponse>`
  - `purchase_plan(&self, request: &PurchaseRequest) -> Result<PurchaseResponse>`
  - `renew_subscription(&self, request: &RenewRequest) -> Result<RenewResponse>`
  - `download_invoice(&self, subscription_transaction_id: i64) -> Result<Vec<u8>>` — returns PDF bytes
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.7: Plans — Models & Endpoints (5 endpoints)

> **Context:** Plan CRUD plus upgrade and cancel operations. Cancel returns a different response shape than other operations.
> **Verification:** Compiles; all 5 methods implemented.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/plan.rs` with:
  - `Plan` (response): id, status, bandwidth_limit, monthly_price, yearly_price, proxy_type, proxy_subtype, proxy_count, proxy_countries, site_checks, on_demand_refresh fields, replacement fields, subuser fields, feature flags, timestamps
  - `UpdatePlanRequest`: automatic_refresh_next_at
  - `UpgradePlanRequest`: same fields as pricing + payment_method, recaptcha
  - `UpgradePlanResponse`: payment_required, plan, pending_payment, stripe fields
  - `CancelPlanResponse`: success (bool), transaction (i64)
- [ ] **Step 2:** Create `src/api/plans.rs`:
  - `list_plans(&self, page: Option<u32>, page_size: Option<u32>) -> Result<PaginatedResponse<Plan>>`
  - `get_plan(&self, id: i64) -> Result<Plan>`
  - `update_plan(&self, id: i64, request: &UpdatePlanRequest) -> Result<Plan>`
  - `upgrade_plan(&self, id: i64, request: &UpgradePlanRequest) -> Result<UpgradePlanResponse>`
  - `cancel_plan(&self, id: i64) -> Result<CancelPlanResponse>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.8: Sub-Users — Models & Endpoints (7 endpoints)

> **Context:** Sub-user management. Includes masquerade functionality which adds an `X-Subuser` header — provide a helper method that returns a modified client or header value.
> **Verification:** Compiles; masquerade helper produces correct header.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/sub_user.rs` with:
  - `SubUser` (response): id, label, proxy_countries, proxy_limit, max_thread_count, aggregate_stats, timestamps, bandwidth_use dates
  - `CreateSubUserRequest`: label, proxy_limit (f64), max_thread_count (i64)
  - `UpdateSubUserRequest`: optional fields with skip_serializing_if
- [ ] **Step 2:** Create `src/api/sub_users.rs`:
  - `list_sub_users(&self, page: Option<u32>, plan_id: Option<i64>) -> Result<PaginatedResponse<SubUser>>`
  - `get_sub_user(&self, id: i64, plan_id: Option<i64>) -> Result<SubUser>`
  - `create_sub_user(&self, request: &CreateSubUserRequest, plan_id: Option<i64>) -> Result<SubUser>`
  - `update_sub_user(&self, id: i64, request: &UpdateSubUserRequest, plan_id: Option<i64>) -> Result<SubUser>`
  - `delete_sub_user(&self, id: i64, plan_id: Option<i64>) -> Result<()>`
  - `refresh_sub_user_proxy_list(&self, id: i64) -> Result<SubUser>`
  - `masquerade_header(sub_user_id: i64) -> (String, String)` — returns `("X-Subuser", "{id}")` for manual header injection
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.9: IP Authorization — Models & Endpoints (5 endpoints)

> **Context:** IP authorization CRUD plus the `whatsmyip` utility endpoint. Simple model structures.
> **Verification:** Compiles; all 5 methods implemented.

- **Priority:** P0
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/ip_authorization.rs` with:
  - `IpAuthorization` (response): id (i64), ip_address (String), created_at (String), last_used_at (Option<String>)
  - `CreateIpAuthRequest`: ip_address (String)
  - `WhatsMyIpResponse`: ip_address (String)
- [ ] **Step 2:** Create `src/api/ip_authorization.rs`:
  - `create_ip_authorization(&self, request: &CreateIpAuthRequest, plan_id: Option<i64>) -> Result<IpAuthorization>`
  - `list_ip_authorizations(&self, plan_id: Option<i64>, page: Option<u32>) -> Result<PaginatedResponse<IpAuthorization>>`
  - `get_ip_authorization(&self, id: &str, plan_id: Option<i64>) -> Result<IpAuthorization>`
  - `delete_ip_authorization(&self, id: &str, plan_id: Option<i64>) -> Result<()>`
  - `whats_my_ip(&self) -> Result<WhatsMyIpResponse>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.10: Notifications — Models & Endpoints (4 endpoints)

> **Context:** Notification listing with dismiss/restore lifecycle.
> **Verification:** Compiles; dismiss/restore return the notification object.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/notification.rs` with:
  - `Notification` (response): id, type_, is_dismissable, context (serde_json::Value), created_at, updated_at, dismissed_at (Option)
  - `ListNotificationsParams`: dismissed_at__isnull (Option<bool>), ordering (Option<String>), type_ (Option<String>)
- [ ] **Step 2:** Create `src/api/notifications.rs`:
  - `list_notifications(&self, params: &ListNotificationsParams) -> Result<PaginatedResponse<Notification>>`
  - `get_notification(&self, id: &str) -> Result<Notification>`
  - `dismiss_notification(&self, id: &str) -> Result<Notification>`
  - `restore_notification(&self, id: &str) -> Result<Notification>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.11: User Profile — Models & Endpoints (4 endpoints)

> **Context:** User profile and preferences CRUD. Simple get/update pattern.
> **Verification:** Compiles; all 4 methods implemented.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/user_profile.rs` with:
  - `UserProfile` (response): id, email, first_name, last_name, last_login, timezone, subscription notification flags, tracking_id, timestamps
  - `UpdateProfileRequest`: optional fields (timezone, first_name, last_name, notification prefs)
  - `UserPreferences` (response): id, survey dates, onboarding dates, etc.
  - `UpdatePreferencesRequest`: optional fields
- [ ] **Step 2:** Create `src/api/user_profile.rs`:
  - `get_profile(&self) -> Result<UserProfile>`
  - `update_profile(&self, request: &UpdateProfileRequest) -> Result<UserProfile>`
  - `get_preferences(&self) -> Result<UserPreferences>`
  - `update_preferences(&self, request: &UpdatePreferencesRequest) -> Result<UserPreferences>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.12: Registration & Login — Models & Endpoints (15 endpoints)

> **Context:** The largest single API group. Includes unauthenticated endpoints (register, login, reset password) and authenticated ones (change password, change email, delete account, logout). Some endpoints return tokens, others return 204.
> **Verification:** Compiles; unauthenticated endpoints don't send auth header.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/auth.rs` with:
  - `TokenResponse`: token (String)
  - `RegisterResponse`: token (String), logged_in_existing_user (bool)
  - `RegisterRequest`: email, password, recaptcha, tos_accepted (bool), marketing_email_accepted (bool)
  - `LoginRequest`: email, password, recaptcha
  - `SocialAuthRequest`: provider, code, redirect_uri, tos_accepted (Option), marketing_email_accepted (Option)
  - `ChangePasswordRequest`: password, new_password
  - `ResetPasswordRequest`: email, recaptcha
  - `ResetPasswordCompleteRequest`: password, password_reset_token, recaptcha
  - `ChangeEmailRequest`: password, new_email
  - `ChangeEmailCompleteRequest`: confirmation_code
  - `ActivationStatus`: email_is_verified, last_time_email_verification_email_sent, created_at, updated_at
  - `ActivationCompleteRequest`: activation_token
  - `DeleteAccountRequest`: password, recaptcha
  - `DeleteSocialAccountRequest`: provider, code, redirect_uri
- [ ] **Step 2:** Create `src/api/auth.rs` with `impl WebshareClient`:
  - `register(&self, request: &RegisterRequest) -> Result<RegisterResponse>` (unauthenticated)
  - `login(&self, request: &LoginRequest) -> Result<TokenResponse>` (unauthenticated)
  - `register_social(&self, request: &SocialAuthRequest) -> Result<RegisterResponse>` (unauthenticated)
  - `login_social(&self, request: &SocialAuthRequest) -> Result<TokenResponse>` (unauthenticated)
  - `change_password(&self, request: &ChangePasswordRequest) -> Result<()>`
  - `reset_password(&self, request: &ResetPasswordRequest) -> Result<()>` (unauthenticated)
  - `reset_password_complete(&self, request: &ResetPasswordCompleteRequest) -> Result<TokenResponse>` (unauthenticated)
  - `change_email(&self, request: &ChangeEmailRequest) -> Result<()>`
  - `change_email_complete(&self, request: &ChangeEmailCompleteRequest) -> Result<()>`
  - `get_activation_status(&self) -> Result<ActivationStatus>`
  - `resend_activation_email(&self) -> Result<ActivationStatus>`
  - `complete_activation(&self, request: &ActivationCompleteRequest) -> Result<TokenResponse>` (unauthenticated)
  - `delete_account(&self, request: &DeleteAccountRequest) -> Result<()>`
  - `delete_social_account(&self, request: &DeleteSocialAccountRequest) -> Result<()>`
  - `logout(&self) -> Result<()>`
- [ ] **Step 3:** For unauthenticated endpoints, the request helper should skip the `Authorization` header. Add internal method: `fn post_unauthenticated(&self, path: &str) -> hpx::RequestBuilder`.
- [ ] **Step 4:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.13: Two-Factor Auth — Models & Endpoints (5 endpoints)

> **Context:** 2FA management including code entry, method management, and activation.
> **Verification:** Compiles; all 5 methods implemented.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/two_factor_auth.rs` with:
  - `TwoFactorMethod` (response): id, type_ (String), active (bool), created_at, updated_at, secret_key (Option<String>)
  - `Enter2faCodeRequest`: code (String), recaptcha (String)
  - `Change2faMethodRequest`: type_ (String) — "email_code" or "device_totp"
  - `Activate2faMethodRequest`: code_1 (String), code_2 (String)
  - `Resend2faEmailResponse`: email_sent (bool)
- [ ] **Step 2:** Create `src/api/two_factor_auth.rs`:
  - `enter_2fa_code(&self, request: &Enter2faCodeRequest) -> Result<()>`
  - `get_2fa_method(&self) -> Result<TwoFactorMethod>`
  - `change_2fa_method(&self, request: &Change2faMethodRequest) -> Result<TwoFactorMethod>`
  - `activate_2fa_method(&self, id: i64, request: &Activate2faMethodRequest) -> Result<TwoFactorMethod>`
  - `resend_2fa_email(&self) -> Result<Resend2faEmailResponse>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.14: Account Verification — Models & Endpoints (13 endpoints)

> **Context:** Complex verification system with flows, questions, answers, evidence submission (multipart), appeals, abuse reports, categories, limits, thresholds, and suspension. Evidence and answer submission require `hpx::multipart::Form`.
> **Verification:** Compiles; multipart form construction works for evidence submission.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/verification.rs` with:
  - `VerificationFlow`: id, type_, state, started_at, updated_at, needs_evidence, evidence, id_verification_restores_access, id_verification_required
  - `VerificationQuestion`: id, question, created_at, updated_at, flow (i64), answer (Option<VerificationAnswer>)
  - `VerificationAnswer`: id, answer, created_at, updated_at, files (Vec<String>)
  - `VerificationCategory`: description, request_threshold, id_verification_required, id_verification_restores_access
  - `VerificationLimits`: proxy_state (String)
  - `VerificationThreshold`: description, id_verification_required, id_verification_restores_access, request_count, request_threshold, triggered
  - `Suspension`: created_at (String), reason (String)
  - `AbuseReport`: id, content, flow, created_at, updated_at
  - `Appeal`: id, appeal, state, created_at, updated_at
  - `SubmitAppealRequest`: appeal (String)
  - `SubmitSecurityCodeRequest`: security_code (String)
  - `ListQuestionsParams`: various filter params
- [ ] **Step 2:** Create `src/api/verification.rs`:
  - `list_verification_flows(&self, page: Option<u32>) -> Result<PaginatedResponse<VerificationFlow>>`
  - `get_verification_flow(&self, id: i64) -> Result<VerificationFlow>`
  - `submit_evidence(&self, flow_id: i64, explanation: &str, files: Vec<(String, Vec<u8>)>) -> Result<VerificationFlow>` — multipart form
  - `submit_security_code(&self, flow_id: i64, request: &SubmitSecurityCodeRequest) -> Result<VerificationFlow>`
  - `get_verification_categories(&self) -> Result<serde_json::Value>` — returns dict by category name
  - `get_verification_limits(&self) -> Result<VerificationLimits>`
  - `get_verification_thresholds(&self) -> Result<serde_json::Value>` — returns dict by category
  - `get_suspension(&self) -> Result<Suspension>`
  - `list_verification_questions(&self, params: &ListQuestionsParams) -> Result<PaginatedResponse<VerificationQuestion>>`
  - `submit_answer(&self, question_id: i64, answer: &str, files: Option<Vec<(String, Vec<u8>)>>) -> Result<VerificationAnswer>` — multipart form
  - `list_abuse_reports(&self, page: Option<u32>) -> Result<PaginatedResponse<AbuseReport>>`
  - `list_appeals(&self, state: Option<&str>, page: Option<u32>) -> Result<PaginatedResponse<Appeal>>`
  - `submit_appeal(&self, request: &SubmitAppealRequest) -> Result<Appeal>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.15: ID Verification — Models & Endpoints (3 endpoints)

> **Context:** Simple 3-endpoint group for Stripe identity verification flow.
> **Verification:** Compiles; all 3 methods implemented.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/id_verification.rs` with:
  - `IdVerification` (response): id, state, client_secret (Option<String>), verification_failure_times, max_verification_failure_times, created_at, updated_at, verified_at (Option<String>)
- [ ] **Step 2:** Create `src/api/id_verification.rs`:
  - `get_id_verification(&self) -> Result<IdVerification>`
  - `start_id_verification(&self) -> Result<IdVerification>`
  - `complete_id_verification(&self) -> Result<IdVerification>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.16: Billing & Payments — Models & Endpoints (9 endpoints)

> **Context:** Billing info, payment methods, transactions, and pending payments. Multiple sub-groups that share some patterns.
> **Verification:** Compiles; all 9 methods implemented.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/billing.rs` with:
  - `BillingInfo`: id, name, address, billing_email, created_at, updated_at
  - `UpdateBillingInfoRequest`: name (Option), address (Option), billing_email (Option)
  - `PaymentMethod`: id, type_, brand (Option), last4 (Option), expiration_year/month (Option), timestamps, address fields (Option)
  - `UpdatePaymentMethodRequest`: recaptcha (String)
  - `UpdatePaymentMethodResponse`: pending_payment (Option), stripe_client_secret, stripe_payment_intent
  - `Transaction`: id, status, payment_method (Option<PaymentMethod>), reason, amount, credits_used, credits_gained, refund_amount, refund_date, timestamps
  - `PendingPayment`: id, status, failure_reason, payment_method, plan, transaction, is_renewal, term, timestamps, completed_at (Option)
- [ ] **Step 2:** Create `src/api/billing.rs`:
  - `get_billing_info(&self) -> Result<BillingInfo>`
  - `update_billing_info(&self, request: &UpdateBillingInfoRequest) -> Result<BillingInfo>`
  - `list_payment_methods(&self, page: Option<u32>) -> Result<PaginatedResponse<PaymentMethod>>`
  - `get_payment_method(&self, id: i64) -> Result<PaymentMethod>`
  - `update_payment_method(&self, request: &UpdatePaymentMethodRequest) -> Result<UpdatePaymentMethodResponse>`
  - `list_transactions(&self, page: Option<u32>) -> Result<PaginatedResponse<Transaction>>`
  - `get_transaction(&self, id: i64) -> Result<Transaction>`
  - `list_pending_payments(&self, page: Option<u32>) -> Result<PaginatedResponse<PendingPayment>>`
  - `get_pending_payment(&self, id: i64) -> Result<PendingPayment>`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.17: Referral & Affiliate — Models & Endpoints (7 endpoints)

> **Context:** Referral system with config, credits, earn-outs, and public referral code lookup. The public endpoint (`get_referral_code_info`) is unauthenticated.
> **Verification:** Compiles; public endpoint skips auth header.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/referral.rs` with:
  - `ReferralConfig`: id, mode, paypal_payout_email, id_verification_required, credits/payouts earned, user counts, earn_out_frequency, dates, amounts, referral_code, promo fields, referral URL, timestamps
  - `UpdateReferralConfigRequest`: mode (Option), paypal_payout_email (Option)
  - `ReferralCredit`: id, user_id, mode, amount, status, created_at, updated_at, reverted_at (Option)
  - `ReferralEarnOut`: id, mode, paypal_payout_email, amount, status, error_reason, timestamps
  - `ReferralCodeInfo`: referral_code, promo_type, promo_value
- [ ] **Step 2:** Create `src/api/referral.rs`:
  - `get_referral_config(&self) -> Result<ReferralConfig>`
  - `update_referral_config(&self, request: &UpdateReferralConfigRequest) -> Result<ReferralConfig>`
  - `list_referral_credits(&self, page: Option<u32>) -> Result<PaginatedResponse<ReferralCredit>>`
  - `get_referral_credit(&self, id: i64) -> Result<ReferralCredit>`
  - `list_referral_earnouts(&self, page: Option<u32>) -> Result<PaginatedResponse<ReferralEarnOut>>`
  - `get_referral_earnout(&self, id: i64) -> Result<ReferralEarnOut>`
  - `get_referral_code_info(&self, referral_code: &str) -> Result<ReferralCodeInfo>` (unauthenticated)
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

### Task 2.18: Downloads — Models & Endpoints (2 endpoints)

> **Context:** Download token management. Simple get/reset pattern with `scope` path parameter.
> **Verification:** Compiles; both methods implemented.

- **Priority:** P1
- **Scope:** Complete API group implementation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Create `src/models/download_token.rs` with:
  - `DownloadToken`: id, key (String), scope (String), expire_at (String)
  - `DownloadTokenScope` enum: `ProxyList`, `ReplacedProxy`, `Activity` — with `Display` and `FromStr` for path serialization
- [ ] **Step 2:** Create `src/api/downloads.rs`:
  - `get_download_token(&self, scope: DownloadTokenScope) -> Result<DownloadToken>` — GET `/api/v2/download_token/{scope}/`
  - `reset_download_token(&self, scope: DownloadTokenScope) -> Result<DownloadToken>` — POST `/api/v2/download_token/{scope}/reset/`
- [ ] **Step 3:** Register modules.
- [ ] **Verification:** `cargo check`.

---

## Phase 3: Integration & Features

### Task 3.1: Wire All Modules in `lib.rs` and `mod.rs` Files

> **Context:** After all API groups are implemented, ensure all modules are properly declared, re-exported, and the public API surface is clean and well-organized.
> **Verification:** `cargo check --all-targets`; `cargo doc --no-deps` builds without warnings.

- **Priority:** P0
- **Scope:** Module wiring
- **Status:** 🔴 TODO

- [ ] **Step 1:** Finalize `src/models/mod.rs` — ensure all 18 model modules are declared and have `pub use` re-exports for key types.
- [ ] **Step 2:** Finalize `src/api/mod.rs` — ensure all 18 API modules are declared.
- [ ] **Step 3:** Finalize `src/lib.rs` — ensure top-level re-exports are complete:
  - `pub use client::*`
  - `pub use error::*`
  - `pub use pagination::*`
  - `pub mod models` (users import specific types from `webshare_rs::models::*`)
- [ ] **Step 4:** Verify all public types are accessible from the crate root or one level deep.
- [ ] **Verification:** `cargo doc --no-deps` generates complete documentation; `cargo check --all-targets`.

---

### Task 3.2: Add Unauthenticated Request Support

> **Context:** Several endpoints (register, login, reset password, referral code info, download endpoints) don't require auth. The client needs internal helpers that skip the `Authorization` header.
> **Verification:** Unauthenticated methods create requests without `Authorization` header.

- **Priority:** P0
- **Scope:** Client enhancement
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add to `WebshareClient`:
  - `fn request_unauthed(&self, method: hpx::Method, path: &str) -> hpx::RequestBuilder` — same as `request()` but without auth header.
  - `fn post_unauthed(&self, path: &str) -> hpx::RequestBuilder`
  - `fn get_unauthed(&self, path: &str) -> hpx::RequestBuilder`
- [ ] **Step 2:** Update all unauthenticated API methods (from Tasks 2.2, 2.4, 2.5, 2.12, 2.17) to use unauthed helpers.
- [ ] **Verification:** Unit test: `request_unauthed()` does not include `Authorization` header.

---

### Task 3.3: Remove Old Code & Clean Up

> **Context:** Old files (`rest.rs`, `apis/proxy_list_api.rs`, `models/proxy_list.rs`) should already be deleted in Task 1.5. Verify no remnants exist. Clean up any `#[allow]` attributes.
> **Verification:** No references to `reqwest` in the codebase; no old files remain.

- **Priority:** P1
- **Scope:** Cleanup
- **Status:** 🔴 TODO

- [ ] **Step 1:** `grep -r "reqwest" src/` returns zero results.
- [ ] **Step 2:** `grep -r "eyre" src/` returns zero results (except possibly in test utilities if needed).
- [ ] **Step 3:** No `#[allow(non_snake_case)]` in `lib.rs`.
- [ ] **Step 4:** Verify `Cargo.toml` has no `reqwest` or `eyre` dependency.
- [ ] **Verification:** `cargo machete` reports no unused dependencies; grep confirms no forbidden imports.

---

## Phase 4: Polish, QA & Docs

### Task 4.1: Add Unit Tests for Models (Serialization/Deserialization)

> **Context:** Every model struct needs at least one serde round-trip test using representative JSON from the API docs. Tests go in `#[cfg(test)] mod tests` within each model file.
> **Verification:** `cargo test` passes; each model file has at least one test.

- **Priority:** P1
- **Scope:** Unit testing
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add tests to each `src/models/*.rs` file:
  - Test deserializing representative JSON into the response struct.
  - Test serializing request structs and verifying JSON output.
  - Test that `Option::None` fields are omitted via `skip_serializing_if`.
- [ ] **Step 2:** Add tests for `PaginatedResponse<T>` with various item types.
- [ ] **Step 3:** Add tests for `WebshareError` display formatting.
- [ ] **Step 4:** Add tests for `DownloadTokenScope` `Display`/`FromStr`.
- [ ] **Verification:** `cargo test --all-features` — all tests pass.

---

### Task 4.2: Add Unit Tests for Client & URL Construction

> **Context:** Verify that the client constructs correct URLs, adds correct headers, and handles different configurations.
> **Verification:** `cargo test` passes; client URL/header construction is validated.

- **Priority:** P1
- **Scope:** Unit testing
- **Status:** 🔴 TODO

- [ ] **Step 1:** Test `WebshareClient::new()` sets correct defaults.
- [ ] **Step 2:** Test `WebshareClientBuilder` with custom base_url and timeout.
- [ ] **Step 3:** Test debug output redacts token.
- [ ] **Step 4:** Test `PaginatedResponse` helper methods (`is_empty`, `has_next`).
- [ ] **Verification:** `cargo test --all-features` — all client tests pass.

---

### Task 4.3: Doc Comments & Crate Documentation

> **Context:** Every public type, method, and field should have `///` doc comments. Module-level `//!` docs describe each API group. Crate-level docs provide a quick-start example.
> **Verification:** `cargo doc --no-deps` builds without warnings; key types have documentation.

- **Priority:** P2
- **Scope:** Documentation
- **Status:** 🔴 TODO

- [ ] **Step 1:** Add crate-level `//!` doc comment to `src/lib.rs` with usage example.
- [ ] **Step 2:** Add `///` doc comments to all public structs in `src/models/*.rs` — reference the API docs URL.
- [ ] **Step 3:** Add `///` doc comments to all public methods in `src/api/*.rs` — include HTTP method, path, and brief description.
- [ ] **Step 4:** Add `//!` module docs to each `src/api/*.rs` and `src/models/*.rs` file.
- [ ] **Verification:** `cargo doc --no-deps` builds cleanly; spot-check key types have docs.

---

### Task 4.4: Linting, Formatting & Final Verification

> **Context:** Run the full project quality checklist. All lint, format, and test commands must pass.
> **Verification:** `just lint`, `just format`, `just test` all pass.

- **Priority:** P0
- **Scope:** Quality assurance
- **Status:** 🔴 TODO

- [ ] **Step 1:** Run `cargo +nightly fmt --all` — fix any formatting issues.
- [ ] **Step 2:** Run `cargo +nightly clippy --all -- -D warnings` — fix all warnings.
- [ ] **Step 3:** Run `cargo test --all-features` — all tests pass.
- [ ] **Step 4:** Run `cargo machete` — no unused dependencies.
- [ ] **Step 5:** Run `cargo doc --no-deps` — no documentation warnings.
- [ ] **Step 6:** Run `cargo check --all-targets --all-features` — compiles cleanly.
- [ ] **Step 7:** Verify no Chinese characters: `rg --line-number --column "\p{Han}" src/` returns nothing.
- [ ] **Verification:** All commands pass with zero errors/warnings.

---

## Summary & Timeline

| Phase | Tasks | Target Date |
| :--- | :---: | :--- |
| **1. Foundation** | 5 | 02-24 |
| **2. Core Logic** | 18 | 03-04 |
| **3. Integration** | 3 | 03-06 |
| **4. Polish** | 4 | 03-08 |
| **Total** | **30** | |

## Definition of Done

1. [ ] **Linted:** `cargo +nightly clippy --all -- -D warnings` passes with zero warnings.
2. [ ] **Tested:** Unit tests for all model types and client construction.
3. [ ] **Formatted:** `cargo +nightly fmt --all -- --check` passes.
4. [ ] **Verified:** Each task's specific Verification criterion met.
5. [ ] **No forbidden deps:** Zero `reqwest`, `eyre`, `anyhow`, `log` usage.
6. [ ] **Complete coverage:** All ~108 Webshare API endpoints have typed methods.
7. [ ] **Documented:** `cargo doc --no-deps` builds without warnings.
8. [ ] **English Only:** No Chinese characters in source code or comments.
