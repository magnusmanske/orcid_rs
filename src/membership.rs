use crate::date::Date;
use crate::organization::Organization;
use serde_json;

#[derive(Debug, Clone)]
pub struct Membership {
    organization: Option<Organization>,
    department_name: Option<String>,
    role_title: Option<String>,
    start_date: Option<Date>,
    end_date: Option<Date>,
    external_ids: Vec<(String, String)>,
    url: Option<String>,
}

impl Membership {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        let membership_summary = &j["membership-summary"][0];

        let external_ids =
            if let Some(ext_ids) = membership_summary["external-ids"]["external-id"].as_array() {
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

        Self {
            organization: if membership_summary["organization"].is_object() {
                Some(Organization::new_from_json(
                    &membership_summary["organization"],
                ))
            } else {
                None
            },
            department_name: membership_summary["department-name"]
                .as_str()
                .map(|s| s.to_string()),
            role_title: membership_summary["role-title"]
                .as_str()
                .map(|s| s.to_string()),
            start_date: if membership_summary["start-date"].is_object() {
                Some(Date::new_from_json(&membership_summary["start-date"]))
            } else {
                None
            },
            end_date: if membership_summary["end-date"].is_object() {
                Some(Date::new_from_json(&membership_summary["end-date"]))
            } else {
                None
            },
            external_ids,
            url: membership_summary["url"]["value"]
                .as_str()
                .map(|s| s.to_string()),
        }
    }

    // Getter methods
    pub fn organization(&self) -> Option<&Organization> {
        self.organization.as_ref()
    }

    pub fn department_name(&self) -> Option<&String> {
        self.department_name.as_ref()
    }

    pub fn role_title(&self) -> Option<&String> {
        self.role_title.as_ref()
    }

    pub fn start_date(&self) -> Option<&Date> {
        self.start_date.as_ref()
    }

    pub fn end_date(&self) -> Option<&Date> {
        self.end_date.as_ref()
    }

    pub fn external_ids(&self) -> &Vec<(String, String)> {
        &self.external_ids
    }

    pub fn url(&self) -> Option<&String> {
        self.url.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json_complete() {
        let j = json!({
            "membership-summary": [{
                "organization": {
                    "name": "Professional Society",
                    "address": {
                        "city": "London",
                        "region": "England",
                        "country": "GB"
                    }
                },
                "department-name": "Computer Science Division",
                "role-title": "Fellow",
                "start-date": {
                    "year": { "value": 2020 },
                    "month": { "value": 6 },
                    "day": { "value": 1 }
                },
                "end-date": {
                    "year": { "value": 2025 },
                    "month": { "value": 5 },
                    "day": { "value": 31 }
                },
                "external-ids": {
                    "external-id": [{
                        "external-id-type": "membership-id",
                        "external-id-value": "MEM-2020-12345"
                    }]
                },
                "url": {
                    "value": "https://example.org/members/12345"
                }
            }]
        });

        let membership = Membership::new_from_json(&j);

        assert!(membership.organization().is_some());
        assert_eq!(
            membership.organization().unwrap().name(),
            Some(&"Professional Society".to_string())
        );
        assert_eq!(
            membership.department_name(),
            Some(&"Computer Science Division".to_string())
        );
        assert_eq!(membership.role_title(), Some(&"Fellow".to_string()));
        assert!(membership.start_date().is_some());
        assert!(membership.end_date().is_some());
        assert_eq!(membership.external_ids().len(), 1);
        assert_eq!(
            membership.external_ids()[0],
            ("membership-id".to_string(), "MEM-2020-12345".to_string())
        );
        assert_eq!(
            membership.url(),
            Some(&"https://example.org/members/12345".to_string())
        );
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "membership-summary": [{
                "organization": {
                    "name": "Basic Society"
                }
            }]
        });

        let membership = Membership::new_from_json(&j);

        assert!(membership.organization().is_some());
        assert_eq!(membership.department_name(), None);
        assert_eq!(membership.role_title(), None);
        assert!(membership.start_date().is_none());
        assert!(membership.end_date().is_none());
        assert_eq!(membership.external_ids().len(), 0);
        assert_eq!(membership.url(), None);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({
            "membership-summary": [{}]
        });

        let membership = Membership::new_from_json(&j);

        assert!(membership.organization().is_none());
        assert_eq!(membership.department_name(), None);
        assert_eq!(membership.role_title(), None);
        assert!(membership.start_date().is_none());
        assert!(membership.end_date().is_none());
        assert_eq!(membership.external_ids().len(), 0);
        assert_eq!(membership.url(), None);
    }

    #[test]
    fn test_new_from_json_multiple_external_ids() {
        let j = json!({
            "membership-summary": [{
                "organization": {
                    "name": "Multi-ID Society"
                },
                "external-ids": {
                    "external-id": [
                        {
                            "external-id-type": "membership-id",
                            "external-id-value": "MEM-123"
                        },
                        {
                            "external-id-type": "legacy-id",
                            "external-id-value": "OLD-456"
                        },
                        {
                            "external-id-type": "doi",
                            "external-id-value": "10.1234/membership"
                        }
                    ]
                }
            }]
        });

        let membership = Membership::new_from_json(&j);

        assert_eq!(membership.external_ids().len(), 3);
        assert_eq!(
            membership.external_ids()[0],
            ("membership-id".to_string(), "MEM-123".to_string())
        );
        assert_eq!(
            membership.external_ids()[1],
            ("legacy-id".to_string(), "OLD-456".to_string())
        );
        assert_eq!(
            membership.external_ids()[2],
            ("doi".to_string(), "10.1234/membership".to_string())
        );
    }

    #[test]
    fn test_debug_trait() {
        let membership = Membership {
            organization: None,
            department_name: Some("Test Dept".to_string()),
            role_title: Some("Member".to_string()),
            start_date: None,
            end_date: None,
            external_ids: vec![],
            url: None,
        };

        let debug_str = format!("{:?}", membership);
        assert!(debug_str.contains("Membership"));
        assert!(debug_str.contains("Test Dept"));
        assert!(debug_str.contains("Member"));
    }

    #[test]
    fn test_clone_trait() {
        let membership = Membership {
            organization: None,
            department_name: Some("Engineering".to_string()),
            role_title: Some("Senior Member".to_string()),
            start_date: None,
            end_date: None,
            external_ids: vec![("id".to_string(), "123".to_string())],
            url: Some("https://example.com".to_string()),
        };

        let cloned = membership.clone();
        assert_eq!(cloned.department_name(), membership.department_name());
        assert_eq!(cloned.role_title(), membership.role_title());
        assert_eq!(cloned.external_ids(), membership.external_ids());
        assert_eq!(cloned.url(), membership.url());
    }
}
