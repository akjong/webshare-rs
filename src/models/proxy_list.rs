use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize)]
pub struct ProxyListQueryParams {
    /// Must be either `direct` or `backbone`. Required field. This field must be `backbone` if `plan.pool_filter` is `residential`.
    pub mode: String,
    /// Filter by comma separated country code. Example filtering USA and French proxies: `FR,US`. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code__in: Option<String>,
    /// Filter by a search phrase. Can accept arbitrary text. Search does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    /// Comma separated list of fields to specify ordering. Reverse ordering (DESC) can be achieved by adding minus in front of the field. Example: `-valid,proxy_address`. Ordering is not supported in the `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ordering: Option<String>,
    /// Filter by proxy create date. `created_at` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Filter by a specific proxy address. `proxy_address` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_address: Option<String>,
    /// Filter by comma separated proxy addresses. `proxy_address__in` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy_address__in: Option<String>,
    /// Filter by the validity of the proxy address. `valid` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid: Option<bool>,
    /// Filter by the the proxy address ASN number. `asn_number` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_number: Option<String>,
    /// Filter by the the proxy address ASN name. `asn_name` filter does not work in `backbone` mode. Optional field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asn_name: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WebshareResult<T> {
    pub count: u32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Proxy {
    pub id: String,
    pub username: String,
    pub password: String,
    pub proxy_address: String,
    pub port: u32,
    pub valid: bool,
    pub last_verification: String,
    pub country_code: String,
    pub city_name: String,
    pub created_at: String,
}
