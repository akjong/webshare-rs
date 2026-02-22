//! API endpoint implementations.
//!
//! Each sub-module implements the methods on [`crate::WebshareClient`] for a
//! specific Webshare API resource group.

pub mod api_keys;
pub mod auth;
pub mod billing;
pub mod downloads;
pub mod id_verification;
pub mod ip_authorization;
pub mod notifications;
pub mod plans;
pub mod proxy_config;
pub mod proxy_list;
pub mod proxy_replacement;
pub mod proxy_stats;
pub mod referral;
pub mod sub_users;
pub mod subscription;
pub mod two_factor_auth;
pub mod user_profile;
pub mod verification;
