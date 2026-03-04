use crate::date::Date;
use crate::organization::Organization;

#[derive(Debug, Clone)]
pub struct Role {
    department: Option<String>,
    title: Option<String>,
    start_date: Option<Date>,
    end_date: Option<Date>,
    organization: Option<Organization>,
    external_ids: Vec<(String, String)>,
}

impl Role {
    pub fn new() -> Self {
        Self {
            department: None,
            title: None,
            start_date: None,
            end_date: None,
            organization: None,
            external_ids: Vec::new(),
        }
    }

    // Getter methods
    pub fn department(&self) -> Option<&String> {
        self.department.as_ref()
    }

    pub fn title(&self) -> Option<&String> {
        self.title.as_ref()
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

    // Setter methods
    pub fn set_department(&mut self, department: Option<String>) {
        self.department = department;
    }

    pub fn set_title(&mut self, title: Option<String>) {
        self.title = title;
    }

    pub fn set_start_date(&mut self, start_date: Option<Date>) {
        self.start_date = start_date;
    }

    pub fn set_end_date(&mut self, end_date: Option<Date>) {
        self.end_date = end_date;
    }

    pub fn set_organization(&mut self, organization: Option<Organization>) {
        self.organization = organization;
    }

    pub fn external_ids(&self) -> &Vec<(String, String)> {
        &self.external_ids
    }

    pub fn add_external_id(&mut self, id_type: &str, id_value: &str) {
        self.external_ids
            .push((id_type.to_string(), id_value.to_string()));
    }

    pub fn set_external_ids(&mut self, external_ids: Vec<(String, String)>) {
        self.external_ids = external_ids;
    }
}

impl Default for Role {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let role = Role::new();
        assert!(role.department().is_none());
        assert!(role.title().is_none());
        assert!(role.start_date().is_none());
        assert!(role.end_date().is_none());
        assert!(role.organization().is_none());
    }

    #[test]
    fn test_default() {
        let role = Role::default();
        assert!(role.department().is_none());
        assert!(role.title().is_none());
        assert!(role.start_date().is_none());
        assert!(role.end_date().is_none());
        assert!(role.organization().is_none());
    }

    #[test]
    fn test_debug_trait() {
        let role = Role::new();
        let debug_str = format!("{:?}", role);
        assert!(debug_str.contains("Role"));
        assert!(debug_str.contains("department: None"));
        assert!(debug_str.contains("title: None"));
        assert!(debug_str.contains("start_date: None"));
        assert!(debug_str.contains("end_date: None"));
        assert!(debug_str.contains("organization: None"));
    }

    #[test]
    fn test_clone_trait() {
        let mut role = Role::new();
        role.set_department(Some("Test Department".to_string()));
        role.set_title(Some("Test Title".to_string()));

        let cloned_role = role.clone();
        assert_eq!(
            cloned_role.department().map(|s| s.as_str()),
            Some("Test Department")
        );
        assert_eq!(cloned_role.title().map(|s| s.as_str()), Some("Test Title"));
        assert!(cloned_role.start_date().is_none());
        assert!(cloned_role.end_date().is_none());
        assert!(cloned_role.organization().is_none());
    }

    #[test]
    fn test_setters_and_getters() {
        let mut role = Role::new();

        // Test department
        role.set_department(Some("Engineering".to_string()));
        assert_eq!(role.department().map(|s| s.as_str()), Some("Engineering"));

        // Test title
        role.set_title(Some("Senior Developer".to_string()));
        assert_eq!(role.title().map(|s| s.as_str()), Some("Senior Developer"));

        // Test setting to None
        role.set_department(None);
        assert!(role.department().is_none());
    }

    #[test]
    fn test_external_ids() {
        // This test ensures Role supports external IDs
        // Currently this functionality is not implemented (TODO)
        let mut role = Role::new();

        // Test default state - no external IDs
        assert_eq!(role.external_ids().len(), 0);

        // Test adding external IDs
        role.add_external_id("grant_number", "GR-2023-12345");
        role.add_external_id("project_id", "PROJ-456");

        assert_eq!(role.external_ids().len(), 2);
        assert_eq!(
            role.external_ids()[0],
            ("grant_number".to_string(), "GR-2023-12345".to_string())
        );
        assert_eq!(
            role.external_ids()[1],
            ("project_id".to_string(), "PROJ-456".to_string())
        );
    }
}
