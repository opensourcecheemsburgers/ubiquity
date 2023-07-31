//! The path module provides utilities for working with file and directory paths.
//! 
//! The APIs must be added to tauri.allowlist.path in tauri.conf.json:
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "path": {
//!                 "all": true, // enable all Path APIs
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use std::path::PathBuf;
use wasm_bindgen::JsValue;

/// Returns the path to the suggested directory for your app's config files.
///
/// Resolves to `${configDir}/${bundleIdentifier}`, where `bundleIdentifier` is the value [`tauri.bundle.identifier`](https://tauri.app/v1/api/config/#bundleconfig.identifier) is configured in `tauri.conf.json`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::app_config_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_config_dir_path = app_config_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn app_config_dir() -> crate::Result<PathBuf> {
    let raw = inner::appConfigDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the suggested directory for your app's data files.
///
/// Resolves to `${dataDir}/${bundleIdentifier}`, where `bundleIdentifier` is the value [`tauri.bundle.identifier`](https://tauri.app/v1/api/config/#bundleconfig.identifier) is configured in `tauri.conf.json`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::app_data_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_data_dir_path = app_data_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn app_data_dir() -> crate::Result<PathBuf> {
    let raw = inner::appDataDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the suggested directory for your app's local data files.
///
/// Resolves to `${localDataDir}/${bundleIdentifier}`, where `bundleIdentifier` is the value [`tauri.bundle.identifier`](https://tauri.app/v1/api/config/#bundleconfig.identifier) is configured in `tauri.conf.json`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::app_local_data_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_local_data_dir_path = app_local_data_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn app_local_data_dir() -> crate::Result<PathBuf> {
    let raw = inner::appLocalDataDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the suggested directory for your app's cache files.
///
/// Resolves to `${cacheDir}/${bundleIdentifier}`, where `bundleIdentifier` is the value [`tauri.bundle.identifier`](https://tauri.app/v1/api/config/#bundleconfig.identifier) is configured in `tauri.conf.json`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::app_cache_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_cache_dir_path = app_cache_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn app_cache_dir() -> crate::Result<PathBuf> {
    let raw = inner::appCacheDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's audio directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_MUSIC_DIR`.
/// - **macOS:** Resolves to `$HOME/Music`.
/// - **Windows:** Resolves to `{FOLDERID_Music}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::audio_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let audio_dir_path = audio_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn audio_dir() -> crate::Result<PathBuf> {
    let raw = inner::audioDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's cache directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_CACHE_HOME` or `$HOME/.cache`.
/// - **macOS:** Resolves to `$HOME/Library/Caches`.
/// - **Windows:** Resolves to `{FOLDERID_LocalAppData}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::cache_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let cache_dir_path = cache_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn cache_dir() -> crate::Result<PathBuf> {
    let raw = inner::cacheDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's config directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_CONFIG_HOME` or `$HOME/.config`.
/// - **macOS:** Resolves to `$HOME/Library/Application Support`.
/// - **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::config_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let config_dir_path = config_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn config_dir() -> crate::Result<PathBuf> {
    let raw = inner::configDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's data directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
/// - **macOS:** Resolves to `$HOME/Library/Application Support`.
/// - **Windows:** Resolves to `{FOLDERID_RoamingAppData}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::data_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let data_dir_path = data_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn data_dir() -> crate::Result<PathBuf> {
    let raw = inner::dataDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's desktop directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DESKTOP_DIR`.
/// - **macOS:** Resolves to `$HOME/Desktop`.
/// - **Windows:** Resolves to `{FOLDERID_Desktop}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::desktop_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let desktop_dir_path = desktop_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn desktop_dir() -> crate::Result<PathBuf> {
    let raw = inner::desktopDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's document directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DOCUMENTS_DIR`.
/// - **macOS:** Resolves to `$HOME/Documents`.
/// - **Windows:** Resolves to `{FOLDERID_Documents}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::document_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let document_dir_path = document_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn document_dir() -> crate::Result<PathBuf> {
    let raw = inner::documentDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's download directory.
///
/// #### Platform-specific
///
/// - **Linux**: Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_DOWNLOAD_DIR`.
/// - **macOS**: Resolves to `$HOME/Downloads`.
/// - **Windows**: Resolves to `{FOLDERID_Downloads}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::download_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let download_dir_path = download_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn download_dir() -> crate::Result<PathBuf> {
    let raw = inner::downloadDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's executable directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_BIN_HOME/../bin` or `$XDG_DATA_HOME/../bin` or `$HOME/.local/bin`.
/// - **macOS:** Not supported.
/// - **Windows:** Not supported.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::executable_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let executable_dir_path = executable_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn executable_dir() -> crate::Result<PathBuf> {
    let raw = inner::executableDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's font directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_DATA_HOME/fonts` or `$HOME/.local/share/fonts`.
/// - **macOS:** Resolves to `$HOME/Library/Fonts`.
/// - **Windows:** Not supported.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::font_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let font_dir_path = font_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn font_dir() -> crate::Result<PathBuf> {
    let raw = inner::fontDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's home directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$HOME`.
/// - **macOS:** Resolves to `$HOME`.
/// - **Windows:** Resolves to `{FOLDERID_Profile}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::home_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let home_dir_path = home_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn home_dir() -> crate::Result<PathBuf> {
    let raw = inner::homeDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's local data directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_DATA_HOME` or `$HOME/.local/share`.
/// - **macOS:** Resolves to `$HOME/Library/Application Support`.
/// - **Windows:** Resolves to `{FOLDERID_LocalAppData}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::local_data_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let local_data_dir_path = local_data_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn local_data_dir() -> crate::Result<PathBuf> {
    let raw = inner::localDataDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's picture directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_PICTURES_DIR`.
/// - **macOS:** Resolves to `$HOME/Pictures`.
/// - **Windows:** Resolves to `{FOLDERID_Pictures}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::picture_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let picture_dir_path = picture_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn picture_dir() -> crate::Result<PathBuf> {
    let raw = inner::pictureDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's public directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_PUBLICSHARE_DIR`.
/// - **macOS:** Resolves to `$HOME/Public`.
/// - **Windows:** Resolves to `{FOLDERID_Public}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::public_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let public_dir_path = public_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn public_dir() -> crate::Result<PathBuf> {
    let raw = inner::publicDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the application's resource directory.
///
/// To resolve a resource path, see the [`resolve_resource`] function.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::resource_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let resource_dir_path = resource_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn resource_dir() -> crate::Result<PathBuf> {
    let raw = inner::resourceDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Resolve the path to a resource file.
///
/// @param resourcePath The path to the resource.
/// Must follow the same syntax as defined in `tauri.conf.json > tauri > bundle > resources`, i.e. keeping subfolders and parent dir components (`../`).
/// @returns The full path to the resource.
///
/// ```rust,no_run
/// use tauri_sys::path::resolve_resource;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let resource_path = resolve_resource("script.sh").await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn resolve_resource(resource_path: &str) -> crate::Result<PathBuf> {
    let raw = inner::resolveResource(JsValue::from_str(resource_path)).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's runtime directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `$XDG_RUNTIME_DIR`.
/// - **macOS:** Not supported.
/// - **Windows:** Not supported.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::runtime_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let runtime_dir_path = runtime_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn runtime_dir() -> crate::Result<PathBuf> {
    let raw = inner::runtimeDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's template directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_TEMPLATES_DIR`.
/// - **macOS:** Not supported.
/// - **Windows:** Resolves to `{FOLDERID_Templates}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::template_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let template_dir_path = template_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn template_dir() -> crate::Result<PathBuf> {
    let raw = inner::templateDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the user's video directory.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to [`xdg-user-dirs`](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/)' `XDG_VIDEOS_DIR`.
/// - **macOS:** Resolves to `$HOME/Movies`.
/// - **Windows:** Resolves to `{FOLDERID_Videos}`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::video_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let video_dir_path = video_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn video_dir() -> crate::Result<PathBuf> {
    let raw = inner::videoDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the path to the suggested directory for your app's log files.
///
/// #### Platform-specific
///
/// - **Linux:** Resolves to `${configDir}/${bundleIdentifier}/logs`.
/// - **macOS:** Resolves to `${homeDir}/Library/Logs/{bundleIdentifier}`
/// - **Windows:** Resolves to `${configDir}/${bundleIdentifier}/logs`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::app_log_dir;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_log_dir_path = app_log_dir().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn app_log_dir() -> crate::Result<PathBuf> {
    let raw = inner::appLogDir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Resolves a sequence of `paths` or `path` segments into an absolute path.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{resolve, app_data_dir};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_data_dir_path = app_data_dir().await?;
///
/// let path = resolve([app_data_dir_path, "..", "users", "tauri", "avatar.png"]).await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn resolve(paths: impl IntoIterator<Item = &str>) -> crate::Result<PathBuf> {
    let paths = paths.into_iter();
    let raw = inner::resolve(serde_wasm_bindgen::to_value(&paths.collect::<Vec<_>>())?).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Normalizes the given `path`, resolving `'..'` and `'.'` segments and resolve symbolic links.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{normalize, app_data_dir};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_data_dir_path = app_data_dir().await?;
///
/// let path = normalize([app_data_dir_path, "..", "users", "tauri", "avatar.png"]).await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn normalize(path: &str) -> crate::Result<PathBuf> {
    let raw = inner::normalize(JsValue::from_str(path)).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

///  Joins all given `path` segments together using the platform-specific separator as a delimiter, then normalizes the resulting path.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{join, app_data_dir};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_data_dir_path = app_data_dir().await?;
///
/// let path = join([app_data_dir_path, "..", "users", "tauri", "avatar.png"]).await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn join(paths: impl IntoIterator<Item = &str>) -> crate::Result<PathBuf> {
    let paths = paths.into_iter();
    let raw = inner::join(serde_wasm_bindgen::to_value(&paths.collect::<Vec<_>>())?).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the directory name of a `path`. Trailing directory separators are ignored.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{dirname, app_data_dir};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let app_data_dir_path = app_data_dir().await?;
///
/// let dir = dirname(app_data_dir_path).await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn dirname(path: &str) -> crate::Result<PathBuf> {
    let raw = inner::dirname(JsValue::from_str(path)).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the extension of the `path`.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{extname, resolve_resource};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let resource_path = await resolve_resource("app.conf").await?;
/// let ext = extname(resource_path).await?;
/// assert_eq!(ext, "conf");
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn extname(path: &str) -> crate::Result<PathBuf> {
    let raw = inner::extname(JsValue::from_str(path)).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the last portion of a `path`. Trailing directory separators are ignored.
///
/// @param ext An optional file extension to be removed from the returned path.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::{basename, resolve_resource};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let resource_path = await resolve_resource("app.conf").await?;
/// let ext = basename(resource_path).await?;
/// assert_eq!(ext, "app");
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn basename(path: &str, ext: Option<&str>) -> crate::Result<PathBuf> {
    let raw = inner::basename(
        JsValue::from_str(path),
        ext.map(JsValue::from_str).unwrap_or(JsValue::null()),
    )
    .await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns whether the path is absolute or not.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::path::is_absolute;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// assert!(is_absolute("/home/tauri").await?);
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn is_absolute(path: &str) -> crate::Result<bool> {
    let raw = inner::isAbsolute(JsValue::from_str(path)).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/path.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn appCacheDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn appConfigDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn appDataDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn appLocalDataDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn appLogDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn audioDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn basename(path: JsValue, ext: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn cacheDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn configDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn dataDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn desktopDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn dirname(path: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn documentDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn downloadDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn executableDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn extname(path: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn fontDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn homeDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn isAbsolute(path: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn join(paths: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn localDataDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn normalize(path: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn pictureDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn publicDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn resolve(paths: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn resolveResource(path: JsValue) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn resourceDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn runtimeDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn templateDir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn videoDir() -> Result<JsValue, JsValue>;
    }
}
