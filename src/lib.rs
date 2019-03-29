extern crate reqwest;

use reqwest::header::ACCEPT;
use serde_json;

fn collect_parts(j: &serde_json::Value, parts: Vec<&str>) -> Vec<Vec<String>> {
    j.as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|v| {
            parts
                .iter()
                .map(|part| v[part].as_str().unwrap_or("").to_string())
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct PublicationDate {
    year: Option<u32>,
    month: Option<u8>,
    day: Option<u8>,
}

impl PublicationDate {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        Self {
            year: match j["year"]["value"].as_str().map(|v| v.to_string()) {
                Some(v) => v.parse::<u32>().ok(),
                None => None,
            },
            month: match j["month"]["value"].as_str().map(|v| v.to_string()) {
                Some(v) => v.parse::<u8>().ok(),
                None => None,
            },
            day: match j["day"]["value"].as_str().map(|v| v.to_string()) {
                Some(v) => v.parse::<u8>().ok(),
                None => None,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct Work {
    pub title: Option<String>,
    pub external_ids: Vec<(String, String)>,
    pub publication_date: PublicationDate,
    pub pub_type: Option<String>,
}

impl Work {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        Self {
            title: j["work-summary"][0]["title"]["title"]["value"]
                .as_str()
                .map(|v| v.to_string()),
            external_ids: collect_parts(
                &j["external-ids"]["external-id"],
                vec!["external-id-type", "external-id-value"],
            )
            .iter()
            .map(|v| (v[0].to_owned(), v[1].to_owned()))
            .collect(),
            pub_type: j["work-summary"][0]["type"].as_str().map(|v| v.to_string()),
            publication_date: PublicationDate::new_from_json(
                &j["work-summary"][0]["publication-date"],
            ),
        }
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
        collect_parts(
            &self.j["person"]["external-identifiers"]["external-identifier"],
            vec!["external-id-type", "external-id-value"],
        )
        .iter()
        .map(|v| (v[0].to_owned(), v[1].to_owned()))
        .collect()
    }

    pub fn keywords(&self) -> Vec<String> {
        collect_parts(&self.j["person"]["keywords"]["keyword"], vec!["content"])
            .iter()
            .map(|v| v[0].to_owned())
            .collect()
    }

    pub fn works(&self) -> Vec<Work> {
        self.j["activities-summary"]["works"]["group"]
            .as_array()
            .unwrap_or(&vec![])
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
