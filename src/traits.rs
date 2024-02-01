use crate::ValidationError;

pub trait Validate {
    fn validate(&self) -> Result<(), Vec<ValidationError>>;
}

pub trait Contains {
    #[must_use]
    fn has_element(&self, value: &str) -> bool;
}

pub trait HasLength {
    #[must_use]
    fn length(&self) -> u32;
}

pub trait MaxLength {
    #[must_use]
    fn is_max(&self, value: i32) -> bool;
}

impl Contains for String {
    fn has_element(&self, value: &str) -> bool {
        self.contains(value)
    }
}

impl Contains for str {
    fn has_element(&self, value: &str) -> bool {
        self.contains(value)
    }
}

impl HasLength for String {
    fn length(&self) -> u32 {
        self.len() as u32
    }
}

impl HasLength for str {
    fn length(&self) -> u32 {
        self.len() as u32
    }
}
