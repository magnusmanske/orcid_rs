extern crate reqwest;

use reqwest::header::ACCEPT;
use serde_json;

#[derive(Debug, Clone)]
pub struct Author {
    j: serde_json::Value,
}

impl Author {
    pub fn new(j: serde_json::Value) -> Author {
        Author { j: j }
    }

    pub fn orcid_id(&self) -> Option<&str> {
        self.j["orcid-identifier"]["path"].as_str()
    }

    pub fn credit_name(&self) -> Option<&str> {
        self.j["person"]["name"]["credit-name"]["value"].as_str()
    }
}

#[derive(Debug, Clone)]
pub struct Client {
    api_url: String,
}

impl Client {
    pub fn new() -> Client {
        Client {
            api_url: "https://pub.orcid.org/v2.1/".to_string(),
        }
    }

    fn get_json_from_api(&self, query: String) -> Result<serde_json::Value, reqwest::Error> {
        let url = self.api_url.clone() + &query;
        println!("{}", &url);
        reqwest::Client::new()
            .get(url.as_str())
            .header(ACCEPT, "application/json")
            .send()?
            .json()
    }

    /// Returns an `Author` for a given ORCID ID
    pub fn author(&self, orcid_id: &String) -> Result<Author, Box<::std::error::Error>> {
        // TODO validate ORCID ID
        let json: serde_json::Value = self.get_json_from_api(orcid_id.to_string())?;

        match json["error-code"].as_str() {
            Some(_) => Err(From::from(format!(
                "{}:{}",
                orcid_id,
                json["developer-message"].as_str().unwrap()
            ))),
            None => Ok(Author::new(json)),
        }
    }

    /// Takes a DOI, quotes and searches it, returns a Vec<String> of ORCID IDs
    pub fn search_doi(&self, doi: &String) -> Result<Vec<String>, Box<::std::error::Error>> {
        self.search(&("\"".to_string() + doi + "\""))
    }

    /// Takes a search query, returns a Vec<String> of ORCID IDs
    pub fn search(&self, query: &String) -> Result<Vec<String>, Box<::std::error::Error>> {
        // TODO urlencode search query
        let json: serde_json::Value = self.get_json_from_api("search?q=".to_string() + query)?;
        let ret = json["result"]
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| x["orcid-identifier"]["path"].as_str().unwrap().to_string())
            .collect();
        Ok(ret)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let _client = Client::new();
        assert_eq!(2 + 2, 4);
    }
}
