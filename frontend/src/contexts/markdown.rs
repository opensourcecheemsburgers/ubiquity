use std::ops::Deref;
use std::path::PathBuf;
use error::UbiquityError;
use gloo::console::log;
use gloo::storage::errors::StorageError;
use serde::Deserialize;
use serde::Serialize;
use yew::platform::spawn_local;
use yew::prelude::*;
use gloo::storage::LocalStorage;
use gloo::storage::Storage;

#[derive(Clone, Debug, PartialEq)]
pub struct Markdown {
    pub text: AttrValue,
    pub key: AttrValue
}

impl Markdown {    
    fn from(text: AttrValue, key: AttrValue) -> Self {
        Self { text, key }
    }

    fn current(&self) -> &Self {
        &self
    }

    fn load_from_storage() -> Option<Markdown> {
        let load_result: Result<String, StorageError> = LocalStorage::get(DOCS_KEY);
        
        match load_result {
            Ok(item) => {
                let text = AttrValue::from(item);
                Some(Markdown::from(text, DOCS_KEY))
            }, 
            Err(err) => {
                log!("Err: {}", err.to_string());
                None
            }
        }
    }
}

impl Default for Markdown {
    fn default() -> Self {
        DOCS_MD
    }
}


impl MarkdownContext {
    pub fn new(inner: UseStateHandle<Markdown>) -> Self {
        Self { inner }
    }

    pub fn update_markdown(&self, text: AttrValue) -> Result<(), UbiquityError> {
        let markdown = Markdown::from(text.clone(), DOCS_KEY);
        self.inner.set(markdown);
        self.save_to_browser_storage(text)?;
        Ok(())
    }

    pub fn save_to_browser_storage(&self, text: AttrValue) -> Result<(), UbiquityError> {
        let key_str = self.key.as_str();
        let text_str = text.as_str();
        LocalStorage::set(key_str, text_str)?;
        Ok(())
    }

    #[cfg(feature = "desktop")]
    pub async fn save_to_fs(&self) -> Result<(), UbiquityError> {
        let ctx_clone = self.clone();
        spawn_local(async move {
            let mut file_dialog = tauri_sys::dialog::FileDialogBuilder::new();
            file_dialog.set_title("Save Markdown");
            let path = file_dialog.save().await.unwrap().unwrap();
            let markdown = ctx_clone.state().text.to_string();
            let save_md: () = tauri_sys::tauri::invoke("save_file", &MarkdownFile {path, markdown}).await.unwrap();
        });
        Ok(())
    }

    #[cfg(feature = "desktop")]
    pub async fn read_from_fs(&self) -> Result<(), UbiquityError> {
        let ctx_clone = self.clone();
        spawn_local(async move {
            let mut file_dialog = tauri_sys::dialog::FileDialogBuilder::new();
            file_dialog.set_title("Choose Markdown File");
            file_dialog.add_filter("Text", &["md", "txt", "markdown", "text"]);
            let path = file_dialog.pick_file().await.unwrap().unwrap();
            
            let read_file: String = tauri_sys::tauri::invoke("read_file", &MarkdownPath {path}).await.unwrap();
            let text: AttrValue = AttrValue::from(read_file);
            let key: AttrValue = AttrValue::from("read_file_md");
            ctx_clone.add_markdown(Markdown::from(text, key));
        });
        Ok(())
    }

    pub fn add_markdown(&self, markdown: Markdown) {
        self.inner.set(markdown);
    }

    pub fn state(&self) -> Markdown {
        self.inner.current().clone()
    }
}

impl Deref for MarkdownContext {
    type Target = Markdown;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for MarkdownContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MarkdownContext {
   inner: UseStateHandle<Markdown>,
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct MarkdownProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn MarkdownProvider(props: &MarkdownProviderProps) -> Html {
    let markdown = Markdown::load_from_storage().unwrap_or_default();
    let markdown_state = use_state(|| markdown);
    let markdown_context = MarkdownContext::new(markdown_state);

    html! {
        <ContextProvider<MarkdownContext> context={markdown_context}>
            {props.children.clone()}
        </ContextProvider<MarkdownContext>>
    }
}

#[hook]
pub(crate) fn use_markdown() -> MarkdownContext {
    use_context::<MarkdownContext>().unwrap()
}
#[derive(Deserialize, Serialize)]
struct MarkdownFile {
    path: PathBuf,
    markdown: String
}

#[derive(Deserialize, Serialize)]
struct MarkdownPath {
    path: PathBuf,
}


#[derive(Deserialize)]
pub struct BrowserStorageItem {
    pub key: String,
    pub value: String
}

const DOCS_MD: Markdown = Markdown { key: DOCS_KEY, text: DOCS_STR };
const DOCS_KEY: AttrValue = AttrValue::Static("user_markdown_key");
const DOCS_STR: AttrValue = AttrValue::Static(r#"# Ubiquity

Ubiquity is a free and [open-source](https://github.com/opensourcecheemsburgers/ubiquity) markdown editor for Windows, Linux and Mac.

## Author

Stephen Power is the main developer of Ubiquity. Check out his [website](https://www.stephenpower.ie:443/).

## Language

Ubiquity is written in Rust.
![Ferris Gif](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif "*SNAP* I got your finger :)")

Ubiquity utilises two Rust frameworks.

* [Yew](https://www.yew.rs) - A web application framework.
* [Tauri](https://www.tauri.app) - A desktop application framework.

## UI Design

Ubiquity use two main components for UI design.

1. [Tailwind](https://www.tailwindcss.com) - A CSS framework.
2. [DaisyUI](https://www.daisyui.com) - A Tailwind CSS component library.

## Fonts

Ubiquity uses three different fonts.

- Inter[^1]
- Comfortaa[^2]
- Inconsolata[^3]

## File Size

A note from Stephen Power, the developer of Ubiquity:
> Ubiquity use multiple different techniques for file size optimisation.
>
> For example, platform-specific Rust standard library recompilation, debug stripping, LLVM link time optimisations, WASM file size optimisation, and more.

### Sample File Size Optimisation Code

***
```rust
[profile.release]
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```
***

### Platform File Size Table

| Platform | Zipped | Unzipped  |
| --------- | ----- | ----- |
| Web       | 1.4MB | 3.3MB |
| Windows   | 1.4MB | 3.3MB |
| Linux     | 1.4MB | 3.3MB |
| Mac       | 1.4MB | 3.3MB |
| Android   | 1.4MB | 3.3MB |
| iOS       | 1.4MB | 3.3MB |

**Note:** These numbers are fake.

## Contributions

Monero:`86ywTu1eDABJB2CxGf9mwMbX6ibfc7pga1XusQCrfvMYdNpNPto1BaDSbWeas13Nkg3iLigkzS7wSGc8YoQxEkHhHdrA4Ni`

Bitcoin: `bc1q8r90zc8j8a2rvkq4ds8374pxh3rpccxgnjx5x2`

Ethereum: `0xf7107aB4765aD2D97DeeE7AeFEA221ddD9247d7D`


[^1]: [Inter](https://fonts.google.com/specimen/Inter) is a sans-serif font designed by Google.
[^2]: [Comfortaa](https://fonts.google.com/specimen/Comfortaa) is a display font designed by Google.
[^3]: [Inconsolata](https://fonts.google.com/specimen/Inconsolata) is a monospace font designed by Google."#); 