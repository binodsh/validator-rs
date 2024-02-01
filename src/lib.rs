mod traits;
pub mod validation;

pub use traits::{Contains, Validate};

pub use validate_derive::Validate;

#[derive(Debug)]
pub struct ValidationError {
    code: String,
    message: String,
}

impl ValidationError {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}
