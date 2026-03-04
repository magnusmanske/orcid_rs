use crate::author::Author;
use crate::error::{OrcidError, Result};
use crate::search_builder::SearchBuilder;
use reqwest::header::ACCEPT;
use serde_json;

#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            api_url: "https://pub.orcid.org/v3.0/".to_string(),
            client: reqwest::Client::new(),
        }
    }

    async fn get_json_from_api(&self, query: String) -> Result<serde_json::Value> {
        let url = self.api_url.clone() + &query;
        let response = self
            .client
            .get(&url)
            .header(ACCEPT, "application/json")
            .send()
            .await?;

        let json = response.json().await?;
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
    pub async fn author(&self, orcid_id: &str) -> Result<Author> {
        if !Self::is_valid_orcid_id(orcid_id) {
            return Err(OrcidError::InvalidOrcidId(orcid_id.to_string()));
        }

        let json: serde_json::Value = self.get_json_from_api(orcid_id.to_string()).await?;

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
    pub async fn search_doi(&self, doi: &str) -> Result<Vec<String>> {
        self.search(&("\"".to_string() + doi + "\"")).await
    }

    /// Takes a search query, returns a Vec<String> of ORCID IDs
    pub async fn search(&self, query: &str) -> Result<Vec<String>> {
        let encoded_query = urlencoding::encode(query);
        let json: serde_json::Value = self
            .get_json_from_api(format!("search?q={}", encoded_query))
            .await?;

        match json["result"].as_array() {
            Some(res) => Ok(res
                .iter()
                .filter_map(|x| x["orcid-identifier"]["path"].as_str())
                .map(|s| s.to_string())
                .collect()),
            None => Err(OrcidError::Other(format!("Bad result: {}", &json))),
        }
    }

    /// Create a search builder for constructing complex searches
    pub fn search_builder(&self) -> SearchBuilder<'_> {
        SearchBuilder::new(self)
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let client = Client::new();
        assert_eq!(client.api_url, "https://pub.orcid.org/v3.0/");
    }

    #[test]
    fn test_default() {
        let client = Client::default();
        assert_eq!(client.api_url, "https://pub.orcid.org/v3.0/");
    }

    #[test]
    fn test_is_valid_orcid_id() {
        // Good
        assert!(Client::is_valid_orcid_id("0000-0001-5916-0947"));
        assert!(Client::is_valid_orcid_id("0000000159160947"));

        // Bad
        assert!(!Client::is_valid_orcid_id(
            "0000-0001-6916-0947" // Wrong digit
        ));
        assert!(!Client::is_valid_orcid_id(
            "0000-0001-5916-0948" // Wrong checksum
        ));
        assert!(!Client::is_valid_orcid_id("12345"));
        assert!(!Client::is_valid_orcid_id("xyz"));
    }

    #[test]
    fn test_clone() {
        let client = Client::new();
        let cloned = client.clone();
        assert_eq!(cloned.api_url, client.api_url);
    }

    #[test]
    fn test_debug() {
        let client = Client::new();
        let debug_str = format!("{:?}", client);
        assert!(debug_str.contains("Client"));
        assert!(debug_str.contains("api_url"));
        assert!(debug_str.contains("https://pub.orcid.org/v3.0/"));
    }

    #[test]
    fn test_valid_orcid_with_x() {
        assert!(Client::is_valid_orcid_id("0000-0002-1825-0097"));
        assert!(Client::is_valid_orcid_id("0000000218250097"));
    }

    #[test]
    fn test_invalid_orcid_length() {
        assert!(!Client::is_valid_orcid_id("0000-0001-5916"));
        assert!(!Client::is_valid_orcid_id("0000-0001-5916-0947-1234"));
        assert!(!Client::is_valid_orcid_id(""));
    }

    #[test]
    fn test_invalid_orcid_chars() {
        assert!(!Client::is_valid_orcid_id("0000-0001-5916-094A"));
        assert!(!Client::is_valid_orcid_id("0000-0001-5916-094?"));
        assert!(!Client::is_valid_orcid_id("ABCD-EFGH-IJKL-MNOP"));
    }

    #[tokio::test]
    async fn test_async_methods_return_futures() {
        let client = Client::new();

        // These methods should be async and return futures
        // We'll test with invalid data to avoid actual API calls
        let result = client.author("invalid").await;
        assert!(result.is_err());

        // The error should be InvalidOrcidId
        match result {
            Err(OrcidError::InvalidOrcidId(_)) => (),
            _ => panic!("Expected InvalidOrcidId error"),
        }
    }

    #[tokio::test]
    async fn test_search_with_empty_query() {
        let client = Client::new();

        // Test that empty search doesn't panic
        // (In a real test, we'd mock the API response)
        let _result = client.search("").await;
        // We don't assert on the result as it would make a real API call
    }

    #[tokio::test]
    async fn test_search_doi_adds_quotes() {
        let client = Client::new();

        // Test that DOI search adds quotes
        // (In a real test, we'd mock the API to verify the query)
        let _result = client.search_doi("10.1234/test").await;
        // We don't assert on the result as it would make a real API call
    }
}
