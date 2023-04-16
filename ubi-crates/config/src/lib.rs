

use dirs::{config_dir, data_dir};
use figment::{
    providers::{Format, Serialized, Toml},
    Figment,
};

use std::fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use crate::error::ConfigError;

mod error;

#[derive(Clone, Deserialize, Serialize)]
pub struct Config {
    pub theme: String,
    pub data_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "synthwave".to_string(),
            data_path: data_dir(),
        }
    }
}

impl Config {
    pub fn save(&self) -> Result<(), ConfigError> {
        let toml_string = get_config_toml_string(&self)?;
        write_config_file(toml_string)?;
        Ok(())
    }

    pub fn load(&mut self) -> Result<(), ConfigError> {
        let config_path = get_config_file()?;
        self.save()?;
        let fig = Figment::new()
            .merge(Serialized::defaults(Config::default()))
            .merge(Toml::file(config_path.as_path()));
        let config = read_config_toml(fig)?;
        *self = config;
        Ok(())
    }

    pub fn init() -> Result<(), ConfigError> {
        let config = Self::default();
        config.save()?;
        Ok(())
    }
}

pub fn get_config_file() -> Result<PathBuf, ConfigError> {
    let mut path = get_config_folder()?;
    path.push("config.toml");
    match path.exists() {
        true => Ok(path),
        false => {
            match init_config_file(path) {
                Ok(path) => Ok(path),
                Err(err) => Err(err)
            }
        }
    }
}

pub fn get_config_folder() -> Result<PathBuf, ConfigError> {
    match config_dir() {
        Some(mut path) => {
            path.push("ubiquity/");
            match path.exists() {
                true => Ok(path),
                false => {
                    match fs::create_dir(&path) {
                        Ok(_) => Ok(path),
                        Err(_) => Err(ConfigError::CreateFolderError)
                    }
                },
            }
        },
        None => Err(ConfigError::OsConfigFolderError),
    }
}

pub fn get_config_toml_string(config: &Config) -> Result<String, ConfigError> {
    match toml::to_string_pretty(config) {
        Ok(str) => Ok(str),
        Err(source) => Err(ConfigError::TomlSerError { source })
    }
}

pub fn read_config_toml(fig: Figment) -> Result<Config, ConfigError> {
    let config_res: Result<Config, figment::Error> = fig.extract();
    match config_res {
        Ok(config) => Ok(config),
        Err(source) => Err(ConfigError::TomlDeserError { source }),
    }
}

pub fn write_config_file(contents: String) -> Result<(), ConfigError> {
    let path = get_config_file()?;
    match std::fs::write(path, contents) {
        Ok(_) => Ok(()),
        Err(source) => Err(ConfigError::SaveError { source }),
    }
}

pub fn init_config_file(path: PathBuf) -> Result<PathBuf, ConfigError> {
    match fs::write(&path, "") {
            Ok(_) => Ok(path),
            Err(_) => Err(ConfigError::CreateFileError)
    }
}