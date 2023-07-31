//! Send toast notifications (brief auto-expiring OS window element) to your user. Can also be used with the Notification Web API.
//! 
//! The APIs must be added to tauri.allowlist.notification in tauri.conf.json:
//! 
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "notification": {
//!                 "all": true // enable all notification APIs
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use serde::{Deserialize, Serialize};

/// Checks if the permission to send notifications is granted.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::notification;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let is_granted = notification::is_permission_granted().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn is_permission_granted() -> crate::Result<bool> {
    let raw = inner::isPermissionGranted().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Requests the permission to send notifications.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::notification;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let perm = notification::request_permission().await?;
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn request_permission() -> crate::Result<Permission> {
    let raw = inner::requestPermission().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Possible permission values.
#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    #[default]
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "granted")]
    Granted,
    #[serde(rename = "denied")]
    Denied,
}

/// The desktop notification definition.
///
/// Allows you to construct a Notification data and send it.
#[derive(Debug, Default, Serialize)]
pub struct Notification<'a> {
    body: Option<&'a str>,
    title: Option<&'a str>,
    icon: Option<&'a str>,
}

impl<'a> Notification<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the notification title.
    pub fn set_title(&mut self, title: &'a str) {
        self.title = Some(title);
    }

    /// Sets the notification body.
    pub fn set_body(&mut self, body: &'a str) {
        self.body = Some(body);
    }

    /// Sets the notification icon.
    pub fn set_icon(&mut self, icon: &'a str) {
        self.icon = Some(icon);
    }

    /// Shows the notification.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use tauri_sys::notification::Notification;
    ///
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// Notification::new()
    ///     .set_title("Tauri")
    ///     .set_body("Tauri is awesome!")
    ///     .show()?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline(always)]
    pub fn show(&self) -> crate::Result<()> {
        inner::sendNotification(serde_wasm_bindgen::to_value(&self)?)?;

        Ok(())
    }
}

mod inner {
    use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

    #[wasm_bindgen(module = "/src/notification.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn isPermissionGranted() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn requestPermission() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub fn sendNotification(notification: JsValue) -> Result<(), JsValue>;
    }
}
