use crate::traits::HasLength;

/// Validates whether the value contains the needle
/// The value needs to implement the Contains trait, which is implement on String, str and Hashmap<String>
/// by default.

#[allow(dead_code)]
#[must_use]
pub fn validate_max_length<T: HasLength>(val: &T, needle: u32) -> bool {
    val.length() <= needle
}

#[cfg(test)]
mod tests {
    // use std::borrow::Cow;
    // use std::collections::HashMap;

    // use super::*;

    // #[test]
    // fn test_validate_min() {
    //     assert!(validate_min(32, 20));
    // }

}