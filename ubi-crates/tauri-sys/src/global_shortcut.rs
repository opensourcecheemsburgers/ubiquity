//! Register global shortcuts.
//!
//! ## Differences to the JavaScript API
//!
//! ## `registerAll`
//!
//! ```rust,no_run
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let shortcuts = ["CommandOrControl+Shift+C", "Ctrl+Alt+F12"];
//!
//! let streams = futures::future::try_join_all(shortcuts.map(|s| async move {
//!     let stream = global_shortcut::register(s).await?;
//!
//!     anyhow::Ok(stream.map(move |_| s))
//! }))
//! .await?;
//!
//! let mut events = futures::stream::select_all(streams);
//!
//! while let Some(shortcut) = events.next().await {
//!     log::debug!("Shortcut {} triggered", shortcut)
//! }
//! # Ok(())
//! # }
//! ```
//!
//! The APIs must be added to tauri.allowlist.globalShortcut in tauri.conf.json:
//!
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "globalShortcut": {
//!                 "all": true // enable all global shortcut APIs
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use futures::{channel::mpsc, Stream, StreamExt};
use wasm_bindgen::{prelude::Closure, JsValue};

/// Determines whether the given shortcut is registered by this application or not.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::global_shortcut::is_registered;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let registered = is_registered("CommandOrControl+P").await?;
/// # Ok(())
/// # }
/// ```
pub async fn is_registered(shortcut: &str) -> crate::Result<bool> {
    let raw = inner::isRegistered(shortcut).await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Register a global shortcut.
///
/// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
/// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
///
/// # Examples
///
/// ```rust,no_run
/// use tauri_sys::global_shortcut::register;
/// use web_sys::console;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let events = register("CommandOrControl+Shift+C").await?;
///
/// while let Some(_) in events.next().await {
///     console::log_1(&"Shortcut triggered".into());
/// }
/// # Ok(())
/// # }
/// ```
pub async fn register(shortcut: &str) -> crate::Result<impl Stream<Item = ()>> {
    let (tx, rx) = mpsc::unbounded();

    let closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
        let _ = tx.unbounded_send(());
    });
    inner::register(shortcut, &closure).await?;
    closure.forget();

    Ok(Listen {
        shortcut: JsValue::from_str(shortcut),
        rx,
    })
}

struct Listen<T> {
    pub shortcut: JsValue,
    pub rx: mpsc::UnboundedReceiver<T>,
}

impl<T> Drop for Listen<T> {
    fn drop(&mut self) {
        log::debug!("Unregistering shortcut {:?}", self.shortcut);
        inner::unregister(self.shortcut.clone());
    }
}

impl<T> Stream for Listen<T> {
    type Item = T;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        self.rx.poll_next_unpin(cx)
    }
}

/// Register a collection of global shortcuts.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::global_shortcut::register;
/// use web_sys::console;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let events = register_all(["CommandOrControl+Shift+C", "Ctrl+Alt+F12"]).await?;
///
/// while let Some(shortcut) in events.next().await {
///     console::log_1(&format!("Shortcut {} triggered", shortcut).into());
/// }
/// # Ok(())
/// # }
/// ```
// pub async fn register_all<I>(shortcuts: impl IntoIterator<Item = &str>) -> crate::Result<impl Stream<Item = String>>
// {
//     let shortcuts: Array = shortcuts.into_iter().map(JsValue::from_str).collect();
//     let (tx, rx) = mpsc::unbounded::<String>();

//     let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
//         let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap());
//     });
//     inner::registerAll(shortcuts.clone(), &closure).await?;
//     closure.forget();

//     Ok(ListenAll { shortcuts, rx })
// }

// struct ListenAll<T> {
//     pub shortcuts: js_sys::Array,
//     pub rx: mpsc::UnboundedReceiver<T>,
// }

// impl<T> Drop for ListenAll<T> {
//     fn drop(&mut self) {
//         for shortcut in self.shortcuts.iter() {
//             inner::unregister(shortcut);
//         }
//     }
// }

// impl<T> Stream for ListenAll<T> {
//     type Item = T;

//     fn poll_next(
//         mut self: std::pin::Pin<&mut Self>,
//         cx: &mut std::task::Context<'_>,
//     ) -> std::task::Poll<Option<Self::Item>> {
//         self.rx.poll_next_unpin(cx)
//     }
// }

mod inner {
    // use js_sys::Array;
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/globalShortcut.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn isRegistered(shortcut: &str) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn register(
            shortcut: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<(), JsValue>;
        // #[wasm_bindgen(catch)]
        // pub async fn registerAll(
        //     shortcuts: Array,
        //     handler: &Closure<dyn FnMut(JsValue)>,
        // ) -> Result<(), JsValue>;
        pub fn unregister(shortcut: JsValue);
    }
}
