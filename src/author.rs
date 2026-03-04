use crate::date::Date;
use crate::organization::Organization;
use crate::work::Work;
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

use crate::role::Role;

#[derive(Debug, Clone)]
pub struct Author {
    j: serde_json::Value,
}

impl Author {
    pub fn new_from_json(j: serde_json::Value) -> Self {
        Author { j }
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
            (None, Some(l)) => Some((&l).to_string()),
            _ => None,
        }
    }

    pub fn other_names(&self) -> Vec<&str> {
        match self.j["person"]["other-names"]["other-name"].as_array() {
            Some(x) => x
                .iter()
                .filter(|x| x["content"].is_string())
                .filter_map(|x| x["content"].as_str())
                .collect(),
            None => vec![],
        }
    }

    pub fn biography(&self) -> Option<&str> {
        self.j["person"]["biography"]["content"].as_str()
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
            .map(Work::new_from_json)
            .collect()
    }

    pub fn researcher_urls(&self) -> Vec<(&str, &str)> {
        match self.j["person"]["researcher-urls"]["researcher-url"].as_array() {
            Some(x) => x
                .iter()
                .filter(|x| x["url-name"].is_string())
                .filter(|x| x["url"]["value"].is_string())
                .filter_map(
                    |x| match (x["url-name"].as_str(), x["url"]["value"].as_str()) {
                        (Some(name), Some(value)) => Some((name, value)),
                        _ => None,
                    },
                )
                .collect(),
            None => vec![],
        }
    }

    fn roles(&self, key1: &str, key2: &str) -> Vec<Role> {
        match self.j["activities-summary"][key1]["affiliation-group"].as_array() {
            Some(groups) => {
                let mut ret = vec![];
                for group in groups {
                    // TODO external-ids
                    if let Some(summaries) = group["summaries"].as_array() {
                        for summary in summaries {
                            if !summary[key2].is_object() {
                                continue;
                            }
                            let x2 = &summary[key2];
                            let mut role = Role::new();
                            role.set_department(
                                x2["department-name"].as_str().map(|s| s.to_string()),
                            );
                            role.set_title(x2["role-title"].as_str().map(|s| s.to_string()));
                            if x2["start-date"].is_object() {
                                role.set_start_date(Some(Date::new_from_json(&x2["start-date"])))
                            }
                            if x2["end-date"].is_object() {
                                role.set_end_date(Some(Date::new_from_json(&x2["end-date"])))
                            }
                            if x2["organization"].is_object() {
                                role.set_organization(Some(Organization::new_from_json(
                                    &x2["organization"],
                                )))
                            }
                            ret.push(role);
                        }
                    }
                }
                ret
            }
            None => vec![],
        }
    }

    pub fn education(&self) -> Vec<Role> {
        self.roles("educations", "education-summary")
    }

    pub fn employment(&self) -> Vec<Role> {
        self.roles("employments", "employment-summary")
    }

    // TODO name and name variants
    // fundings
    // invited-positions
    // memberships
    // peer-reviews
    // qualifications
    // research-resources?
    // services?
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json() {
        let j = json!({
            "orcid-identifier": {
                "path": "0000-0001-5916-0947"
            }
        });

        let author = Author::new_from_json(j.clone());
        assert_eq!(author.json(), &j);
    }

    #[test]
    fn test_orcid_id() {
        let j = json!({
            "orcid-identifier": {
                "path": "0000-0001-5916-0947"
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(author.orcid_id(), Some("0000-0001-5916-0947"));
    }

    #[test]
    fn test_credit_name() {
        let j = json!({
            "person": {
                "name": {
                    "credit-name": {
                        "value": "Dr. John Doe"
                    }
                }
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(author.credit_name(), Some("Dr. John Doe"));
    }

    #[test]
    fn test_full_name() {
        let j = json!({
            "person": {
                "name": {
                    "given-names": { "value": "John" },
                    "family-name": { "value": "Doe" }
                }
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(author.full_name(), Some("John Doe".to_string()));
    }

    #[test]
    fn test_full_name_only_last() {
        let j = json!({
            "person": {
                "name": {
                    "family-name": { "value": "Doe" }
                }
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(author.full_name(), Some("Doe".to_string()));
    }

    #[test]
    fn test_full_name_none() {
        let j = json!({
            "person": {
                "name": {}
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(author.full_name(), None);
    }

    #[test]
    fn test_other_names() {
        let j = json!({
            "person": {
                "other-names": {
                    "other-name": [
                        { "content": "Johnny" },
                        { "content": "J. Doe" }
                    ]
                }
            }
        });

        let author = Author::new_from_json(j);
        let other_names = author.other_names();
        assert_eq!(other_names.len(), 2);
        assert_eq!(other_names[0], "Johnny");
        assert_eq!(other_names[1], "J. Doe");
    }

    #[test]
    fn test_biography() {
        let j = json!({
            "person": {
                "biography": {
                    "content": "A researcher in computer science."
                }
            }
        });

        let author = Author::new_from_json(j);
        assert_eq!(
            author.biography(),
            Some("A researcher in computer science.")
        );
    }

    #[test]
    fn test_external_ids() {
        let j = json!({
            "person": {
                "external-identifiers": {
                    "external-identifier": [
                        {
                            "external-id-type": "ResearcherID",
                            "external-id-value": "A-1234-5678"
                        },
                        {
                            "external-id-type": "Scopus",
                            "external-id-value": "1234567890"
                        }
                    ]
                }
            }
        });

        let author = Author::new_from_json(j);
        let ids = author.external_ids();
        assert_eq!(ids.len(), 2);
        assert_eq!(
            ids[0],
            ("ResearcherID".to_string(), "A-1234-5678".to_string())
        );
        assert_eq!(ids[1], ("Scopus".to_string(), "1234567890".to_string()));
    }

    #[test]
    fn test_keywords() {
        let j = json!({
            "person": {
                "keywords": {
                    "keyword": [
                        { "content": "computer science" },
                        { "content": "machine learning" }
                    ]
                }
            }
        });

        let author = Author::new_from_json(j);
        let keywords = author.keywords();
        assert_eq!(keywords.len(), 2);
        assert_eq!(keywords[0], "computer science");
        assert_eq!(keywords[1], "machine learning");
    }

    #[test]
    fn test_researcher_urls() {
        let j = json!({
            "person": {
                "researcher-urls": {
                    "researcher-url": [
                        {
                            "url-name": "Personal Website",
                            "url": { "value": "https://example.com" }
                        },
                        {
                            "url-name": "Blog",
                            "url": { "value": "https://blog.example.com" }
                        }
                    ]
                }
            }
        });

        let author = Author::new_from_json(j);
        let urls = author.researcher_urls();
        assert_eq!(urls.len(), 2);
        assert_eq!(urls[0], ("Personal Website", "https://example.com"));
        assert_eq!(urls[1], ("Blog", "https://blog.example.com"));
    }

    #[test]
    fn test_works() {
        let j = json!({
            "activities-summary": {
                "works": {
                    "group": [{
                        "work-summary": [{
                            "title": {
                                "title": {
                                    "value": "Test Publication"
                                }
                            },
                            "type": "journal-article",
                            "publication-date": {
                                "year": { "value": "2023" }
                            }
                        }],
                        "external-ids": {
                            "external-id": []
                        }
                    }]
                }
            }
        });

        let author = Author::new_from_json(j);
        let works = author.works();
        assert_eq!(works.len(), 1);
        assert_eq!(works[0].title, Some("Test Publication".to_string()));
    }

    #[test]
    fn test_education() {
        let j = json!({
            "activities-summary": {
                "educations": {
                    "affiliation-group": [{
                        "summaries": [{
                            "education-summary": {
                                "department-name": "Computer Science",
                                "role-title": "PhD",
                                "start-date": {
                                    "year": { "value": 2015 }
                                },
                                "organization": {
                                    "name": "Test University"
                                }
                            }
                        }]
                    }]
                }
            }
        });

        let author = Author::new_from_json(j);
        let education = author.education();
        assert_eq!(education.len(), 1);
        assert_eq!(
            education[0].department().map(|s| s.to_string()),
            Some("Computer Science".to_string())
        );
        assert_eq!(
            education[0].title().map(|s| s.to_string()),
            Some("PhD".to_string())
        );
    }

    #[test]
    fn test_employment() {
        let j = json!({
            "activities-summary": {
                "employments": {
                    "affiliation-group": [{
                        "summaries": [{
                            "employment-summary": {
                                "department-name": "Engineering",
                                "role-title": "Professor",
                                "start-date": {
                                    "year": { "value": 2020 }
                                },
                                "organization": {
                                    "name": "Test University"
                                }
                            }
                        }]
                    }]
                }
            }
        });

        let author = Author::new_from_json(j);
        let employment = author.employment();
        assert_eq!(employment.len(), 1);
        assert_eq!(
            employment[0].department().map(|s| s.to_string()),
            Some("Engineering".to_string())
        );
        assert_eq!(
            employment[0].title().map(|s| s.to_string()),
            Some("Professor".to_string())
        );
    }
}
