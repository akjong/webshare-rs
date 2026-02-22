//! Download token model types.

use std::{fmt, str::FromStr};

use serde::{Deserialize, Serialize};

/// Supported download token scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadTokenScope {
    ProxyList,
    ReplacedProxy,
    Activity,
}

impl fmt::Display for DownloadTokenScope {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProxyList => write!(f, "proxy_list"),
            Self::ReplacedProxy => write!(f, "replaced_proxy"),
            Self::Activity => write!(f, "activity"),
        }
    }
}

impl FromStr for DownloadTokenScope {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "proxy_list" => Ok(Self::ProxyList),
            "replaced_proxy" => Ok(Self::ReplacedProxy),
            "activity" => Ok(Self::Activity),
            other => Err(format!("unknown download token scope: {other}")),
        }
    }
}

/// A download token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadToken {
    pub id: i64,
    pub key: String,
    pub scope: DownloadTokenScope,
    pub expire_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_download_token() {
        let json = r#"{
            "id": 56,
            "key": "abcdefghijklmnopqrstuvwxyz",
            "scope": "activity",
            "expire_at": "2022-06-14T11:58:10.246406-07:00"
        }"#;
        let token: DownloadToken = serde_json::from_str(json).unwrap();
        assert_eq!(token.scope, DownloadTokenScope::Activity);
        assert_eq!(token.key, "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn display_scope() {
        assert_eq!(DownloadTokenScope::ProxyList.to_string(), "proxy_list");
        assert_eq!(
            DownloadTokenScope::ReplacedProxy.to_string(),
            "replaced_proxy"
        );
        assert_eq!(DownloadTokenScope::Activity.to_string(), "activity");
    }

    #[test]
    fn parse_scope() {
        assert_eq!(
            "proxy_list".parse::<DownloadTokenScope>().unwrap(),
            DownloadTokenScope::ProxyList
        );
        assert!("unknown".parse::<DownloadTokenScope>().is_err());
    }
}
