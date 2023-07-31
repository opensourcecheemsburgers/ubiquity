//! Customize the auto updater flow.

use futures::{Stream, channel::mpsc};
use serde::Deserialize;
use wasm_bindgen::{prelude::Closure, JsValue};
use crate::event::Listen;

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateManifest {
    pub body: String,
    pub date: String,
    pub version: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UpdateResult {
    pub manifest: Option<UpdateManifest>,
    pub should_update: bool,
}

#[derive(Deserialize)]
struct UpdateStatusResult {
    error: Option<String>,
    status: UpdateStatus,
}

#[derive(Deserialize)]
pub enum UpdateStatus {
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "DONE")]
    Done,
    #[serde(rename = "UPTODATE")]
    UpToDate,
}

/// Checks if an update is available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::check_update;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let update = check_update().await?;
/// // now run installUpdate() if needed
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn check_update() -> crate::Result<UpdateResult> {
    let raw = inner::checkUpdate().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Install the update if there's one available.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::{check_update, install_update};
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let update = check_update().await?;
///
/// if update.should_update {
///     log::info("Installing update {:?}", update.manifest);
///     install_update().await?;
/// }
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn install_update() -> crate::Result<()> {
    inner::installUpdate().await?;
    Ok(())
}

/// Listen to an updater event.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
/// 
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::updater::updater_events;
/// use web_sys::console;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let events = updater_events();
///
/// while let Some(event) = events.next().await {
///     console::log_1(&format!("Updater event {:?}", event).into());
/// }
/// # Ok(())
/// # }
/// ```
#[inline(always)]
pub async fn updater_events() -> crate::Result<impl Stream<Item = Result<UpdateStatus, String>>> {
    let (tx, rx) = mpsc::unbounded::<Result<UpdateStatus, String>>();

    let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
        let raw: UpdateStatusResult = serde_wasm_bindgen::from_value(raw).unwrap();

        let msg = if let Some(error) = raw.error {
            Err(error)
        } else {
            Ok(raw.status)
        };

        let _ = tx.unbounded_send(msg);
    });
    let unlisten = inner::onUpdaterEvent(&closure).await?;
    closure.forget();

    Ok(Listen {
        rx,
        unlisten: js_sys::Function::from(unlisten),
    })
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/updater.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn checkUpdate() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn installUpdate() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn onUpdaterEvent(
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;
    }
}
