use crate::traits::HasLength;

#[allow(dead_code)]
#[must_use]
pub fn validate_min_length<T: HasLength>(val1: &T, val2: u32) -> bool {
    val1.length() >= val2
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_validate_min() {
        assert!(validate_min_length(&"hello", 1));
        assert!(validate_min_length(&"hello", 5));
        assert!(!validate_min_length(&"hello", 6));
    }
}
