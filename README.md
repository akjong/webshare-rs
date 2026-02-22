# webshare-rs

[![crates.io](https://img.shields.io/crates/v/webshare-rs.svg)](https://crates.io/crates/webshare-rs)
[![docs.rs](https://docs.rs/webshare-rs/badge.svg)](https://docs.rs/webshare-rs)

An ergonomic, fully-typed Rust SDK for the [Webshare](https://www.webshare.io/) proxy API, covering all 100+ endpoints across 18 API groups.

## Features

- **Complete coverage** — all Webshare REST API endpoints implemented
- **Async-first** — built on [`hpx`](https://github.com/longcipher/hpx) (BoringSSL TLS, Tokio)
- **Strongly typed** — request/response models derived with `serde`; pagination via a generic `PaginatedResponse<T>`
- **Ergonomic builder** — configure base URL, API key, and custom HTTP client via `WebshareClientBuilder`
- **Structured errors** — `WebshareError` powered by `thiserror`

## Installation

```toml
[dependencies]
webshare-rs = "0.1"
```

Or with `cargo add`:

```sh
cargo add webshare-rs
```

## Quick Start

```rust,no_run
use webshare_rs::{WebshareClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WebshareClient::builder()
        .api_key("your_api_key_here")
        .build()?;

    // List your proxies (first page, 25 per page)
    let proxies = client.proxy_list().list(25, 0).await?;
    println!("Total proxies: {}", proxies.count);
    for proxy in proxies.results {
        println!("  {}:{} ({})", proxy.proxy_address, proxy.port, proxy.country_code);
    }

    // Fetch your user profile
    let profile = client.user_profile().retrieve().await?;
    println!("Logged in as: {} {}", profile.first_name, profile.last_name);

    Ok(())
}
```

## API Coverage

| Group | Methods |
|---|---|
| **API Keys** | create, retrieve, list, update, delete |
| **Auth** | local account, social account, activate, change email/password, reset password, logout, delete account |
| **Billing** | retrieve billing, list transactions, list payment methods, list pending payments, update payment method |
| **Downloads** | get download token, reset download token |
| **ID Verification** | retrieve, start, complete |
| **IP Authorization** | create, retrieve, list, delete, whatsmyip |
| **Notifications** | retrieve, list, dismiss, restore |
| **Plans** | list plans, get pricing |
| **Proxy Config** | get config, update config, get stats, get status, allocate unallocated countries |
| **Proxy List** | list, download, on-demand refresh |
| **Proxy Replacement** | get replacement config, get replaced proxy, request replacement, on-demand replace |
| **Proxy Stats** | list stats, aggregate stats, list activity, download activity |
| **Referral** | get referral info, get referral credit, get referral earnout |
| **Sub Users** | create, retrieve, list, update, delete, masquerade, refresh proxy list |
| **Subscription** | get plan, get assets, customize, get pricing, purchase plan, renew, download invoice |
| **Two-Factor Auth** | get method, activate method, change method, enter code |
| **User Profile** | retrieve, update timezone, update last name, update first name |
| **Verification** | retrieve, send email, resend email |

## Usage

### Client Construction

```rust,no_run
use webshare_rs::WebshareClient;

// Read API key from environment (recommended)
let api_key = std::env::var("WEBSHARE_API_KEY").expect("WEBSHARE_API_KEY must be set");
let client = WebshareClient::builder()
    .api_key(api_key)
    .build()
    .expect("failed to build client");
```

### Pagination

All list endpoints return `PaginatedResponse<T>` with `count`, `next`, `previous`, and `results`:

```rust,no_run
use webshare_rs::{WebshareClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WebshareClient::builder()
        .api_key(std::env::var("WEBSHARE_API_KEY").unwrap())
        .build()?;

    let page = client.proxy_list().list(100, 0).await?;
    println!("{} total proxies, showing {}", page.count, page.results.len());
    Ok(())
}
```

### Error Handling

```rust,no_run
use webshare_rs::{WebshareClient, WebshareError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = WebshareClient::builder()
        .api_key("invalid_key")
        .build()?;

    match client.user_profile().retrieve().await {
        Ok(profile) => println!("Hello, {}!", profile.first_name),
        Err(WebshareError::ApiError { status, .. }) => {
            eprintln!("API error: HTTP {}", status);
        }
        Err(e) => eprintln!("Other error: {}", e),
    }
    Ok(())
}
```

## Testing

### Unit Tests

Run unit and doc tests (no API key required):

```sh
just test
```

### Integration Tests

Integration tests call the real Webshare API. Provide `WEBSHARE_API_KEY` in your environment:

```sh
WEBSHARE_API_KEY=<your_key> just test-integration
```

To run all tests (unit + integration + doc) sequentially:

```sh
WEBSHARE_API_KEY=<your_key> just test-all
```

> **Note:** Integration tests must run with `--test-threads=1` due to shared TLS session state.

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-feature`)
3. Commit your changes following [Conventional Commits](https://www.conventionalcommits.org/)
4. Push to the branch and open a Pull Request

Run `just ci` before submitting to ensure lint and tests pass.

## Changelog

See [GitHub releases](https://github.com/akjong/webshare-rs/releases) for the full history of changes.

## License

Licensed under the [Apache-2.0 License](LICENSE).
