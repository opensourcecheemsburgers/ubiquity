use wasm_bindgen::JsValue;


#[derive(Clone, Eq, PartialEq, Debug, thiserror::Error)]
pub enum Error {
    #[error("JS Binding: {0}")]
    Binding(String),
    #[error("JSON: {0}")]
    Serde(String),
    #[cfg(any(feature = "event", feature = "window"))]
    #[error("Oneshot cancelled: {0}")]
    OneshotCanceled(#[from] futures::channel::oneshot::Canceled)
}

impl From<serde_wasm_bindgen::Error> for Error {
    fn from(e: serde_wasm_bindgen::Error) -> Self {
        Self::Serde(e.to_string())
    }
}

impl From<JsValue> for Error {
    fn from(e: JsValue) -> Self {
        Self::Binding(format!("{:?}", e))
    }
}