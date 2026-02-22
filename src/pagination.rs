//! Pagination types for the Webshare API.
//!
//! The Webshare API returns paginated responses for list endpoints. This module
//! provides the [`PaginatedResponse`] generic struct that deserializes these
//! responses and exposes convenient helper methods for inspecting pagination
//! state.

use serde::{Deserialize, Serialize};

/// A generic paginated response returned by Webshare list endpoints.
///
/// The API paginates large collections and returns them in pages. Each page
/// contains a `results` vector along with metadata indicating the total `count`
/// and optional URLs for the `next` and `previous` pages.
///
/// # Type Parameter
///
/// * `T` — The type of each item in the `results` vector. Must implement
///   [`Serialize`] and [`DeserializeOwned`](serde::de::DeserializeOwned).
///
/// # Examples
///
/// ```rust
/// use webshare_rs::pagination::PaginatedResponse;
///
/// let json = r#"{"count":2,"next":"https://proxy.webshare.io/api/v2/proxy/list/?page=2","previous":null,"results":[{"id":1},{"id":2}]}"#;
/// let page: PaginatedResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
///
/// assert_eq!(page.count, 2);
/// assert!(page.has_next());
/// assert!(!page.has_previous());
/// assert_eq!(page.len(), 2);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + serde::de::DeserializeOwned")]
pub struct PaginatedResponse<T> {
    /// Total number of items across all pages.
    pub count: u64,

    /// URL of the next page, if one exists.
    pub next: Option<String>,

    /// URL of the previous page, if one exists.
    pub previous: Option<String>,

    /// The items returned in this page.
    pub results: Vec<T>,
}

impl<T> PaginatedResponse<T> {
    /// Returns `true` if this page contains no results.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use webshare_rs::pagination::PaginatedResponse;
    /// let page: PaginatedResponse<()> = PaginatedResponse {
    ///     count: 0,
    ///     next: None,
    ///     previous: None,
    ///     results: vec![],
    /// };
    /// assert!(page.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.results.is_empty()
    }

    /// Returns `true` if there is a next page available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use webshare_rs::pagination::PaginatedResponse;
    /// let page: PaginatedResponse<u32> = PaginatedResponse {
    ///     count: 10,
    ///     next: Some("https://proxy.webshare.io/api/v2/proxy/list/?page=2".into()),
    ///     previous: None,
    ///     results: vec![1, 2, 3],
    /// };
    /// assert!(page.has_next());
    /// ```
    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    /// Returns `true` if there is a previous page available.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use webshare_rs::pagination::PaginatedResponse;
    /// let page: PaginatedResponse<u32> = PaginatedResponse {
    ///     count: 10,
    ///     next: None,
    ///     previous: Some("https://proxy.webshare.io/api/v2/proxy/list/?page=1".into()),
    ///     results: vec![4, 5, 6],
    /// };
    /// assert!(page.has_previous());
    /// ```
    pub fn has_previous(&self) -> bool {
        self.previous.is_some()
    }

    /// Returns the number of items in this page.
    ///
    /// Note: this is the count of items on the *current* page, not the total
    /// count across all pages. Use [`count`](Self::count) for the total.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use webshare_rs::pagination::PaginatedResponse;
    /// let page: PaginatedResponse<&str> = PaginatedResponse {
    ///     count: 100,
    ///     next: None,
    ///     previous: None,
    ///     results: vec!["a", "b", "c"],
    /// };
    /// assert_eq!(page.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.results.len()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::Value;

    use super::*;

    #[test]
    fn deserialize_paginated_response() {
        let json = r#"{"count":1,"next":null,"previous":null,"results":[{"id":1}]}"#;
        let page: PaginatedResponse<Value> = serde_json::from_str(json).unwrap();

        assert_eq!(page.count, 1);
        assert!(page.next.is_none());
        assert!(page.previous.is_none());
        assert_eq!(page.results.len(), 1);
        assert_eq!(page.results[0]["id"], 1);
    }

    #[test]
    fn helper_methods_with_results() {
        let page: PaginatedResponse<Value> = PaginatedResponse {
            count: 5,
            next: Some("https://proxy.webshare.io/api/v2/proxy/list/?page=2".into()),
            previous: None,
            results: vec![Value::from(1), Value::from(2)],
        };

        assert!(!page.is_empty());
        assert!(page.has_next());
        assert!(!page.has_previous());
        assert_eq!(page.len(), 2);
    }

    #[test]
    fn helper_methods_with_empty_results() {
        let page: PaginatedResponse<Value> = PaginatedResponse {
            count: 0,
            next: None,
            previous: None,
            results: vec![],
        };

        assert!(page.is_empty());
        assert!(!page.has_next());
        assert!(!page.has_previous());
        assert_eq!(page.len(), 0);
    }

    #[test]
    fn serialize_roundtrip() {
        let page: PaginatedResponse<Value> = PaginatedResponse {
            count: 2,
            next: Some("https://example.com/?page=2".into()),
            previous: Some("https://example.com/?page=0".into()),
            results: vec![Value::from("a"), Value::from("b")],
        };

        let json = serde_json::to_string(&page).unwrap();
        let deserialized: PaginatedResponse<Value> = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.count, 2);
        assert!(deserialized.has_next());
        assert!(deserialized.has_previous());
        assert_eq!(deserialized.len(), 2);
    }
}
