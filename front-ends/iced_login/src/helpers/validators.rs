use validator::ValidationError;

/// This function is called by the validator to validate that a password respects the app's requirements
pub fn validate_password(pw: &str) -> Result<(), ValidationError> {
    if pw.len() < 8 {
        return Err(
            ValidationError::new("validation_password_short"));
    }

    if pw.len() > 63 {
        return Err(
            ValidationError::new("validation_password_long"));
    }

    if !pw.chars().any(|c| c.is_ascii_uppercase()) {
        return Err(ValidationError::new("validation_password_uppercase"));
    }
    if !pw.chars().any(|c| c.is_ascii_lowercase()) {
        return Err(ValidationError::new("validation_password_lowercase"));
    }
    if !pw.chars().any(|c| c.is_ascii_digit()) {
        return Err(ValidationError::new("validation_password_digit"));
    }

    Ok(())
}
