use serde_json;

#[derive(Debug, Clone)]
pub struct Date {
    year: Option<u16>,
    month: Option<u8>,
    day: Option<u8>,
}

impl Date {
    pub fn new_from_json(j: &serde_json::Value) -> Self {
        Self {
            year: j["year"]["value"].as_u64().map(|x| x as u16),
            month: j["month"]["value"].as_u64().map(|x| x as u8),
            day: j["day"]["value"].as_u64().map(|x| x as u8),
        }
    }

    pub fn year(&self) -> Option<u16> {
        self.year
    }

    pub fn month(&self) -> Option<u8> {
        self.month
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
            "year": { "value": 2023 },
            "month": { "value": 12 },
            "day": { "value": 25 }
        });

        let date = Date::new_from_json(&j);
        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), Some(12));
        assert_eq!(date.day(), Some(25));
    }

    #[test]
    fn test_new_from_json_partial() {
        let j = json!({
            "year": { "value": 2023 },
            "month": {},
            "day": {}
        });

        let date = Date::new_from_json(&j);
        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), None);
        assert_eq!(date.day(), None);
    }

    #[test]
    fn test_new_from_json_empty() {
        let j = json!({});

        let date = Date::new_from_json(&j);
        assert_eq!(date.year(), None);
        assert_eq!(date.month(), None);
        assert_eq!(date.day(), None);
    }

    #[test]
    fn test_new_from_json_large_values() {
        let j = json!({
            "year": { "value": 65535 }, // Max u16
            "month": { "value": 255 }, // Max u8
            "day": { "value": 255 } // Max u8
        });

        let date = Date::new_from_json(&j);
        assert_eq!(date.year(), Some(65535));
        assert_eq!(date.month(), Some(255));
        assert_eq!(date.day(), Some(255));
    }

    #[test]
    fn test_new_from_json_non_numeric() {
        let j = json!({
            "year": { "value": "not a number" },
            "month": { "value": "twelve" },
            "day": { "value": "twenty-five" }
        });

        let date = Date::new_from_json(&j);
        assert_eq!(date.year(), None);
        assert_eq!(date.month(), None);
        assert_eq!(date.day(), None);
    }
}
