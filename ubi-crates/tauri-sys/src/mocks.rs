//! 

use js_sys::Array;
use wasm_bindgen::{prelude::Closure, JsValue};

/// Mocks the current window label
/// In non-tauri context it is required to call this function///before* using the `@tauri-apps/api/window` module.
///
/// This function only mocks the *presence* of a window,
/// window properties (e.g. width and height) can be mocked like regular IPC calls using the `mockIPC` function.
pub fn mock_window(current: &str) {
    inner::mockWindows(current, JsValue::UNDEFINED)
}

/// Mocks many window labels.
/// In non-tauri context it is required to call this function///before* using the `@tauri-apps/api/window` module.
///
/// This function only mocks the *presence* of windows,
/// window properties (e.g. width and height) can be mocked like regular IPC calls using the `mockIPC` function.
///
/// @param current Label of window this JavaScript context is running in.
/// @param additionalWindows Label of additional windows the app has.
pub fn mock_windows(current: &str, additional_windows: &[&str]) {
    inner::mockWindows(
        current,
        Array::from_iter(additional_windows.iter().map(|str| JsValue::from_str(str))).into(),
    )
}

/// Intercepts all IPC requests with the given mock handler.
///
/// This function can be used when testing tauri frontend applications or when running the frontend in a Node.js context during static site generation.
pub fn mock_ipc<H, R, E>(mut handler: H)
where
    H: FnMut(String, JsValue) -> Result<R, E> + 'static,
    R: Into<JsValue>,
    E: Into<JsValue>,
{
    let closure = Closure::<dyn FnMut(String, JsValue) -> Result<JsValue, JsValue>>::new(
        move |cmd, payload| (handler)(cmd, payload).map(Into::into).map_err(Into::into),
    );

    inner::mockIPC(&closure);

    closure.forget();
}

/// Clears mocked functions/data injected by the other functions in this module.
/// When using a test runner that doesn't provide a fresh window object for each test, calling this function will reset tauri specific properties.
pub fn clear_mocks() {
    inner::clearMocks()
}

mod inner {
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/mocks.js")]
    extern "C" {
        #[wasm_bindgen(variadic)]
        pub fn mockWindows(current: &str, rest: JsValue);
        pub fn mockIPC(handler: &Closure<dyn FnMut(String, JsValue) -> Result<JsValue, JsValue>>);
        pub fn clearMocks();
    }
}
