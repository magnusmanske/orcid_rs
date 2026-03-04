use crate::date::Date;
use crate::organization::Organization;
use serde_json;

#[derive(Debug, Clone)]
pub struct Qualification {
    organization: Option<Organization>,
    department_name: Option<String>,
    role_title: Option<String>,
    start_date: Option<Date>,
    end_date: Option<Date>,
    external_ids: Vec<(String, String)>,
    url: Option<String>,
}

impl Qualification {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        let qualification_summary = &j["qualification-summary"][0];

        let external_ids = if let Some(ext_ids) =
            qualification_summary["external-ids"]["external-id"].as_array()
        {
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
            organization: if qualification_summary["organization"].is_object() {
                Some(Organization::new_from_json(
                    &qualification_summary["organization"],
                ))
            } else {
                None
            },
            department_name: qualification_summary["department-name"]
                .as_str()
                .map(|s| s.to_string()),
            role_title: qualification_summary["role-title"]
                .as_str()
                .map(|s| s.to_string()),
            start_date: if qualification_summary["start-date"].is_object() {
                Some(Date::new_from_json(&qualification_summary["start-date"]))
            } else {
                None
            },
            end_date: if qualification_summary["end-date"].is_object() {
                Some(Date::new_from_json(&qualification_summary["end-date"]))
            } else {
                None
            },
            external_ids,
            url: qualification_summary["url"]["value"]
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
            "qualification-summary": [{
                "organization": {
                    "name": "Oxford University",
                    "address": {
                        "city": "Oxford",
                        "region": "Oxfordshire",
                        "country": "GB"
                    }
                },
                "department-name": "Physics Department",
                "role-title": "Doctor of Philosophy",
                "start-date": {
                    "year": { "value": 2015 },
                    "month": { "value": 9 },
                    "day": { "value": 1 }
                },
                "end-date": {
                    "year": { "value": 2019 },
                    "month": { "value": 6 },
                    "day": { "value": 30 }
                },
                "external-ids": {
                    "external-id": [{
                        "external-id-type": "qualification-id",
                        "external-id-value": "QUAL-2019-PHD-12345"
                    }]
                },
                "url": {
                    "value": "https://oxford.edu/qualifications/phd/12345"
                }
            }]
        });

        let qualification = Qualification::new_from_json(&j);

        assert!(qualification.organization().is_some());
        assert_eq!(
            qualification.organization().unwrap().name(),
            Some(&"Oxford University".to_string())
        );
        assert_eq!(
            qualification.department_name(),
            Some(&"Physics Department".to_string())
        );
        assert_eq!(
            qualification.role_title(),
            Some(&"Doctor of Philosophy".to_string())
        );
        assert!(qualification.start_date().is_some());
        assert!(qualification.end_date().is_some());
        assert_eq!(qualification.external_ids().len(), 1);
        assert_eq!(
            qualification.external_ids()[0],
            (
                "qualification-id".to_string(),
                "QUAL-2019-PHD-12345".to_string()
            )
        );
        assert_eq!(
            qualification.url(),
            Some(&"https://oxford.edu/qualifications/phd/12345".to_string())
        );
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "qualification-summary": [{
                "organization": {
                    "name": "Community College"
                },
                "role-title": "Associate Degree"
            }]
        });

        let qualification = Qualification::new_from_json(&j);

        assert!(qualification.organization().is_some());
        assert_eq!(
            qualification.organization().unwrap().name(),
            Some(&"Community College".to_string())
        );
        assert_eq!(qualification.department_name(), None);
        assert_eq!(
            qualification.role_title(),
            Some(&"Associate Degree".to_string())
        );
        assert!(qualification.start_date().is_none());
        assert!(qualification.end_date().is_none());
        assert_eq!(qualification.external_ids().len(), 0);
        assert_eq!(qualification.url(), None);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({
            "qualification-summary": [{}]
        });

        let qualification = Qualification::new_from_json(&j);

        assert!(qualification.organization().is_none());
        assert_eq!(qualification.department_name(), None);
        assert_eq!(qualification.role_title(), None);
        assert!(qualification.start_date().is_none());
        assert!(qualification.end_date().is_none());
        assert_eq!(qualification.external_ids().len(), 0);
        assert_eq!(qualification.url(), None);
    }

    #[test]
    fn test_new_from_json_multiple_external_ids() {
        let j = json!({
            "qualification-summary": [{
                "organization": {
                    "name": "University"
                },
                "role-title": "Bachelor of Science",
                "external-ids": {
                    "external-id": [
                        {
                            "external-id-type": "student-id",
                            "external-id-value": "STU-123456"
                        },
                        {
                            "external-id-type": "certificate-number",
                            "external-id-value": "CERT-789012"
                        },
                        {
                            "external-id-type": "diploma-id",
                            "external-id-value": "DIP-345678"
                        }
                    ]
                }
            }]
        });

        let qualification = Qualification::new_from_json(&j);

        assert_eq!(qualification.external_ids().len(), 3);
        assert_eq!(
            qualification.external_ids()[0],
            ("student-id".to_string(), "STU-123456".to_string())
        );
        assert_eq!(
            qualification.external_ids()[1],
            ("certificate-number".to_string(), "CERT-789012".to_string())
        );
        assert_eq!(
            qualification.external_ids()[2],
            ("diploma-id".to_string(), "DIP-345678".to_string())
        );
    }

    #[test]
    fn test_debug_trait() {
        let qualification = Qualification {
            organization: None,
            department_name: Some("Mathematics".to_string()),
            role_title: Some("Master of Science".to_string()),
            start_date: None,
            end_date: None,
            external_ids: vec![],
            url: None,
        };

        let debug_str = format!("{:?}", qualification);
        assert!(debug_str.contains("Qualification"));
        assert!(debug_str.contains("Mathematics"));
        assert!(debug_str.contains("Master of Science"));
    }

    #[test]
    fn test_clone_trait() {
        let qualification = Qualification {
            organization: None,
            department_name: Some("Computer Science".to_string()),
            role_title: Some("PhD".to_string()),
            start_date: None,
            end_date: None,
            external_ids: vec![("id".to_string(), "123".to_string())],
            url: Some("https://example.edu".to_string()),
        };

        let cloned = qualification.clone();
        assert_eq!(cloned.department_name(), qualification.department_name());
        assert_eq!(cloned.role_title(), qualification.role_title());
        assert_eq!(cloned.external_ids(), qualification.external_ids());
        assert_eq!(cloned.url(), qualification.url());
    }

    #[test]
    fn test_professional_qualifications() {
        let j = json!({
            "qualification-summary": [{
                "organization": {
                    "name": "Professional Certification Body"
                },
                "department-name": "Information Technology",
                "role-title": "Certified Information Systems Security Professional",
                "start-date": {
                    "year": { "value": 2020 }
                },
                "end-date": {
                    "year": { "value": 2023 }
                },
                "external-ids": {
                    "external-id": [{
                        "external-id-type": "certification-number",
                        "external-id-value": "CISSP-2020-98765"
                    }]
                }
            }]
        });

        let qualification = Qualification::new_from_json(&j);

        assert_eq!(
            qualification.department_name(),
            Some(&"Information Technology".to_string())
        );
        assert_eq!(
            qualification.role_title(),
            Some(&"Certified Information Systems Security Professional".to_string())
        );
        assert!(qualification.start_date().is_some());
        assert!(qualification.end_date().is_some());
        assert_eq!(
            qualification.external_ids()[0],
            (
                "certification-number".to_string(),
                "CISSP-2020-98765".to_string()
            )
        );
    }
}
