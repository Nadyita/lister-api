use crate::error::{AppError, Result};

const MAX_LENGTH: usize = 200;

/// Validates that a string is not empty and does not exceed MAX_LENGTH
pub fn validate_string(value: &str, field_name: &str) -> Result<()> {
    if value.is_empty() {
        return Err(AppError::BadRequest(format!(
            "{} cannot be empty",
            field_name
        )));
    }

    if value.len() > MAX_LENGTH {
        return Err(AppError::BadRequest(format!(
            "{} must not exceed {} characters (got {})",
            field_name,
            MAX_LENGTH,
            value.len()
        )));
    }

    Ok(())
}

/// Validates an optional string (if present, must not exceed MAX_LENGTH)
pub fn validate_optional_string(value: &Option<String>, field_name: &str) -> Result<()> {
    if let Some(ref s) = value {
        if s.len() > MAX_LENGTH {
            return Err(AppError::BadRequest(format!(
                "{} must not exceed {} characters (got {})",
                field_name,
                MAX_LENGTH,
                s.len()
            )));
        }
    }
    Ok(())
}

