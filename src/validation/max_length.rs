use crate::traits::HasLength;

#[allow(dead_code)]
#[must_use]
pub fn validate_max_length<T: HasLength>(val: &T, needle: u32) -> bool {
    val.length() <= needle
}

#[cfg(test)]
mod tests {
    use crate::validation::max_length::validate_max_length;

    
    #[test]
    fn test_validate_max() {
        assert!(validate_max_length(&"hello", 10));
        assert!(validate_max_length(&"hello", 5));
        assert!(!validate_max_length(&"hello", 4));
    }
}