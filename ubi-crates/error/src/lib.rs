use std::{fmt::{Formatter, Display, self, Debug}, error::Error};

#[derive(Debug, Clone, PartialEq)]
pub struct UbiquityError {
    description: String,
}

impl Display for UbiquityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.description, f)
    }
}

impl Default for UbiquityError {
    fn default() -> Self {
        let description: String = "An unknown error has occurred in Ubiquity.".to_string();        
        Self { description }
    }
}

impl UbiquityError {
    pub fn from_error(error: Box<dyn Error>) -> Self {
        Self { description: error.to_string() }
    }
}