use serde_json;

/// Extracts specified parts from a JSON array and returns them as a vector of string vectors.
///
/// This function is used to extract multiple fields from each object in a JSON array.
/// Each object in the array is processed to extract the values for the specified parts.
/// If a part is missing or not a string, an empty string is used.
///
/// # Arguments
///
/// * `j` - A JSON value that should be an array
/// * `parts` - A vector of field names to extract from each object in the array
///
/// # Returns
///
/// A vector where each element is a vector of strings corresponding to the extracted parts
///
/// # Example
///
/// ```
/// use orcid::utils::collect_parts;
/// use serde_json::json;
///
/// let j = json!([
///     { "type": "doi", "value": "10.1234/test" },
///     { "type": "pmid", "value": "12345678" }
/// ]);
///
/// let result = collect_parts(&j, vec!["type", "value"]);
/// // result: [["doi", "10.1234/test"], ["pmid", "12345678"]]
/// ```
pub fn collect_parts(j: &serde_json::Value, parts: Vec<&str>) -> Vec<Vec<String>> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_collect_parts() {
        let j = json!([
            {
                "type": "doi",
                "value": "10.1234/test"
            },
            {
                "type": "pmid",
                "value": "12345678"
            }
        ]);

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["doi", "10.1234/test"]);
        assert_eq!(result[1], vec!["pmid", "12345678"]);
    }

    #[test]
    fn test_collect_parts_missing_fields() {
        let j = json!([
            {
                "type": "doi"
                // missing "value"
            },
            {
                "value": "12345678"
                // missing "type"
            }
        ]);

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["doi", ""]);
        assert_eq!(result[1], vec!["", "12345678"]);
    }

    #[test]
    fn test_collect_parts_empty_array() {
        let j = json!([]);

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_collect_parts_not_array() {
        let j = json!({
            "not": "an array"
        });

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_collect_parts_null_values() {
        let j = json!([
            {
                "type": null,
                "value": "test"
            },
            {
                "type": "doi",
                "value": null
            }
        ]);

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], vec!["", "test"]);
        assert_eq!(result[1], vec!["doi", ""]);
    }

    #[test]
    fn test_collect_parts_numeric_values() {
        let j = json!([
            {
                "type": "numeric",
                "value": 123
            }
        ]);

        let result = collect_parts(&j, vec!["type", "value"]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], vec!["numeric", ""]); // Numbers are not converted to strings
    }
}
