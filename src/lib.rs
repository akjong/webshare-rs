//! # webshare-rs
//!
//! A Rust client for the [Webshare](https://www.webshare.io/) proxy API.
//!
//! ## Quick start
//!
//! ```rust,no_run
//! # async fn example() -> webshare_rs::error::Result<()> {
//! use webshare_rs::{models::proxy::ListProxiesParams, WebshareClient};
//!
//! let client = WebshareClient::new("your-api-token");
//! let proxies = client.list_proxies(&ListProxiesParams::default()).await?;
//! println!("Total proxies: {}", proxies.count);
//! # Ok(())
//! # }
//! ```

pub mod api;
pub mod client;
pub mod error;
pub mod models;
pub mod pagination;

// Re-exports for convenience.
pub use client::{WebshareClient, WebshareClientBuilder};
pub use error::{Result, WebshareError};
pub use pagination::PaginatedResponse;
