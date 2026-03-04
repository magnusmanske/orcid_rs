use serde_json;

#[derive(Debug, Clone)]
pub struct Organization {
    name: Option<String>,
    city: Option<String>,
    region: Option<String>,
    country: Option<String>,
    disambiguated_organization: Option<(String, String)>, // TODO external IDS
}

impl Organization {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        let d_o = match j["disambiguated-organization"].as_object() {
            Some(o) => match (
                o.get("disambiguation-source"),
                o.get("disambiguated-organization-identifier"),
            ) {
                (Some(source), Some(id)) => Some((source.to_string(), id.to_string())),
                _ => None,
            },
            None => None,
        };

        Self {
            name: j["name"].as_str().map(|s| s.to_string()),
            city: j["address"]["city"].as_str().map(|s| s.to_string()),
            region: j["address"]["region"].as_str().map(|s| s.to_string()),
            country: j["address"]["country"].as_str().map(|s| s.to_string()),
            disambiguated_organization: d_o,
        }
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn city(&self) -> Option<&String> {
        self.city.as_ref()
    }

    pub fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }

    pub fn country(&self) -> Option<&String> {
        self.country.as_ref()
    }

    pub fn disambiguated_organization(&self) -> Option<&(String, String)> {
        self.disambiguated_organization.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json_complete() {
        let j = json!({
            "name": "Test University",
            "address": {
                "city": "Test City",
                "region": "Test Region",
                "country": "Test Country"
            },
            "disambiguated-organization": {
                "disambiguation-source": "RINGGOLD",
                "disambiguated-organization-identifier": "12345"
            }
        });

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), Some(&"Test University".to_string()));
        assert_eq!(org.city(), Some(&"Test City".to_string()));
        assert_eq!(org.region(), Some(&"Test Region".to_string()));
        assert_eq!(org.country(), Some(&"Test Country".to_string()));
        assert_eq!(
            org.disambiguated_organization(),
            Some(&("\"RINGGOLD\"".to_string(), "\"12345\"".to_string()))
        );
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "name": "Test University"
        });

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), Some(&"Test University".to_string()));
        assert_eq!(org.city(), None);
        assert_eq!(org.region(), None);
        assert_eq!(org.country(), None);
        assert_eq!(org.disambiguated_organization(), None);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({});

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), None);
        assert_eq!(org.city(), None);
        assert_eq!(org.region(), None);
        assert_eq!(org.country(), None);
        assert_eq!(org.disambiguated_organization(), None);
    }

    #[test]
    fn test_new_from_json_no_address() {
        let j = json!({
            "name": "Test University",
            "disambiguated-organization": {
                "disambiguation-source": "GRID",
                "disambiguated-organization-identifier": "grid.12345.6"
            }
        });

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), Some(&"Test University".to_string()));
        assert_eq!(org.city(), None);
        assert_eq!(org.region(), None);
        assert_eq!(org.country(), None);
        assert_eq!(
            org.disambiguated_organization(),
            Some(&("\"GRID\"".to_string(), "\"grid.12345.6\"".to_string()))
        );
    }

    #[test]
    fn test_new_from_json_incomplete_disambiguated() {
        let j = json!({
            "name": "Test University",
            "disambiguated-organization": {
                "disambiguation-source": "RINGGOLD"
                // Missing identifier
            }
        });

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), Some(&"Test University".to_string()));
        assert_eq!(org.disambiguated_organization(), None);
    }

    #[test]
    fn test_new_from_json_partial_address() {
        let j = json!({
            "name": "Test University",
            "address": {
                "city": "Test City",
                "country": "Test Country"
                // Missing region
            }
        });

        let org = Organization::new_from_json(&j);

        assert_eq!(org.name(), Some(&"Test University".to_string()));
        assert_eq!(org.city(), Some(&"Test City".to_string()));
        assert_eq!(org.region(), None);
        assert_eq!(org.country(), Some(&"Test Country".to_string()));
    }
}
