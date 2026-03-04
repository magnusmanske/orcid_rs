use crate::author::Author;
use anyhow::{anyhow, Result};
use reqwest::header::ACCEPT;
use serde_json;

#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            api_url: "https://pub.orcid.org/v3.0/".to_string(),
        }
    }

    fn get_json_from_api(&self, query: String) -> Result<serde_json::Value, reqwest::Error> {
        let url = self.api_url.clone() + &query;
        //println!("{}", &url);
        reqwest::blocking::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()?
            .json()
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
            return Err(anyhow!("{} is not a valid ORCID ID", orcid_id));
        }

        let json: serde_json::Value = self.get_json_from_api(orcid_id.to_string())?;

        match json["error-code"].as_str() {
            Some(_) => Err(anyhow!(
                "{}:{}",
                orcid_id,
                json["developer-message"]
                    .as_str()
                    .unwrap_or("no developer-message")
            )),
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
            None => Err(anyhow!("Bad result: {}", &json)),
        }
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
}
