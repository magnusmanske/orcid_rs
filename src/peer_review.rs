use crate::date::Date;
use crate::organization::Organization;
use serde_json;

#[derive(Debug, Clone)]
pub struct PeerReview {
    organization: Option<Organization>,
    review_type: Option<String>,
    review_role: Option<String>,
    review_url: Option<String>,
    review_completion_date: Option<Date>,
    review_group_id: Option<String>,
    subject_external_identifier: Option<(String, String)>,
    subject_type: Option<String>,
    subject_name: Option<String>,
    subject_url: Option<String>,
    external_ids: Vec<(String, String)>,
}

impl PeerReview {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        let peer_review_summary = &j["peer-review-summary"][0];

        let external_ids =
            if let Some(ext_ids) = peer_review_summary["external-ids"]["external-id"].as_array() {
                ext_ids
                    .iter()
                    .filter_map(|id| {
                        match (
                            id["external-id-type"].as_str(),
                            id["external-id-value"].as_str(),
                        ) {
                            (Some(id_type), Some(id_value)) => {
                                Some((id_type.to_string(), id_value.to_string()))
                            }
                            _ => None,
                        }
                    })
                    .collect()
            } else {
                vec![]
            };

        let subject_external_identifier =
            match peer_review_summary["subject-external-identifier"].as_object() {
                Some(obj) => {
                    match (
                        obj.get("external-id-type").and_then(|v| v.as_str()),
                        obj.get("external-id-value").and_then(|v| v.as_str()),
                    ) {
                        (Some(id_type), Some(id_value)) => {
                            Some((id_type.to_string(), id_value.to_string()))
                        }
                        _ => None,
                    }
                }
                None => None,
            };

        Self {
            organization: if peer_review_summary["organization"].is_object() {
                Some(Organization::new_from_json(
                    &peer_review_summary["organization"],
                ))
            } else {
                None
            },
            review_type: peer_review_summary["review-type"]
                .as_str()
                .map(|s| s.to_string()),
            review_role: peer_review_summary["role"].as_str().map(|s| s.to_string()),
            review_url: peer_review_summary["review-url"]["value"]
                .as_str()
                .map(|s| s.to_string()),
            review_completion_date: if peer_review_summary["completion-date"]["year"].is_object() {
                Some(Date::new_from_json(&peer_review_summary["completion-date"]))
            } else {
                None
            },
            review_group_id: peer_review_summary["review-group-id"]
                .as_str()
                .map(|s| s.to_string()),
            subject_external_identifier,
            subject_type: peer_review_summary["subject-type"]
                .as_str()
                .map(|s| s.to_string()),
            subject_name: peer_review_summary["subject-name"]["title"]["value"]
                .as_str()
                .map(|s| s.to_string()),
            subject_url: peer_review_summary["subject-url"]["value"]
                .as_str()
                .map(|s| s.to_string()),
            external_ids,
        }
    }

    // Getter methods
    pub fn organization(&self) -> Option<&Organization> {
        self.organization.as_ref()
    }

    pub fn review_type(&self) -> Option<&String> {
        self.review_type.as_ref()
    }

    pub fn review_role(&self) -> Option<&String> {
        self.review_role.as_ref()
    }

    pub fn review_url(&self) -> Option<&String> {
        self.review_url.as_ref()
    }

    pub fn review_completion_date(&self) -> Option<&Date> {
        self.review_completion_date.as_ref()
    }

    pub fn review_group_id(&self) -> Option<&String> {
        self.review_group_id.as_ref()
    }

    pub fn subject_external_identifier(&self) -> Option<&(String, String)> {
        self.subject_external_identifier.as_ref()
    }

    pub fn subject_type(&self) -> Option<&String> {
        self.subject_type.as_ref()
    }

    pub fn subject_name(&self) -> Option<&String> {
        self.subject_name.as_ref()
    }

    pub fn subject_url(&self) -> Option<&String> {
        self.subject_url.as_ref()
    }

    pub fn external_ids(&self) -> &Vec<(String, String)> {
        &self.external_ids
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json_complete() {
        let j = json!({
            "peer-review-summary": [{
                "organization": {
                    "name": "Nature Publishing Group",
                    "address": {
                        "city": "London",
                        "country": "GB"
                    }
                },
                "review-type": "review",
                "role": "reviewer",
                "review-url": {
                    "value": "https://publons.com/review/12345"
                },
                "completion-date": {
                    "year": { "value": 2023 },
                    "month": { "value": 3 },
                    "day": { "value": 15 }
                },
                "review-group-id": "issn:0028-0836",
                "subject-external-identifier": {
                    "external-id-type": "doi",
                    "external-id-value": "10.1038/nature12345"
                },
                "subject-type": "journal-article",
                "subject-name": {
                    "title": {
                        "value": "A groundbreaking discovery in quantum physics"
                    }
                },
                "subject-url": {
                    "value": "https://doi.org/10.1038/nature12345"
                },
                "external-ids": {
                    "external-id": [{
                        "external-id-type": "peer-review",
                        "external-id-value": "PR-2023-12345"
                    }]
                }
            }]
        });

        let review = PeerReview::new_from_json(&j);

        assert!(review.organization().is_some());
        assert_eq!(
            review.organization().unwrap().name(),
            Some(&"Nature Publishing Group".to_string())
        );
        assert_eq!(review.review_type(), Some(&"review".to_string()));
        assert_eq!(review.review_role(), Some(&"reviewer".to_string()));
        assert_eq!(
            review.review_url(),
            Some(&"https://publons.com/review/12345".to_string())
        );
        assert!(review.review_completion_date().is_some());
        assert_eq!(
            review.review_group_id(),
            Some(&"issn:0028-0836".to_string())
        );
        assert_eq!(
            review.subject_external_identifier(),
            Some(&("doi".to_string(), "10.1038/nature12345".to_string()))
        );
        assert_eq!(review.subject_type(), Some(&"journal-article".to_string()));
        assert_eq!(
            review.subject_name(),
            Some(&"A groundbreaking discovery in quantum physics".to_string())
        );
        assert_eq!(
            review.subject_url(),
            Some(&"https://doi.org/10.1038/nature12345".to_string())
        );
        assert_eq!(review.external_ids().len(), 1);
        assert_eq!(
            review.external_ids()[0],
            ("peer-review".to_string(), "PR-2023-12345".to_string())
        );
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "peer-review-summary": [{
                "organization": {
                    "name": "Test Journal"
                },
                "review-type": "review"
            }]
        });

        let review = PeerReview::new_from_json(&j);

        assert!(review.organization().is_some());
        assert_eq!(review.review_type(), Some(&"review".to_string()));
        assert_eq!(review.review_role(), None);
        assert_eq!(review.review_url(), None);
        assert!(review.review_completion_date().is_none());
        assert_eq!(review.review_group_id(), None);
        assert_eq!(review.subject_external_identifier(), None);
        assert_eq!(review.subject_type(), None);
        assert_eq!(review.subject_name(), None);
        assert_eq!(review.subject_url(), None);
        assert_eq!(review.external_ids().len(), 0);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({
            "peer-review-summary": [{}]
        });

        let review = PeerReview::new_from_json(&j);

        assert!(review.organization().is_none());
        assert_eq!(review.review_type(), None);
        assert_eq!(review.review_role(), None);
        assert_eq!(review.review_url(), None);
        assert!(review.review_completion_date().is_none());
        assert_eq!(review.review_group_id(), None);
        assert_eq!(review.subject_external_identifier(), None);
        assert_eq!(review.subject_type(), None);
        assert_eq!(review.subject_name(), None);
        assert_eq!(review.subject_url(), None);
        assert_eq!(review.external_ids().len(), 0);
    }

    #[test]
    fn test_new_from_json_multiple_external_ids() {
        let j = json!({
            "peer-review-summary": [{
                "external-ids": {
                    "external-id": [
                        {
                            "external-id-type": "peer-review",
                            "external-id-value": "PR-123"
                        },
                        {
                            "external-id-type": "publons",
                            "external-id-value": "PUB-456"
                        }
                    ]
                }
            }]
        });

        let review = PeerReview::new_from_json(&j);

        assert_eq!(review.external_ids().len(), 2);
        assert_eq!(
            review.external_ids()[0],
            ("peer-review".to_string(), "PR-123".to_string())
        );
        assert_eq!(
            review.external_ids()[1],
            ("publons".to_string(), "PUB-456".to_string())
        );
    }

    #[test]
    fn test_debug_trait() {
        let review = PeerReview {
            organization: None,
            review_type: Some("review".to_string()),
            review_role: Some("reviewer".to_string()),
            review_url: None,
            review_completion_date: None,
            review_group_id: None,
            subject_external_identifier: None,
            subject_type: None,
            subject_name: None,
            subject_url: None,
            external_ids: vec![],
        };

        let debug_str = format!("{:?}", review);
        assert!(debug_str.contains("PeerReview"));
        assert!(debug_str.contains("review"));
        assert!(debug_str.contains("reviewer"));
    }

    #[test]
    fn test_clone_trait() {
        let review = PeerReview {
            organization: None,
            review_type: Some("review".to_string()),
            review_role: Some("reviewer".to_string()),
            review_url: Some("https://example.com".to_string()),
            review_completion_date: None,
            review_group_id: Some("issn:1234-5678".to_string()),
            subject_external_identifier: Some(("doi".to_string(), "10.1234/test".to_string())),
            subject_type: Some("journal-article".to_string()),
            subject_name: Some("Test Article".to_string()),
            subject_url: Some("https://doi.org/10.1234/test".to_string()),
            external_ids: vec![("id".to_string(), "123".to_string())],
        };

        let cloned = review.clone();
        assert_eq!(cloned.review_type(), review.review_type());
        assert_eq!(cloned.review_role(), review.review_role());
        assert_eq!(cloned.review_url(), review.review_url());
        assert_eq!(cloned.review_group_id(), review.review_group_id());
        assert_eq!(
            cloned.subject_external_identifier(),
            review.subject_external_identifier()
        );
        assert_eq!(cloned.subject_type(), review.subject_type());
        assert_eq!(cloned.subject_name(), review.subject_name());
        assert_eq!(cloned.subject_url(), review.subject_url());
        assert_eq!(cloned.external_ids(), review.external_ids());
    }
}
