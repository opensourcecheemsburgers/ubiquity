use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Cannot get the config folder for the current OS.")]
    OsConfigFolderError,

    #[error("Cannot read config file.")]
    ReadError {
        source: io::Error, 
    },

    #[error("Cannot save config file.")]
    SaveError {
        source: io::Error, 
    },

    #[error("Cannot create config folder.")]
    CreateFolderError,
    
    #[error("Cannot create config file.")]
    CreateFileError,

    #[error("Error serialising TOML to string.")]
    TomlSerError {
        source: toml::ser::Error
    },

    #[error("Error deserialising string to TOML.")]
    TomlDeserError {
        source: figment::Error
    }
}