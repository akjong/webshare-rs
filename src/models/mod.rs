//! Request and response model types for the Webshare API.
//!
//! Each sub-module contains the data structures (with serde derives) for a
//! specific API resource group.

pub mod api_key;
pub mod auth;
pub mod billing;
pub mod download;
pub mod id_verification;
pub mod ip_authorization;
pub mod notification;
pub mod plan;
pub mod proxy;
pub mod proxy_config;
pub mod proxy_replacement;
pub mod proxy_stats;
pub mod referral;
pub mod sub_user;
pub mod subscription;
pub mod two_factor_auth;
pub mod user_profile;
pub mod verification;
