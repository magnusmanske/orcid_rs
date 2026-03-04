use crate::publication_date::PublicationDate;
use crate::utils::collect_parts;
use serde_json;

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_new_from_json() {
        let j = json!({
            "work-summary": [{
                "title": {
                    "title": {
                        "value": "Test Publication"
                    }
                },
                "type": "journal-article",
                "publication-date": {
                    "year": { "value": "2023" },
                    "month": { "value": "6" },
                    "day": { "value": "15" }
                }
            }],
            "external-ids": {
                "external-id": [{
                    "external-id-type": "doi",
                    "external-id-value": "10.1234/test"
                }, {
                    "external-id-type": "pmid",
                    "external-id-value": "12345678"
                }]
            }
        });

        let work = Work::new_from_json(&j);

        assert_eq!(work.title, Some("Test Publication".to_string()));
        assert_eq!(work.pub_type, Some("journal-article".to_string()));
        assert_eq!(work.external_ids.len(), 2);
        assert_eq!(
            work.external_ids[0],
            ("doi".to_string(), "10.1234/test".to_string())
        );
        assert_eq!(
            work.external_ids[1],
            ("pmid".to_string(), "12345678".to_string())
        );
        assert_eq!(work.publication_date.year(), Some(2023));
        assert_eq!(work.publication_date.month(), Some(6));
        assert_eq!(work.publication_date.day(), Some(15));
    }

    #[test]
    fn test_new_from_json_minimal() {
        let j = json!({
            "work-summary": [{
                "title": {
                    "title": {}
                },
                "publication-date": {}
            }],
            "external-ids": {
                "external-id": []
            }
        });

        let work = Work::new_from_json(&j);

        assert_eq!(work.title, None);
        assert_eq!(work.pub_type, None);
        assert_eq!(work.external_ids.len(), 0);
        assert_eq!(work.publication_date.year(), None);
        assert_eq!(work.publication_date.month(), None);
        assert_eq!(work.publication_date.day(), None);
    }
}
