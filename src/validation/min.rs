pub fn validate_min<T>(val: T, needle: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    val >= needle
}

#[cfg(test)]
mod tests {
    use crate::validation::min::validate_min;

    #[test]
    fn test_validate_min() {
        assert!(validate_min(5, 5));
        assert!(validate_min(10, 5));

        assert!(validate_min(10.5, 5.0));
        assert!(validate_min(5.0, 5.0));
    }
}
