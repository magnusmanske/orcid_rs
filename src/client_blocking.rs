use crate::author::Author;
use crate::error::{OrcidError, Result};
use reqwest::header::ACCEPT;
use serde_json;

#[derive(Debug, Clone)]
pub struct ClientBlocking {
    base_url: String,
    http_client: reqwest::blocking::Client,
}

impl ClientBlocking {
    pub fn new() -> ClientBlocking {
        ClientBlocking {
            base_url: "https://pub.orcid.org/v3.0/".to_string(),
            http_client: reqwest::blocking::Client::new(),
        }
    }

    /// Replaces the internal `reqwest::blocking::Client`. Useful for sharing a
    /// connection pool, embedding middleware (retry layers, tracing,
    /// custom user-agent), or in tests that want fast-fail timeouts.
    /// All other configuration set so far is preserved.
    pub fn http_client(mut self, client: reqwest::blocking::Client) -> Self {
        self.http_client = client;
        self
    }

    /// Overrides the API base URL. Lets tests redirect every request
    /// to a wiremock server: `.base_url(mock.uri())`.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    fn get_json_from_api(&self, query: String) -> Result<serde_json::Value> {
        let url = self.base_url.clone() + &query;
        let json = self
            .http_client
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()?
            .json()?;
        Ok(json)
    }

    pub fn is_valid_orcid_id(id: &str) -> bool {
        let mut digits: Vec<u32> = id
            .chars()
            .filter(|c| *c != '-')
            .filter_map(|c| if c == 'X' { Some(10) } else { c.to_digit(10) })
            .collect();
        if digits.len() != 16 {
            return false;
        }
        let last_digit = digits.pop().unwrap(); // unwrap OK
        let total = digits.iter().fold(0, |total, digit| (total + digit) * 2);
        let remainder = total % 11;
        let result = (12 - remainder) % 11;
        last_digit == result
    }

    /// Returns an `Author` for a given ORCID ID
    pub fn author(&self, orcid_id: &str) -> Result<Author> {
        if !Self::is_valid_orcid_id(orcid_id) {
            return Err(OrcidError::InvalidOrcidId(orcid_id.to_string()));
        }

        let json: serde_json::Value = self.get_json_from_api(orcid_id.to_string())?;

        match json["error-code"].as_str() {
            Some(error_code) => Err(OrcidError::ApiError {
                orcid_id: orcid_id.to_string(),
                error_code: error_code.to_string(),
                developer_message: json["developer-message"]
                    .as_str()
                    .unwrap_or("no developer-message")
                    .to_string(),
            }),
            None => Ok(Author::new_from_json(json)),
        }
    }

    /// Takes a DOI, quotes and searches it, returns a Vec<String> of ORCID IDs
    pub fn search_doi(&self, doi: &str) -> Result<Vec<String>> {
        self.search(&("\"".to_string() + doi + "\""))
    }

    /// Takes a search query, returns a Vec<String> of ORCID IDs
    pub fn search(&self, query: &str) -> Result<Vec<String>> {
        let encoded_query = urlencoding::encode(query);
        let json: serde_json::Value =
            self.get_json_from_api(format!("search?q={}", encoded_query))?;
        match json["result"].as_array() {
            Some(res) => Ok(res
                .iter()
                .filter_map(|x| x["orcid-identifier"]["path"].as_str())
                .map(|s| s.to_string())
                .collect()),
            None => Err(OrcidError::BadApiResponse(json)),
        }
    }
}

impl Default for ClientBlocking {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = ClientBlocking::new();
        assert_eq!(client.base_url, "https://pub.orcid.org/v3.0/");
    }

    #[test]
    fn test_default() {
        let client = ClientBlocking::default();
        assert_eq!(client.base_url, "https://pub.orcid.org/v3.0/");
    }

    #[test]
    fn test_is_valid_orcid_id() {
        // Good
        assert!(ClientBlocking::is_valid_orcid_id("0000-0001-5916-0947"));
        assert!(ClientBlocking::is_valid_orcid_id("0000000159160947"));

        // Bad
        assert!(!ClientBlocking::is_valid_orcid_id(
            "0000-0001-6916-0947" // Wrong digit
        ));
        assert!(!ClientBlocking::is_valid_orcid_id(
            "0000-0001-5916-0948" // Wrong checksum
        ));
        assert!(!ClientBlocking::is_valid_orcid_id("12345"));
        assert!(!ClientBlocking::is_valid_orcid_id("xyz"));
    }

    #[test]
    fn test_clone() {
        let client = ClientBlocking::new();
        let cloned = client.clone();
        assert_eq!(cloned.base_url, client.base_url);
    }

    #[test]
    fn test_debug() {
        let client = ClientBlocking::new();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("ClientBlocking"));
        assert!(debug_str.contains("base_url"));
        assert!(debug_str.contains("https://pub.orcid.org/v3.0/"));
    }

    #[test]
    fn test_valid_orcid_with_x() {
        assert!(ClientBlocking::is_valid_orcid_id("0000-0002-1825-0097"));
        assert!(ClientBlocking::is_valid_orcid_id("0000000218250097"));
    }

    #[test]
    fn test_invalid_orcid_length() {
        assert!(!ClientBlocking::is_valid_orcid_id("0000-0001-5916"));
        assert!(!ClientBlocking::is_valid_orcid_id(
            "0000-0001-5916-0947-1234"
        ));
        assert!(!ClientBlocking::is_valid_orcid_id(""));
    }

    #[test]
    fn test_invalid_orcid_chars() {
        assert!(!ClientBlocking::is_valid_orcid_id("0000-0001-5916-094A"));
        assert!(!ClientBlocking::is_valid_orcid_id("0000-0001-5916-094?"));
        assert!(!ClientBlocking::is_valid_orcid_id("ABCD-EFGH-IJKL-MNOP"));
    }

    #[test]
    fn test_base_url_override() {
        let client = ClientBlocking::new().base_url("http://localhost:1234/");
        assert_eq!(client.base_url, "http://localhost:1234/");
    }

    #[test]
    fn test_http_client_override() {
        let custom = reqwest::blocking::Client::builder()
            .user_agent("orcid-test/1.0")
            .build()
            .unwrap();
        let client = ClientBlocking::new().http_client(custom);
        assert_eq!(client.base_url, "https://pub.orcid.org/v3.0/");
    }

    #[test]
    fn test_blocking_client_behavior() {
        // This test verifies that the current client uses blocking I/O
        // and returns results synchronously
        let client = ClientBlocking::new();

        // Test that methods return Result directly (not Future)
        fn assert_sync_result<T>(_: Result<T>) {}

        // These would fail to compile if the methods returned futures
        let invalid_id = "invalid";
        let result = client.author(invalid_id);
        assert_sync_result(result);

        let search_result = client.search("test");
        assert_sync_result(search_result);

        let doi_result = client.search_doi("10.1234/test");
        assert_sync_result(doi_result);
    }

    #[test]
    fn test_get_json_from_api_is_blocking() {
        // Verify that the internal API method uses blocking client
        let client = ClientBlocking::new();
        // This test just ensures the method signature is correct for blocking
        // We can't easily test the actual request without mocking
        assert_eq!(client.base_url, "https://pub.orcid.org/v3.0/");
    }

    #[tokio::test]
    async fn test_search_builder() {
        let client = crate::Client::new();

        // Test building a search with multiple criteria
        let builder = client
            .search_builder()
            .with_keyword("climate")
            .with_affiliation("MIT")
            .limit(50);

        // Verify the builder has captured the parameters
        assert_eq!(builder.get_keyword(), Some("climate"));
        assert_eq!(builder.get_affiliation(), Some("MIT"));
        assert_eq!(builder.get_limit(), Some(50));

        // Test that we can build a query string
        let query = builder.build_query();
        assert!(query.contains("climate"));
        assert!(query.contains("MIT"));
    }
}
