use crate::date::Date;
use crate::organization::Organization;

#[derive(Debug, Clone)]
pub struct Role {
    pub department: Option<String>,
    pub title: Option<String>,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub organization: Option<Organization>,
}

impl Role {
    pub fn new() -> Self {
        Self {
            department: None,
            title: None,
            start_date: None,
            end_date: None,
            organization: None,
        }
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
        assert!(role.department.is_none());
        assert!(role.title.is_none());
        assert!(role.start_date.is_none());
        assert!(role.end_date.is_none());
        assert!(role.organization.is_none());
    }

    #[test]
    fn test_default() {
        let role = Role::default();
        assert!(role.department.is_none());
        assert!(role.title.is_none());
        assert!(role.start_date.is_none());
        assert!(role.end_date.is_none());
        assert!(role.organization.is_none());
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
        role.department = Some("Test Department".to_string());
        role.title = Some("Test Title".to_string());

        let cloned_role = role.clone();
        assert_eq!(cloned_role.department, Some("Test Department".to_string()));
        assert_eq!(cloned_role.title, Some("Test Title".to_string()));
        assert!(cloned_role.start_date.is_none());
        assert!(cloned_role.end_date.is_none());
        assert!(cloned_role.organization.is_none());
    }
}
