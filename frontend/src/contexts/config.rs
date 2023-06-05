use std::io;
use std::ops::Deref;
use std::path::PathBuf;
use config::{Config, View};
use error::UbiquityError;
use gloo::storage::{LocalStorage, Storage, errors::StorageError};
use yew::prelude::*;

static CONFIG_STORAGE_KEY: &'static str = "config";

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConfigContext {
    inner: UseStateHandle<Config>,
}

impl ConfigContext {
    pub fn new(inner: UseStateHandle<Config>) -> Self {
        Self { inner }
    }

    pub fn state(&self) -> Config {
        self.inner.current().clone()
    }

    pub fn set(&self, config: Config) -> Result<(), UbiquityError> {
        self.inner.set(config.clone());
        self.save(config)?;
        Ok(())
    }

    pub fn set_theme(&self, theme: String) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.theme = theme;
        self.set(new_config)?;
        Ok(())
    }

    pub fn save(&self, config: Config) -> Result<(), UbiquityError> {
        let value = config.to_string()?;
        LocalStorage::set(CONFIG_STORAGE_KEY, value)?;
        Ok(())
    }

    pub fn load_from_storage() -> Result<Config, UbiquityError> {
        let ron_str: String = LocalStorage::get(CONFIG_STORAGE_KEY)?;
        let config: Config = Config::from_str(&ron_str)?;
        Ok(config)
    }

    pub fn toggle_mobile_ui(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.mobile_ui = !self.inner.mobile_ui;
        self.set(new_config)?;
        Ok(())
    }


    pub fn toggle_view(&self) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        match &self.state().view {
            View::Dual => new_config.view = View::Input,
            View::Input | View::Preview => new_config.view = View::Dual,
        };
        self.set(new_config)?;
        Ok(())
    }

    pub fn set_view(&self, view: View) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.view = view;
        self.set(new_config)?;
        Ok(())
    }

    pub fn is_single_view(&self) -> bool {
        match &self.state().view {
            View::Dual => false,
            View::Input | View::Preview => true,
        }
    }

    pub fn set_md_input_font_size(&self, size: String) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.md_input_font_size = size;
        self.set(new_config)?;
        Ok(())
    }

    pub fn increase_font_size(&self) -> Result<(), UbiquityError> {
        let current_size = self.state().md_input_font_size;
        let new_size = match current_size.as_str() {
            "text-xs" => "text-sm",
            "text-sm" => "text-base",
            "text-base" => "text-lg",
            "text-lg" => "text-xl",
            "text-xl" => "text-2xl",
            "text-2xl" => "text-3xl",
            "text-3xl" => "text-4xl",
            "text-4xl" => "text-5xl",
            "text-5xl" => "text-6xl",
            "text-6xl" => "text-7xl",
            "text-7xl" => "text-8xl",
            "text-8xl" => "text-9xl",
            "text-9xl" => "text-9xl",
            _ => "text-base"
        };
        self.set_md_input_font_size(new_size.to_owned())?;
        Ok(())
    }
    
    pub fn decrease_font_size(&self) -> Result<(), UbiquityError> {
        let current_size = self.state().md_input_font_size;
        let new_size = match current_size.as_str() {
            "text-xs" => "text-xs",
            "text-sm" => "text-xs",
            "text-base" => "text-sm",
            "text-lg" => "text-base",
            "text-xl" => "text-lg",
            "text-2xl" => "text-xl",
            "text-3xl" => "text-2xl",
            "text-4xl" => "text-3xl",
            "text-5xl" => "text-4xl",
            "text-6xl" => "text-5xl",
            "text-7xl" => "text-6xl",
            "text-8xl" => "text-7xl",
            "text-9xl" => "text-8xl",
            _ => "text-base"
        };
        self.set_md_input_font_size(new_size.to_owned())?;
        Ok(())
    }

    pub fn set_md_preview_font_size(&self, size: String) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.md_preview_font_size = size;
        self.set(new_config)?;
        Ok(())
    }

    pub fn increase_preview_font_size(&self) -> Result<(), UbiquityError> {
        let current_size = self.state().md_preview_font_size; 
        let new_size = match current_size.as_str() {
            "prose-sm" => "prose-base",
            "prose-base" => "prose-lg",
            "prose-lg" => "prose-xl",
            "prose-xl" => "prose-2xl",
            "prose-2xl" => "prose-2xl",
            _ => "prose-base"
        };
        self.set_md_preview_font_size(new_size.to_owned())?;
        Ok(())
    }
    
    pub fn decrease_preview_font_size(&self) -> Result<(), UbiquityError> {
        let current_size = self.state().md_preview_font_size;
        let new_size = match current_size.as_str() {
            "prose-sm" => "prose-sm",
            "prose-base" => "prose-sm",
            "prose-lg" => "prose-base",
            "prose-xl" => "prose-lg",
            "prose-2xl" => "prose-xl",
            _ => "prose-base"
        };
        self.set_md_preview_font_size(new_size.to_owned())?;
        Ok(())
    }

    pub fn set_data_path(&mut self, path: PathBuf) -> Result<(), UbiquityError> {
        let mut new_config = self.state();
        new_config.data_path = Some(path);
        self.set(new_config)?;
        Ok(())
    }
}

impl Deref for ConfigContext {
    type Target = Config;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ConfigProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn ConfigProvider(props: &ConfigProviderProps) -> Html {
    let config = ConfigContext::load_from_storage().unwrap_or_default();
    let config_state = use_state(|| config);
    let config_context = ConfigContext::new(config_state);

    html! {
        <ContextProvider<ConfigContext> context={config_context}>
            {props.children.clone()}
        </ContextProvider<ConfigContext>>
    }
}

#[hook]
pub(crate) fn use_config() -> ConfigContext {
    use_context::<ConfigContext>().unwrap()
}

pub const THEMES: &'static [&'static str] = &[
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
];