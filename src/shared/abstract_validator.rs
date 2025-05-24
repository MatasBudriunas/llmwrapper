use std::fmt::Display;

pub fn validate_required_str(field: &str, field_name: &str) -> Result<(), String> {
    if field.trim().is_empty() {
        Err(format!("{} cannot be empty.", field_name))
    } else {
        Ok(())
    }
}

pub fn validate_max<T: PartialOrd + Copy + Display>(value: T, max: T, field_name: &str) -> Result<(), String> {
    if value > max {
        Err(format!("{} must be <= {}", field_name, max))
    } else {
        Ok(())
    }
}

pub fn validate_min<T: PartialOrd + Copy + Display>(value: T, min: T, field_name: &str) -> Result<(), String> {
    if value < min {
        Err(format!("{} must be >= {}", field_name, min))
    } else {
        Ok(())
    }
}
