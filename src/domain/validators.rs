use crate::domain::errors::DomainError::{self, BusinessRule};

pub struct CommonValidator;

impl CommonValidator {
    pub fn validate_id(id: &str) -> Result<(), DomainError> {
        if id.is_empty() {
            return Err(BusinessRule("ID cannot be empty".to_string()));
        }
        if uuid::Uuid::parse_str(id).is_err() {
            return Err(BusinessRule("ID must be valid UUID".to_string()));
        }
        Ok(())
    }

    pub fn validate_name(name: &str, field_name: &str) -> Result<(), DomainError> {
        let name = name.trim();

        if name.is_empty() {
            return Err(BusinessRule(format!("{} name cannot be empty", field_name)));
        }

        if name.len() > 50 {
            return Err(BusinessRule(format!(
                "{} name cannot exceed 50 characters",
                field_name
            )));
        }

        if name.chars().all(|c| c.is_whitespace()) {
            return Err(BusinessRule(format!(
                "{} name cannot consist only of whitespace",
                field_name
            )));
        }

        let forbidden_chars = ['<', '>', '&', '"', '\''];
        if name.chars().any(|c| forbidden_chars.contains(&c)) {
            return Err(BusinessRule(format!(
                "{} name contains invalid characters",
                field_name
            )));
        }

        if name.contains("  ") {
            return Err(BusinessRule(format!(
                "{} name cannot contain multiple spaces and a rows",
                field_name
            )));
        }

        Ok(())
    }

    pub fn validate_description(desc: &str, max_length: usize) -> Result<(), DomainError> {
        let desc = desc.trim();

        if desc.len() > max_length {
            return Err(BusinessRule(format!(
                "Description cannot exceed {} characters",
                max_length
            )));
        }
        Ok(())
    }
}
