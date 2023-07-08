use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {    
    #[error(transparent)]
    IoError(#[from] io::Error),

    // #[error("Error deserialising RON to string.")]
    // RonToStringError(#[from] ron::de::Error),

    #[error("Error deserialising RON to string.")]
    RonToStringError(#[from] ron::de::SpannedError),

    #[error("An error occured with RON.")]
    RonError(#[from] ron::Error)
}
