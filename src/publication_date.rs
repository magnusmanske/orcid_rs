use serde_json;

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

    pub fn month(&self) -> Option<u8> {
        self.month
    }

    pub fn year(&self) -> Option<u32> {
        self.year
    }

    pub fn day(&self) -> Option<u8> {
        self.day
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json_complete() {
        let j = json!({
            "year": { "value": "2023" },
            "month": { "value": "12" },
            "day": { "value": "25" }
        });

        let date = PublicationDate::new_from_json(&j);
        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), Some(12));
        assert_eq!(date.day(), Some(25));
    }

    #[test]
    fn test_new_from_json_partial() {
        let j = json!({
            "year": { "value": "2023" },
            "month": {},
            "day": {}
        });

        let date = PublicationDate::new_from_json(&j);
        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), None);
        assert_eq!(date.day(), None);
    }

    #[test]
    fn test_new_from_json_invalid() {
        let j = json!({
            "year": { "value": "not_a_number" },
            "month": { "value": "13" }, // Invalid month
            "day": { "value": "32" } // Invalid day
        });

        let date = PublicationDate::new_from_json(&j);
        assert_eq!(date.year(), None);
        assert_eq!(date.month(), Some(13)); // Parser doesn't validate ranges
        assert_eq!(date.day(), Some(32)); // Parser doesn't validate ranges
    }
}
