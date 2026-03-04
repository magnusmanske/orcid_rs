use crate::date::Date;
use crate::funding::Funding;
use crate::membership::Membership;
use crate::organization::Organization;
use crate::peer_review::PeerReview;
use crate::role::Role;
use crate::utils::collect_parts;
use crate::work::Work;
use serde_json;

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
            (Some(f), Some(l)) => Some(format!("{} {}", f, l)),
            (None, Some(l)) => Some(l.to_string()),
            _ => None,
        }
    }

    pub fn other_names(&self) -> Vec<&str> {
        self.j["person"]["other-names"]["other-name"]
            .as_array()
            .map(|x| x.iter().filter_map(|x| x["content"].as_str()).collect())
            .unwrap_or_default()
    }

    pub fn biography(&self) -> Option<&str> {
        self.j["person"]["biography"]["content"].as_str()
    }

    pub fn external_ids(&self) -> Vec<(String, String)> {
        collect_parts(
            &self.j["person"]["external-identifiers"]["external-identifier"],
            vec!["external-id-type", "external-id-value"],
        )
        .into_iter()
        .map(|v| (v[0].clone(), v[1].clone()))
        .collect()
    }

    pub fn keywords(&self) -> Vec<String> {
        collect_parts(&self.j["person"]["keywords"]["keyword"], vec!["content"])
            .into_iter()
            .map(|v| v.into_iter().next().unwrap_or_default())
            .collect()
    }

    pub fn works(&self) -> Vec<Work> {
        self.j["activities-summary"]["works"]["group"]
            .as_array()
            .map(|arr| arr.iter().map(Work::new_from_json).collect())
            .unwrap_or_default()
    }

    pub fn researcher_urls(&self) -> Vec<(&str, &str)> {
        self.j["person"]["researcher-urls"]["researcher-url"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(
                        |x| match (x["url-name"].as_str(), x["url"]["value"].as_str()) {
                            (Some(name), Some(value)) => Some((name, value)),
                            _ => None,
                        },
                    )
                    .collect()
            })
            .unwrap_or_default()
    }

    fn roles(&self, key1: &str, key2: &str) -> Vec<Role> {
        self.j["activities-summary"][key1]["affiliation-group"]
            .as_array()
            .map(|groups| {
                let mut ret = Vec::new();
                for group in groups {
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
            })
            .unwrap_or_default()
    }

    pub fn education(&self) -> Vec<Role> {
        self.roles("educations", "education-summary")
    }

    pub fn employment(&self) -> Vec<Role> {
        self.roles("employments", "employment-summary")
    }

    pub fn fundings(&self) -> Vec<Funding> {
        self.j["activities-summary"]["fundings"]["group"]
            .as_array()
            .map(|arr| arr.iter().map(Funding::new_from_json).collect())
            .unwrap_or_default()
    }

    pub fn memberships(&self) -> Vec<Membership> {
        self.j["activities-summary"]["memberships"]["group"]
            .as_array()
            .map(|arr| arr.iter().map(Membership::new_from_json).collect())
            .unwrap_or_default()
    }

    pub fn peer_reviews(&self) -> Vec<PeerReview> {
        self.j["activities-summary"]["peer-reviews"]["group"]
            .as_array()
            .map(|arr| arr.iter().map(PeerReview::new_from_json).collect())
            .unwrap_or_default()
    }

    // TODO name and name variants
    // invited-positions
    // qualifications
    // research-resources?
    // services?
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_author_comprehensive_access_patterns() {
        // This test ensures all accessor methods work correctly with current JSON storage
        // This will help ensure lazy parsing maintains the same behavior
        let j = json!({
            "orcid-identifier": {
                "path": "0000-0001-5916-0947"
            },
            "person": {
                "name": {
                    "given-names": { "value": "John" },
                    "family-name": { "value": "Doe" },
                    "credit-name": { "value": "J. Doe" }
                },
                "other-names": {
                    "other-name": [
                        { "content": "Johnny" },
                        { "content": "JD" }
                    ]
                },
                "biography": {
                    "content": "A test biography"
                },
                "researcher-urls": {
                    "researcher-url": [
                        {
                            "url-name": "Website",
                            "url": { "value": "https://example.com" }
                        }
                    ]
                },
                "external-identifiers": {
                    "external-identifier": [
                        {
                            "external-id-type": "ISNI",
                            "external-id-value": "0000000012345678"
                        }
                    ]
                },
                "keywords": {
                    "keyword": [
                        { "content": "testing" },
                        { "content": "rust" }
                    ]
                }
            },
            "activities-summary": {
                "works": {
                    "group": [{
                        "work-summary": [{
                            "title": {
                                "title": {
                                    "value": "Test Work"
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
                },
                "educations": {
                    "affiliation-group": [{
                        "summaries": [{
                            "education-summary": {
                                "department-name": "CS",
                                "role-title": "PhD",
                                "organization": {
                                    "name": "Test U"
                                }
                            }
                        }]
                    }]
                },
                "employments": {
                    "affiliation-group": [{
                        "summaries": [{
                            "employment-summary": {
                                "department-name": "Engineering",
                                "role-title": "Engineer",
                                "organization": {
                                    "name": "Test Corp"
                                }
                            }
                        }]
                    }]
                },
                "fundings": {
                    "group": [{
                        "funding-summary": [{
                            "title": {
                                "title": {
                                    "value": "Test Grant"
                                }
                            }
                        }]
                    }]
                }
            }
        });

        let author = Author::new_from_json(j.clone());

        // Test all accessor methods work
        assert_eq!(author.orcid_id(), Some("0000-0001-5916-0947"));
        assert_eq!(author.credit_name(), Some("J. Doe"));
        assert_eq!(author.full_name(), Some("John Doe".to_string()));

        let other_names = author.other_names();
        assert_eq!(other_names.len(), 2);
        assert_eq!(other_names[0], "Johnny");

        assert_eq!(author.biography(), Some("A test biography"));

        let urls = author.researcher_urls();
        assert_eq!(urls.len(), 1);
        assert_eq!(urls[0], ("Website", "https://example.com"));

        let ext_ids = author.external_ids();
        assert_eq!(ext_ids.len(), 1);
        assert_eq!(
            ext_ids[0],
            ("ISNI".to_string(), "0000000012345678".to_string())
        );

        let keywords = author.keywords();
        assert_eq!(keywords.len(), 2);
        assert_eq!(keywords[0], "testing");

        let works = author.works();
        assert_eq!(works.len(), 1);
        assert_eq!(works[0].title, Some("Test Work".to_string()));

        let education = author.education();
        assert_eq!(education.len(), 1);
        assert_eq!(education[0].department().map(|s| s.as_str()), Some("CS"));

        let employment = author.employment();
        assert_eq!(employment.len(), 1);
        assert_eq!(employment[0].title().map(|s| s.as_str()), Some("Engineer"));

        let fundings = author.fundings();
        assert_eq!(fundings.len(), 1);
        assert_eq!(fundings[0].title().map(|s| s.as_str()), Some("Test Grant"));

        // Ensure json() method returns the original JSON
        assert_eq!(author.json(), &j);
    }

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

    #[test]
    fn test_memberships() {
        let j = json!({
            "activities-summary": {
                "memberships": {
                    "group": [{
                        "membership-summary": [{
                            "organization": {
                                "name": "ACM"
                            },
                            "department-name": "Computer Science",
                            "role-title": "Senior Member",
                            "start-date": {
                                "year": { "value": 2018 }
                            },
                            "external-ids": {
                                "external-id": [{
                                    "external-id-type": "membership-id",
                                    "external-id-value": "ACM-123456"
                                }]
                            }
                        }]
                    }]
                }
            }
        });

        let author = Author::new_from_json(j);
        let memberships = author.memberships();
        assert_eq!(memberships.len(), 1);
        assert_eq!(
            memberships[0].role_title().map(|s| s.as_str()),
            Some("Senior Member")
        );
        assert_eq!(
            memberships[0].department_name().map(|s| s.as_str()),
            Some("Computer Science")
        );
        assert!(memberships[0].organization().is_some());
        assert_eq!(memberships[0].external_ids().len(), 1);
    }
}
