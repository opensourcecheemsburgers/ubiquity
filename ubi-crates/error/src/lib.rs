use std::{fmt::{Formatter, Display, self, Debug}, error::Error, io};

use gloo::storage::errors::StorageError;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UbiquityError {
    title: String,
    human_description: String,
    verbose_description: Option<String>
}

impl Display for UbiquityError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match &self.verbose_description {
            Some(verbose_description) => write!(f, "An error has occured in Ubiquity:/n/n{}/n/n{}/n/n{}", self.title, self.human_description, verbose_description),
            None => write!(f, "An error has occured in Ubiquity:/n/n{}/n/n{}", self.title, self.human_description),
        }
    }
}

impl Default for UbiquityError {
    fn default() -> Self {
        let title: String = String::from("Unknown Error");
        let human_description: String = String::from("An error has occured."); 
        let verbose_description: Option<String> = Some(String::from("Error code: 492068617665206E6F20636C75652077686174206A7573742068617070656E65642E204920646F6E27742063617265206569746865722E20537570706F727420746869732070726F6A65637420616E642049202A6D696768742A206669782069742E"));       
        Self { title, human_description, verbose_description }
    }
}

impl From<io::Error> for UbiquityError {
    fn from(io_error: io::Error) -> Self {
        let title = String::from("I/O Error");
        let human_description = io_error.kind().to_string();
        let verbose_description = None;
        Self { title, human_description, verbose_description }
    }
}

impl From<StorageError> for UbiquityError {
    fn from(storage_error: StorageError) -> Self {
        let title = String::from("Browser Storage Error");
        let human_description = storage_error.to_string();
        let verbose_description = None;
        Self { title, human_description, verbose_description }
    }
}

impl From<ron::Error> for UbiquityError {
    fn from(ron_error: ron::Error) -> Self {
        let title = String::from("RON Error");
        let human_description = String::from("An error occured with RON.");
        let verbose_description = Some(ron_error.to_string());
        Self { title, human_description, verbose_description }
    }
}

impl From<ron::error::SpannedError> for UbiquityError {
    fn from(ron_error: ron::error::SpannedError) -> Self {
        let title = String::from("RON Error");
        let human_description = String::from("An error occured with RON.");
        let verbose_description = Some(ron_error.to_string());
        Self { title, human_description, verbose_description }
    }
}

impl UbiquityError {
    pub fn no_config_folder() -> Self {
        let title = String::from("Config Folder Error");
        let human_description = String::from("Ubiquity could not find your operating system's default configuration folder.");
        let verbose_description = None;
        Self { title, human_description, verbose_description }
    }

    pub fn empty_config_ctx() -> Self {
        let title = String::from("Context Error");
        let human_description = String::from("The config context is empty.");
        let verbose_description = None;
        Self { title, human_description, verbose_description }
    }

    pub fn empty_md_ctx() -> Self {
        let title = String::from("Context Error");
        let human_description = String::from("The markdown context is empty.");
        let verbose_description = None;
        Self { title, human_description, verbose_description }
    }
}