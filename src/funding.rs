use crate::date::Date;
use crate::organization::Organization;
use serde_json;

#[derive(Debug, Clone)]
pub struct Funding {
    title: Option<String>,
    translated_title: Option<(String, String)>, // (title, language_code)
    funding_type: Option<String>,
    organization_defined_type: Option<String>,
    short_description: Option<String>,
    amount: Option<String>,
    currency: Option<String>,
    start_date: Option<Date>,
    end_date: Option<Date>,
    organization: Option<Organization>,
    external_ids: Vec<(String, String)>,
    url: Option<String>,
}

impl Funding {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        let translated_title =
            match j["funding-summary"][0]["title"]["translated-title"].as_object() {
                Some(tt) => match (tt.get("value"), tt.get("language-code")) {
                    (Some(title), Some(lang)) => match (title.as_str(), lang.as_str()) {
                        (Some(t), Some(l)) => Some((t.to_string(), l.to_string())),
                        _ => None,
                    },
                    _ => None,
                },
                None => None,
            };

        let external_ids = if let Some(ext_ids) =
            j["funding-summary"][0]["external-ids"]["external-id"].as_array()
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
            title: j["funding-summary"][0]["title"]["title"]["value"]
                .as_str()
                .map(|s| s.to_string()),
            translated_title,
            funding_type: j["funding-summary"][0]["type"]
                .as_str()
                .map(|s| s.to_string()),
            organization_defined_type: j["funding-summary"][0]["organization-defined-type"]
                ["value"]
                .as_str()
                .map(|s| s.to_string()),
            short_description: j["funding-summary"][0]["short-description"]
                .as_str()
                .map(|s| s.to_string()),
            amount: j["funding-summary"][0]["amount"]["value"]
                .as_str()
                .map(|s| s.to_string()),
            currency: j["funding-summary"][0]["amount"]["currency-code"]
                .as_str()
                .map(|s| s.to_string()),
            start_date: if j["funding-summary"][0]["start-date"].is_object() {
                Some(Date::new_from_json(&j["funding-summary"][0]["start-date"]))
            } else {
                None
            },
            end_date: if j["funding-summary"][0]["end-date"].is_object() {
                Some(Date::new_from_json(&j["funding-summary"][0]["end-date"]))
            } else {
                None
            },
            organization: if j["funding-summary"][0]["organization"].is_object() {
                Some(Organization::new_from_json(
                    &j["funding-summary"][0]["organization"],
                ))
            } else {
                None
            },
            external_ids,
            url: j["funding-summary"][0]["url"]["value"]
                .as_str()
                .map(|s| s.to_string()),
        }
    }

    // Getter methods
    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
    }

    pub fn translated_title(&self) -> Option<&(String, String)> {
        self.translated_title.as_ref()
    }

    pub fn funding_type(&self) -> Option<&String> {
        self.funding_type.as_ref()
    }

    pub fn organization_defined_type(&self) -> Option<&String> {
        self.organization_defined_type.as_ref()
    }

    pub fn short_description(&self) -> Option<&String> {
        self.short_description.as_ref()
    }

    pub fn amount(&self) -> Option<&String> {
        self.amount.as_ref()
    }

    pub fn currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }

    pub fn start_date(&self) -> Option<&Date> {
        self.start_date.as_ref()
    }

    pub fn end_date(&self) -> Option<&Date> {
        self.end_date.as_ref()
    }

    pub fn organization(&self) -> Option<&Organization> {
        self.organization.as_ref()
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
            "funding-summary": [{
                "title": {
                    "title": {
                        "value": "Research Grant for Climate Studies"
                    },
                    "translated-title": {
                        "value": "Subvención de investigación para estudios climáticos",
                        "language-code": "es"
                    }
                },
                "type": "grant",
                "organization-defined-type": {
                    "value": "Standard Research Grant"
                },
                "short-description": "A grant to study climate change impacts",
                "amount": {
                    "value": "100000",
                    "currency-code": "USD"
                },
                "start-date": {
                    "year": { "value": 2023 },
                    "month": { "value": 1 },
                    "day": { "value": 15 }
                },
                "end-date": {
                    "year": { "value": 2025 },
                    "month": { "value": 12 },
                    "day": { "value": 31 }
                },
                "organization": {
                    "name": "National Science Foundation",
                    "address": {
                        "city": "Alexandria",
                        "region": "VA",
                        "country": "US"
                    }
                },
                "external-ids": {
                    "external-id": [{
                        "external-id-type": "grant_number",
                        "external-id-value": "NSF-2023-12345"
                    }]
                },
                "url": {
                    "value": "https://example.com/grant/12345"
                }
            }]
        });

        let funding = Funding::new_from_json(&j);

        assert_eq!(
            funding.title(),
            Some(&"Research Grant for Climate Studies".to_string())
        );
        assert_eq!(
            funding.translated_title(),
            Some(&(
                "Subvención de investigación para estudios climáticos".to_string(),
                "es".to_string()
            ))
        );
        assert_eq!(funding.funding_type(), Some(&"grant".to_string()));
        assert_eq!(
            funding.organization_defined_type(),
            Some(&"Standard Research Grant".to_string())
        );
        assert_eq!(
            funding.short_description(),
            Some(&"A grant to study climate change impacts".to_string())
        );
        assert_eq!(funding.amount(), Some(&"100000".to_string()));
        assert_eq!(funding.currency(), Some(&"USD".to_string()));
        assert!(funding.start_date().is_some());
        assert!(funding.end_date().is_some());
        assert!(funding.organization().is_some());
        assert_eq!(funding.external_ids().len(), 1);
        assert_eq!(
            funding.external_ids()[0],
            ("grant_number".to_string(), "NSF-2023-12345".to_string())
        );
        assert_eq!(
            funding.url(),
            Some(&"https://example.com/grant/12345".to_string())
        );
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "funding-summary": [{
                "title": {
                    "title": {
                        "value": "Basic Grant"
                    }
                }
            }]
        });

        let funding = Funding::new_from_json(&j);

        assert_eq!(funding.title(), Some(&"Basic Grant".to_string()));
        assert_eq!(funding.translated_title(), None);
        assert_eq!(funding.funding_type(), None);
        assert_eq!(funding.organization_defined_type(), None);
        assert_eq!(funding.short_description(), None);
        assert_eq!(funding.amount(), None);
        assert_eq!(funding.currency(), None);
        assert!(funding.start_date().is_none());
        assert!(funding.end_date().is_none());
        assert!(funding.organization().is_none());
        assert_eq!(funding.external_ids().len(), 0);
        assert_eq!(funding.url(), None);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({
            "funding-summary": [{}]
        });

        let funding = Funding::new_from_json(&j);

        assert_eq!(funding.title(), None);
        assert_eq!(funding.translated_title(), None);
        assert_eq!(funding.funding_type(), None);
        assert_eq!(funding.organization_defined_type(), None);
        assert_eq!(funding.short_description(), None);
        assert_eq!(funding.amount(), None);
        assert_eq!(funding.currency(), None);
        assert!(funding.start_date().is_none());
        assert!(funding.end_date().is_none());
        assert!(funding.organization().is_none());
        assert_eq!(funding.external_ids().len(), 0);
        assert_eq!(funding.url(), None);
    }

    #[test]
    fn test_new_from_json_multiple_external_ids() {
        let j = json!({
            "funding-summary": [{
                "title": {
                    "title": {
                        "value": "Multi-ID Grant"
                    }
                },
                "external-ids": {
                    "external-id": [
                        {
                            "external-id-type": "grant_number",
                            "external-id-value": "ABC-123"
                        },
                        {
                            "external-id-type": "proposal_id",
                            "external-id-value": "PROP-456"
                        },
                        {
                            "external-id-type": "award_number",
                            "external-id-value": "AWD-789"
                        }
                    ]
                }
            }]
        });

        let funding = Funding::new_from_json(&j);

        assert_eq!(funding.title(), Some(&"Multi-ID Grant".to_string()));
        assert_eq!(funding.external_ids().len(), 3);
        assert_eq!(
            funding.external_ids()[0],
            ("grant_number".to_string(), "ABC-123".to_string())
        );
        assert_eq!(
            funding.external_ids()[1],
            ("proposal_id".to_string(), "PROP-456".to_string())
        );
        assert_eq!(
            funding.external_ids()[2],
            ("award_number".to_string(), "AWD-789".to_string())
        );
    }

    #[test]
    fn test_debug_trait() {
        let funding = Funding {
            title: Some("Test Grant".to_string()),
            translated_title: None,
            funding_type: Some("grant".to_string()),
            organization_defined_type: None,
            short_description: None,
            amount: None,
            currency: None,
            start_date: None,
            end_date: None,
            organization: None,
            external_ids: vec![],
            url: None,
        };

        let debug_str = format!("{:?}", funding);
        assert!(debug_str.contains("Funding"));
        assert!(debug_str.contains("Test Grant"));
        assert!(debug_str.contains("grant"));
    }

    #[test]
    fn test_clone_trait() {
        let funding = Funding {
            title: Some("Test Grant".to_string()),
            translated_title: Some(("Translated".to_string(), "en".to_string())),
            funding_type: Some("grant".to_string()),
            organization_defined_type: None,
            short_description: None,
            amount: Some("50000".to_string()),
            currency: Some("EUR".to_string()),
            start_date: None,
            end_date: None,
            organization: None,
            external_ids: vec![("id".to_string(), "123".to_string())],
            url: None,
        };

        let cloned = funding.clone();
        assert_eq!(cloned.title(), funding.title());
        assert_eq!(cloned.translated_title(), funding.translated_title());
        assert_eq!(cloned.funding_type(), funding.funding_type());
        assert_eq!(cloned.amount(), funding.amount());
        assert_eq!(cloned.currency(), funding.currency());
        assert_eq!(cloned.external_ids(), funding.external_ids());
    }
}
