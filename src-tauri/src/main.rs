#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(let_chains)]


use std::{
    cmp::min,
    io::{Read, Seek, SeekFrom},
    rc::Rc, ffi::OsStr,
};

use tauri::{
    Manager, http::{HttpRange, ResponseBuilder}, generate_context,
};
use walkdir::WalkDir;

#[cfg(not(target_os = "linux"))]
use window_vibrancy::{apply_blur, apply_vibrancy, NSVisualEffectMaterial};

use window_shadows::set_shadow;

pub struct PlayerState {
    pub playing: bool,
    pub vol: i32,
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let window = app.get_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

            #[cfg(target_os = "windows")]
            apply_blur(&window, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
            
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&window, true).unwrap();

            Ok(())
        })
        .run(generate_context!())
        .expect("error while running tauri application");
}