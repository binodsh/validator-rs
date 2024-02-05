pub fn validate_max<T>(val: T, needle: T) -> bool
where
    T: PartialOrd + PartialEq,
{
    val <= needle
}

#[cfg(test)]
mod tests {
    use crate::validation::max::validate_max;

    #[test]
    fn test_validate_min() {
        assert!(validate_max(5, 5));
        assert!(validate_max(1, 5));

        assert!(validate_max(5.0, 5.0));
        assert!(validate_max(1.0, 5.0));
    }
}
