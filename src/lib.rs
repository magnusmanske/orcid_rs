extern crate reqwest;

use reqwest::header::ACCEPT;
use serde_json;

#[derive(Debug, Clone)]
pub struct Work {
    pub external_ids: Vec<(String, String)>,
}

impl Work {
    pub fn new_from_json(j: &serde_json::Value) -> Work {
        let mut ret = Work {
            external_ids: vec![],
        };

        ret.external_ids = j["external-ids"]["external-id"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    v["external-id-type"].as_str().unwrap().to_string(),
                    v["external-id-value"].as_str().unwrap().to_string(),
                )
            })
            .collect();

        ret
    }
}

#[derive(Debug, Clone)]
pub struct Author {
    j: serde_json::Value,
}

impl Author {
    pub fn new_from_json(j: serde_json::Value) -> Author {
        Author { j: j }
    }

    pub fn json(&self) -> &serde_json::Value {
        &self.j
    }

    pub fn orcid_id(&self) -> Option<&str> {
        self.j["orcid-identifier"]["path"].as_str()
    }

    pub fn credit_name(&self) -> Option<&str> {
        self.j["person"]["name"]["credit-name"]["value"].as_str()
    }

    pub fn external_ids(&self) -> Vec<(String, String)> {
        self.j["person"]["external-identifiers"]["external-identifier"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| {
                (
                    v["external-id-type"].as_str().unwrap().to_string(),
                    v["external-id-value"].as_str().unwrap().to_string(),
                )
            })
            .collect()
    }

    pub fn keywords(&self) -> Vec<String> {
        self.j["person"]["keywords"]["keyword"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v["content"].as_str().unwrap().to_string())
            .collect()
    }

    pub fn works(&self) -> Vec<Work> {
        self.j["person"]["activities"]["works"]["group"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| Work::new_from_json(&v))
            .collect()
    }

    // TODO name and name variants
    // Homepage
    // activities: education, employments
    // fundings
    // peer-reviews
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
            None => Ok(Author::new_from_json(json)),
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
