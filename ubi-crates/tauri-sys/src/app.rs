//! Get application metadata.
//! 
//! he APIs must be added to tauri.allowlist.app in tauri.conf.json:
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "app": {
//!                 "all": true, // enable all app APIs
//!                 "show": true,
//!                 "hide": true
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use semver::Version;

/// Gets the application name.
///
/// # Example
///
/// ```typescript
/// import { getName } from '@tauri-apps/api/app';
/// const appName = await getName();
/// ```
#[inline(always)]
pub async fn get_name() -> crate::Result<String> {
    let js_val = inner::getName().await?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Gets the application version.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::get_version;
///     
/// let version = get_version().await;
/// ```
#[inline(always)]
pub async fn get_version() -> crate::Result<Version> {
    let js_val = inner::getVersion().await?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Gets the Tauri version.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_app::app:get_tauri_version;
///
/// let version = get_tauri_version().await;
/// ```
#[inline(always)]
pub async fn get_tauri_version() -> crate::Result<Version> {
    let js_val = inner::getTauriVersion().await?;

    Ok(serde_wasm_bindgen::from_value(js_val)?)
}

/// Shows the application on macOS. This function does not automatically focus the apps windows.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::show;
///
/// show().await;
/// ```
/// 
/// Requires [`allowlist > app > show`](https://tauri.app/v1/api/config#appallowlistconfig.show) to be enabled.
#[inline(always)]
pub async fn show() -> crate::Result<()> {
    Ok(inner::show().await?)
}

/// Hides the application on macOS.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_api::app::hide;
///
/// hide().await;
/// ```
/// 
/// Requires [`allowlist > app > hide`](https://tauri.app/v1/api/config#appallowlistconfig.hide) to be enabled.
#[inline(always)]
pub async fn hide() -> crate::Result<()> {
    Ok(inner::hide().await?)
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/app.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn getName() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn getTauriVersion() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn getVersion() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn hide() -> Result<(), JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn show() -> Result<(), JsValue>;
    }
}
