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
pub struct Date {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        Self {
            year: j["year"]["value"]
                .as_str()
                .map(|x| x.parse::<u16>().unwrap()),
            month: j["month"]["value"]
                .as_str()
                .map(|x| x.parse::<u8>().unwrap()),
            day: j["day"]["value"].as_str().map(|x| x.parse::<u8>().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Organization {
    name: Option<String>,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    // TODO external IDS
    // TODO disambiguated-organization
}

impl Organization {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        Self {
            name: j["name"].as_str().map(|s| s.to_string()),
            city: j["address"]["city"].as_str().map(|s| s.to_string()),
            region: j["address"]["region"].as_str().map(|s| s.to_string()),
            country: j["address"]["country"].as_str().map(|s| s.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Role {
    department: Option<String>,
    title: Option<String>,
    start_date: Option<Date>,
    end_date: Option<Date>,
    organization: Option<Organization>,
}

impl Role {
    pub fn new() -> Self {
        Self {
            department: None,
            title: None,
            start_date: None,
            end_date: None,
            organization: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Author {
    j: serde_json::Value,
}

impl Author {
    pub fn new_from_json(j: serde_json::Value) -> Self {
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

    pub fn full_name(&self) -> Option<String> {
        let last_name = self.j["person"]["name"]["family-name"]["value"].as_str();
        let given_names = self.j["person"]["name"]["given-names"]["value"].as_str();
        match (given_names, last_name) {
            (Some(f), Some(l)) => Some(format!("{} {}", &f, &l)),
            (None, Some(l)) => Some(format!("{}", &l)),
            _ => None,
        }
    }

    pub fn other_names(&self) -> Vec<&str> {
        match self.j["person"]["other-names"]["other-name"].as_array() {
            Some(x) => x
                .iter()
                .filter(|x| x["content"].is_string())
                .map(|x| x["content"].as_str().unwrap())
                .collect(),
            None => vec![],
        }
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

    pub fn researcher_urls(&self) -> Vec<(&str, &str)> {
        match self.j["person"]["researcher-urls"]["researcher-url"].as_array() {
            Some(x) => x
                .iter()
                .filter(|x| x["url-name"].is_string())
                .filter(|x| x["url"]["value"].is_string())
                .map(|x| {
                    (
                        x["url-name"].as_str().unwrap(),
                        x["url"]["value"].as_str().unwrap(),
                    )
                })
                .collect(),
            None => vec![],
        }
    }

    pub fn education(&self) -> Vec<Role> {
        match self.j["activities-summary"]["educations"]["affiliation-group"].as_array() {
            Some(groups) => {
                let mut ret = vec![];
                for group in groups {
                    // TODO external-ids
                    match group["summaries"].as_array() {
                        Some(summaries) => {
                            for summary in summaries {
                                println!("{}", &summary);
                                if !summary["education-summary"].is_object() {
                                    continue;
                                }
                                let mut role = Role::new();
                                role.department = summary["education-summary"]["department-name"]
                                    .as_str()
                                    .map(|s| s.to_string());
                                role.title = summary["education-summary"]["role-title"]
                                    .as_str()
                                    .map(|s| s.to_string());
                                match summary["education-summary"]["start-date"].is_object() {
                                    true => {
                                        role.start_date = Some(Date::new_from_json(
                                            &summary["education-summary"]["start-date"],
                                        ))
                                    }
                                    false => {}
                                }
                                match summary["education-summary"]["end-date"].is_object() {
                                    true => {
                                        role.end_date = Some(Date::new_from_json(
                                            &summary["education-summary"]["end-date"],
                                        ))
                                    }
                                    false => {}
                                }
                                match summary["education-summary"]["organization"].is_object() {
                                    true => {
                                        role.organization = Some(Organization::new_from_json(
                                            &summary["education-summary"]["organization"],
                                        ))
                                    }
                                    false => {}
                                }
                                ret.push(role);
                            }
                        }
                        None => {}
                    }
                }
                ret
            }
            None => vec![],
        }
    }

    // TODO name and name variants
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
            api_url: "https://pub.orcid.org/v3.0/".to_string(),
        }
    }

    fn get_json_from_api(&self, query: String) -> Result<serde_json::Value, reqwest::Error> {
        let url = self.api_url.clone() + &query;
        //println!("{}", &url);
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
