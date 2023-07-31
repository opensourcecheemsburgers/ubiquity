//! Bindings to the [`Tauri API`](https://tauri.app/v1/api/js/) for projects using [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen)
//!
//! Tauri is a framework for building tiny, blazing fast, and secure cross-platform applications.
//! Developers can integrate any front-end framework that compiles to HTML, JS and CSS for building their user interface.
//! The backend of the application is a rust binary, leveraging the [`tauri`] crate.
//!
//! This crate contains idiomatic rust bindings to the backend, for usage within Rust projects that target wasm32-unknown-unknown,
//! for example Rust frontend frameworks such as [`yew`](https://yew.rs), [`sycamore`](https://sycamore-rs.netlify.app) or [`dominator`](https://github.com/Pauan/rust-dominator).
//!
//! The wasmtime crate has similar concepts to the the JS WebAssembly API as well as the proposed C API, but the Rust API is designed for efficiency, ergonomics, and expressivity in Rust. As with all other Rust code you’re guaranteed that programs will be safe (not have undefined behavior or segfault) so long as you don’t use unsafe in your own program.
//!
//! # Differences to the JavaScript API
//!
//! ## Event Listeners
//!
//! Event Listeners, such as [`event::listen`] module or [`window::WebviewWindow::listen`],
//! are modeled as async streams of data using the [`futures::Stream`] trait instead of using callbacks.
//! Streams have multiple advantages over callbacks:
//!
//! #### Stream Combinators
//!
//! Streams are essentially the async equivalent of the standard [`Iterator`] and therefore expose a very similar set of combinator functions.
//! This means streams are much more versatile and ergonomic than simple callbacks.
//!
//! For example, we can use Stream combinators and various utility functions
//! to replicate the `registerAll` function that unregisters the shortcuts after 20 seconds:
//!
//! ```rust
//! use futures::{future, stream, Stream, StreamExt};
//! use std::time::Duration;
//! use tauri_sys::global_shortcut;
//!
//! async fn register_with_shortcut<'a>(
//!     shortcut: &'a str,
//! ) -> anyhow::Result<impl Stream<Item = &'a str>> {
//!     let stream = global_shortcut::register(shortcut).await?;
//!
//!     Ok(stream.map(move |_| shortcut))
//! }
//!
//! async fn register_all() {
//!     let shortcuts = ["CommandOrControl+Shift+C", "Ctrl+Alt+F12"];
//!
//!     let timeout = gloo_timers::future::sleep(Duration::from_secs(20));
//!
//!     // await the futures that creates the streams, exiting early if any future resolves with an error
//!     let streams = future::try_join_all(shortcuts.map(register_with_shortcut)).await?;
//!
//!     // combine all streams into one
//!     let mut events = stream::select_all(streams).take_until(timeout);
//!
//!     while let Some(shortcut) = events.next().await {
//!         log::debug!("Shortcut {} triggered", shortcut);
//!     }
//! }
//! ```
//!
//! #### Automatic cleanup
//!
//! Streams follow Rust's RAII idiom as they automatically clean up after themselves when being dropped.
//! No need to manually call `unlisten` like in the JS API to avoid memory leaks or double-listens.
//!
//! ```rust
//! async fn process_some_errors() {
//!     let win = WebviewWindow::get_by_label("main").unwrap();
//!
//!     let errors = win.listen("tauri://error").await?
//!         .take(3);
//!     
//!     while let Some(err) = errors.next().await {
//!         log::error!("Something bad happened! {}", err)
//!     }
//!
//!     // the stream is dropped here and the underlying listener automatically detached.
//! }
//! ```
//!
//! #### Streams are buffered
//!
//! Streams, much like iterators, are poll-based meaning the caller is responsible for advancing it.
//! This allows greater flexibilty as you can freely decide *when* to process events.
//! Event streams are internally backed by an unbounded queue so events are buffered until read,
//! so no events are getting lost even if you temporarily pause processing.
//!
//! Being unbounded means the memory consumption will grow if the stream is kept around, but not read from.
//! This is rarely a concern in practice, but if you need to suspend processing of events for a long time,
//! you should rather drop the entire stream and re-create it as needed later.
//!
//! ### Cancelling Streams
//!
//! One usecase of the `unlisten` function might intuitively not map well to streams: Cancellation.
//! In JavaScript you can do this when you want to detach an event listener:
//!
//! ```js
//! import { listen } from '@tauri-apps/api/event'
//!
//! const unlisten = await listen('rust-event', (ev) => console.log(ev))
//!
//! // Some time later. We are no longer interested in listening to the event
//! unlisten()
//! ```
//!
//! But if the Rust event stream only gets detached when the stream get's dropped, how can we cancel the stream at will?
//! We can make use of the combinators and utility functions offered by the [`futures`] crate again, namely the [`futures::stream::Abortable`] type:
//!
//! ```rust
//! use tauri_sys::event::listen;
//!
//! let events = listen::<()>("rust-event").await?
//! // abort handle behaves identical to the JavaScript `unlisten` function
//! let (events, abort_handle) = futures::stream::abortable(events);
//!
//! while let Some(_) = events.next().await {
//!     log::debug!("Received event!");
//! }
//!
//! // in some other task, when we're done with listening to the events
//! abort_handle.abort();
//! ```

#[cfg(feature = "app")]
pub mod app;
#[cfg(feature = "clipboard")]
pub mod clipboard;
#[cfg(feature = "dialog")]
pub mod dialog;
pub mod error;
#[cfg(feature = "event")]
pub mod event;
#[cfg(feature = "global_shortcut")]
pub mod global_shortcut;
#[cfg(feature = "mocks")]
pub mod mocks;
#[cfg(feature = "notification")]
pub mod notification;
#[cfg(feature = "os")]
pub mod os;
#[cfg(feature = "path")]
pub mod path;
#[cfg(feature = "process")]
pub mod process;
#[cfg(feature = "tauri")]
pub mod tauri;
#[cfg(feature = "updater")]
pub mod updater;
#[cfg(feature = "window")]
pub mod window;

pub(crate) use error::Error;
pub(crate) type Result<T> = core::result::Result<T, Error>;

#[cfg(any(feature = "dialog", feature = "window"))]
pub(crate) mod utils {
    pub struct ArrayIterator {
        pos: u32,
        arr: js_sys::Array,
    }
    
    impl ArrayIterator {
        pub fn new(arr: js_sys::Array) -> Self {
            Self { pos: 0, arr }
        }
    }
    
    impl Iterator for ArrayIterator {
        type Item = wasm_bindgen::JsValue;
    
        fn next(&mut self) -> Option<Self::Item> {
            let raw = self.arr.get(self.pos);
    
            if raw.is_undefined() {
                None
            } else {
                self.pos += 1;
    
                Some(raw)
            }
        }
    }
}
