#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(let_chains)]

use std::{fs, path::PathBuf};

use error::UbiquityError;
use tauri::{generate_context, Manager};

#[cfg(not(target_os = "linux"))]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};
use window_shadows::set_shadow;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, save_file])
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
fn read_file(path: PathBuf) -> Result<String, UbiquityError> {
    Ok(fs::read_to_string(path)?)
}

#[tauri::command]
fn save_file(path: PathBuf, markdown: String) -> Result<(), UbiquityError> {
    Ok(fs::write(path, markdown)?)
}