use crate::traits::Contains;

#[allow(dead_code)]
#[must_use]
pub fn validate_contains<T: Contains>(val: T, needle: &str) -> bool {
    val.has_element(needle)
}

#[cfg(test)]
mod tests {
    use crate::validation::contains::validate_contains;

    #[test]
    fn test_validate_contains_string() {
        assert!(validate_contains("hey".to_string(), "hey"));
        assert!(validate_contains("hey".to_string(), "e"));

        assert!(validate_contains("hey", "hey"));
        assert!(validate_contains("hey", "e"));
    }

    #[test]
    fn test_validate_contains_string_can_fail() {
        assert!(!validate_contains("hey", "o"));
        assert!(!validate_contains("hey".to_string(), "o"));
    }
}
