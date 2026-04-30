//! Shared helper utilities.

use crate::errors::ApiError;

/// Validate collection name: lowercase alphanumeric + underscore, 1-40 chars, no _ prefix
pub fn validate_collection_name(name: &str) -> Result<(), ApiError> {
    if name.is_empty() || name.len() > 40 {
        return Err(ApiError::BadRequest(
            "collection name must be 1-40 characters".into(),
        ));
    }

    if name.starts_with('_') {
        return Err(ApiError::BadRequest(
            "collection names cannot start with underscore (reserved)".into(),
        ));
    }

    let valid = name
        .chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');

    if !valid || !name.chars().next().unwrap().is_ascii_lowercase() {
        return Err(ApiError::BadRequest(
            "collection name must start with lowercase letter and contain only [a-z0-9_]".into(),
        ));
    }

    Ok(())
}
