#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(let_chains)]

use std::{fs, path::PathBuf};

use error::UbiquityError;
use tauri::{generate_context, Manager};
use rfd::{AsyncFileDialog, FileDialog};
use md::*;

#[cfg(not(target_os = "linux"))]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, save_file, open_file_dialog])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            // #[cfg(target_os = "macos")]
            // apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            // .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            // #[cfg(target_os = "windows")]
            // apply_blur(&window, Some((18, 18, 18, 125)))
            // .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn save_file(path: Option<String>, contents: String) -> Result<String, UbiquityError> {
    if let Some(path_key) = path && !path_key.eq(&DOCS_KEY) {
      let path = PathBuf::from(path_key.clone());
      save_to_fs(path, contents);
      Ok(path_key)

    } else {
        let mut dir = PathBuf::from("/");
        if let Some(docs_dir) = dirs::document_dir() {
          dir = docs_dir;
        }
        let file_dialog = FileDialog::new().set_directory(dir).save_file();
    
        match file_dialog {
          Some(file_handle) => {
            fs::write(file_handle.clone(), contents)?;
            Ok(file_handle.to_str().unwrap().to_string())
          },
          None => Err(UbiquityError::no_save_path_selected()),
      }
  }
}

#[tauri::command]
fn open_file_dialog() -> Result<MarkdownFile, UbiquityError> {
    let mut dir = PathBuf::from("/");
    if let Some(docs_dir) = dirs::document_dir() {
      dir = docs_dir;
    }
    let file_dialog_res = FileDialog::new().set_directory(dir).pick_file();

    if let Some(file_handle) = file_dialog_res {
        let contents = read_from_fs(file_handle.clone())?;
        let path = Some(file_handle.to_str().unwrap().to_string());
        let markdown_file = MarkdownFile { path, contents};
        Ok(markdown_file)
    } else {
      Err(UbiquityError::no_file_selected())
    }
}

#[tauri::command]
fn read_file(path: String) -> Result<String, UbiquityError> {
    Ok(fs::read_to_string(PathBuf::from(path))?)
}

fn read_from_fs(path: PathBuf) -> Result<String, UbiquityError> {
  Ok(fs::read_to_string(PathBuf::from(path))?)
}

fn save_to_fs(path: PathBuf, contents: String) -> Result<(), UbiquityError> {
    Ok(fs::write(path, contents)?)
}

#[tauri::command]
fn auto_save(path: PathBuf, contents: String) -> Result<(), UbiquityError> {
    Ok(fs::write(path, contents)?)
}