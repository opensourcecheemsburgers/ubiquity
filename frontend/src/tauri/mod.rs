use crate::contexts::markdown::Markdown;
use error::UbiquityError;
use tauri_sys::tauri::invoke;
use yew::AttrValue;
use md::*;

pub async fn save_markdown_to_fs(markdown: Markdown) -> Result<String, UbiquityError> {
    let contents = markdown.text.to_string();
    let path = match markdown.key {
        Some(key) => Some(key.to_string()),
        None => None
    };
    let markdown: &MarkdownFile = &MarkdownFile { path, contents };
    let save_file: Result<String, tauri_sys::error::Error> = invoke("save_file", markdown).await;
    match save_file {
        Ok(path) => Ok(path),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error))   
    }
}

pub async fn read_markdown_from_fs(key: AttrValue) -> Result<String, UbiquityError> {
    let path = key.to_string();
    let markdown_file = &MarkdownPath { path };
    let read_file: Result<String, tauri_sys::error::Error> = invoke("read_file", markdown_file).await;
    match read_file {
        Ok(md) => Ok(md),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error))   
    }
}

pub async fn create_new_markdown_file() -> Result<String, UbiquityError> {
    let contents = String::from("");
    let path = None;
    let markdown: &MarkdownFile = &MarkdownFile { path, contents };
    let save_file: Result<String, tauri_sys::error::Error> = invoke("save_file", markdown).await;
    match save_file {
        Ok(path) => Ok(path),
        Err(tauri_error) => Err(UbiquityError::from(tauri_error))   
    }
}

pub async fn import_markdown_file() -> Result<Markdown, UbiquityError> {
    let save_file: Result<MarkdownFile, tauri_sys::error::Error> = invoke("open_file_dialog", &()).await;

    match save_file {
        Ok(markdown_file) => {
            let key = Some(AttrValue::from(markdown_file.path.unwrap()));
            let text = AttrValue::from(markdown_file.contents);
            let markdown = Markdown {key, text};
            Ok(markdown)
        }
        Err(tauri_error) => {
            Err(UbiquityError::from(tauri_error))
        }
    }
}