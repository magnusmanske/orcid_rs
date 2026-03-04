use crate::error::Result;
use crate::Client;

#[derive(Debug, Clone)]
pub struct SearchBuilder<'a> {
    client: &'a Client,
    keyword: Option<String>,
    affiliation: Option<String>,
    given_names: Option<String>,
    family_name: Option<String>,
    orcid: Option<String>,
    doi: Option<String>,
    eid: Option<String>,
    pmid: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl<'a> SearchBuilder<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self {
            client,
            keyword: None,
            affiliation: None,
            given_names: None,
            family_name: None,
            orcid: None,
            doi: None,
            eid: None,
            pmid: None,
            limit: None,
            offset: None,
        }
    }

    /// Add a keyword to search for
    pub fn with_keyword(mut self, keyword: &str) -> Self {
        self.keyword = Some(keyword.to_string());
        self
    }

    /// Add an affiliation to search for
    pub fn with_affiliation(mut self, affiliation: &str) -> Self {
        self.affiliation = Some(affiliation.to_string());
        self
    }

    /// Add given names to search for
    pub fn with_given_names(mut self, given_names: &str) -> Self {
        self.given_names = Some(given_names.to_string());
        self
    }

    /// Add family name to search for
    pub fn with_family_name(mut self, family_name: &str) -> Self {
        self.family_name = Some(family_name.to_string());
        self
    }

    /// Add ORCID ID to search for
    pub fn with_orcid(mut self, orcid: &str) -> Self {
        self.orcid = Some(orcid.to_string());
        self
    }

    /// Add DOI to search for
    pub fn with_doi(mut self, doi: &str) -> Self {
        self.doi = Some(doi.to_string());
        self
    }

    /// Add EID to search for
    pub fn with_eid(mut self, eid: &str) -> Self {
        self.eid = Some(eid.to_string());
        self
    }

    /// Add PubMed ID to search for
    pub fn with_pmid(mut self, pmid: &str) -> Self {
        self.pmid = Some(pmid.to_string());
        self
    }

    /// Set the maximum number of results to return
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the offset for pagination
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    // Getter methods for testing
    pub fn get_keyword(&self) -> Option<&str> {
        self.keyword.as_deref()
    }

    pub fn get_affiliation(&self) -> Option<&str> {
        self.affiliation.as_deref()
    }

    pub fn get_limit(&self) -> Option<usize> {
        self.limit
    }

    /// Build the search query string
    pub fn build_query(&self) -> String {
        let mut parts = Vec::new();

        if let Some(ref keyword) = self.keyword {
            parts.push(format!("text:{}", quote_if_needed(keyword)));
        }

        if let Some(ref affiliation) = self.affiliation {
            parts.push(format!(
                "affiliation-org-name:{}",
                quote_if_needed(affiliation)
            ));
        }

        if let Some(ref given_names) = self.given_names {
            parts.push(format!("given-names:{}", quote_if_needed(given_names)));
        }

        if let Some(ref family_name) = self.family_name {
            parts.push(format!("family-name:{}", quote_if_needed(family_name)));
        }

        if let Some(ref orcid) = self.orcid {
            parts.push(format!("orcid:{}", orcid));
        }

        if let Some(ref doi) = self.doi {
            parts.push(format!("doi-self:{}", quote_if_needed(doi)));
        }

        if let Some(ref eid) = self.eid {
            parts.push(format!("eid:{}", eid));
        }

        if let Some(ref pmid) = self.pmid {
            parts.push(format!("pmid:{}", pmid));
        }

        let mut query = parts.join(" AND ");

        // Add limit and offset as query parameters
        let mut params = Vec::new();
        if let Some(limit) = self.limit {
            params.push(format!("rows={}", limit));
        }
        if let Some(offset) = self.offset {
            params.push(format!("start={}", offset));
        }

        if !params.is_empty() {
            if !query.is_empty() {
                query = format!("{}&{}", query, params.join("&"));
            } else {
                query = params.join("&");
            }
        }

        query
    }

    /// Execute the search and return ORCID IDs
    pub fn execute(&self) -> Result<Vec<String>> {
        let query = self.build_query();
        self.client.search(&query)
    }
}

/// Quote a string if it contains spaces or special characters
fn quote_if_needed(s: &str) -> String {
    if s.contains(' ') || s.contains('"') || s.contains(':') {
        format!("\"{}\"", s.replace('"', "\\\""))
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_builder_basic() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client);

        assert!(builder.keyword.is_none());
        assert!(builder.affiliation.is_none());
        assert!(builder.limit.is_none());
    }

    #[test]
    fn test_search_builder_with_keyword() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client).with_keyword("climate");

        assert_eq!(builder.get_keyword(), Some("climate"));
        assert_eq!(builder.build_query(), "text:climate");
    }

    #[test]
    fn test_search_builder_with_affiliation() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client).with_affiliation("MIT");

        assert_eq!(builder.get_affiliation(), Some("MIT"));
        assert_eq!(builder.build_query(), "affiliation-org-name:MIT");
    }

    #[test]
    fn test_search_builder_multiple_criteria() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client)
            .with_keyword("quantum computing")
            .with_affiliation("Stanford University")
            .with_family_name("Smith");

        let query = builder.build_query();
        assert!(query.contains("text:\"quantum computing\""));
        assert!(query.contains("affiliation-org-name:\"Stanford University\""));
        assert!(query.contains("family-name:Smith"));
        assert!(query.contains(" AND "));
    }

    #[test]
    fn test_search_builder_with_pagination() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client)
            .with_keyword("physics")
            .limit(50)
            .offset(100);

        let query = builder.build_query();
        assert!(query.contains("text:physics"));
        assert!(query.contains("rows=50"));
        assert!(query.contains("start=100"));
    }

    #[test]
    fn test_search_builder_doi_search() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client).with_doi("10.1038/nature12373");

        let query = builder.build_query();
        assert_eq!(query, "doi-self:10.1038/nature12373");
    }

    #[test]
    fn test_quote_if_needed() {
        assert_eq!(quote_if_needed("simple"), "simple");
        assert_eq!(quote_if_needed("with space"), "\"with space\"");
        assert_eq!(quote_if_needed("with:colon"), "\"with:colon\"");
        assert_eq!(quote_if_needed("with\"quote"), "\"with\\\"quote\"");
    }

    #[test]
    fn test_search_builder_all_fields() {
        let client = Client::new();
        let builder = SearchBuilder::new(&client)
            .with_keyword("test")
            .with_affiliation("University")
            .with_given_names("John")
            .with_family_name("Doe")
            .with_orcid("0000-0001-2345-6789")
            .with_doi("10.1234/test")
            .with_eid("2-s2.0-12345")
            .with_pmid("12345678")
            .limit(25)
            .offset(50);

        let query = builder.build_query();
        assert!(query.contains("text:test"));
        assert!(query.contains("affiliation-org-name:University"));
        assert!(query.contains("given-names:John"));
        assert!(query.contains("family-name:Doe"));
        assert!(query.contains("orcid:0000-0001-2345-6789"));
        assert!(query.contains("doi-self:10.1234/test"));
        assert!(query.contains("eid:2-s2.0-12345"));
        assert!(query.contains("pmid:12345678"));
        assert!(query.contains("rows=25"));
        assert!(query.contains("start=50"));
    }

    #[test]
    fn test_search_builder_chaining() {
        let client = Client::new();
        let query = SearchBuilder::new(&client)
            .with_keyword("climate change")
            .with_affiliation("Harvard")
            .limit(100)
            .build_query();

        assert!(query.contains("text:\"climate change\""));
        assert!(query.contains("affiliation-org-name:Harvard"));
        assert!(query.contains("rows=100"));
    }
}
