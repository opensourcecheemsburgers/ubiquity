use dirs::{config_dir, data_dir};
use ::error::UbiquityError;
use ron::ser::PrettyConfig;

use std::fs;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod error;

#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub theme: String,
    pub md_input_font_size: String,
    pub md_preview_font_size: String,
    pub mobile_ui: bool,
    pub data_path: Option<PathBuf>,
    pub view: View
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: "synthwave".to_string(),
            data_path: data_dir(),
            md_input_font_size: String::from("text-base"),
            md_preview_font_size: String::from("prose-base"),
            mobile_ui: false,
            view: View::Dual,
        }
    }
}



#[derive(Clone, Deserialize, Serialize, Debug, PartialEq)]
pub enum View {
    Dual,
    Input,
    Preview
}

impl Config {
    pub fn mobile() -> Self {
        Self {
            theme: "synthwave".to_string(),
            data_path: data_dir(),
            md_input_font_size: String::from("text-base"),
            md_preview_font_size: String::from("prose-base"),
            mobile_ui: true,
            view: View::Input,
        }
    }

    pub fn save(&self) -> Result<(), UbiquityError> {
        let pretty_ron_config = PrettyConfig::default();
        let ron_string = ron::ser::to_string_pretty(self, pretty_ron_config)?;
        write_config_file(&ron_string)?;
        Ok(())
    }

    pub fn from_str(ron_str: &str) -> Result<Self, UbiquityError> {
        let config: Config = ron::from_str(&ron_str)?;
        Ok(config)
    }

    pub fn to_string(&self) -> Result<String, UbiquityError> {
        let pretty_ron_config = PrettyConfig::default();
        Ok(ron::ser::to_string_pretty(self, pretty_ron_config)?)
    }

    pub fn load(&mut self) -> Result<(), UbiquityError> {
        let config = read_config_file()?;
        *self = config;
        Ok(())
    }

    pub fn init() -> Result<(), UbiquityError> {
        let config = Self::default();
        config.save()?;
        Ok(())
    }

    pub fn current(&self) -> &Self {
        &self
    }
}

pub fn read_config_file() -> Result<Config, UbiquityError> {
    let path = get_config_file()?;
    let config_str = fs::read_to_string(path)?;
    let config: Config = ron::from_str(&config_str)?;
    Ok(config)
}

pub fn write_config_file(ron_string: &str) -> Result<(), UbiquityError> {
    let path = get_config_file()?;
    fs::write(path, ron_string)?;
    Ok(())
}

pub fn get_config_file() -> Result<PathBuf, UbiquityError> {
    let mut path = get_config_folder()?;
    path.push("config.ron");
    match path.exists() {
        true => Ok(path),
        false => {
             fs::write(path.clone(), "")?;
             Ok(path)
        }
    }
}

pub fn get_config_folder() -> Result<PathBuf, UbiquityError> {
    match config_dir() {
        Some(mut path) => {
            path.push("ubiquity/");
            fs::create_dir(&path)?;
            Ok(path)
        },
        None => Err(UbiquityError::no_config_folder()),
    }
}