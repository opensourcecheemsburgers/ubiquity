//! Provides operating system-related utility methods and properties.
//! 
//! The APIs must be added to tauri.allowlist.os in tauri.conf.json:
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "os": {
//!                 "all": true, // enable all Os APIs
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Arch {
    #[serde(rename = "x86")]
    X86,
    #[serde(rename = "x86_64")]
    X86_64,
    #[serde(rename = "arm")]
    Arm,
    #[serde(rename = "aarch64")]
    Aarch64,
    #[serde(rename = "mips")]
    Mips,
    #[serde(rename = "mips64")]
    Mips64,
    #[serde(rename = "powerpc")]
    Powerpc,
    #[serde(rename = "powerpc64")]
    Powerpc64,
    #[serde(rename = "riscv64")]
    Riscv64,
    #[serde(rename = "s390x")]
    S390x,
    #[serde(rename = "sparc64")]
    Sparc64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Platform {
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "darwin")]
    Darwin,
    #[serde(rename = "ios")]
    Ios,
    #[serde(rename = "freebsd")]
    Freebsd,
    #[serde(rename = "dragonfly")]
    Dragonfly,
    #[serde(rename = "netbsd")]
    Netbsd,
    #[serde(rename = "openbsd")]
    Openbsd,
    #[serde(rename = "solaris")]
    Solaris,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "win32")]
    Win32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum OsKind {
    #[serde(rename = "Linux")]
    Linux,
    #[serde(rename = "Darwin")]
    Darwin,
    #[serde(rename = "Windows_NT")]
    WindowsNT,
}

/// Returns the operating system CPU architecture for which the tauri app was compiled.
#[inline(always)]
pub async fn arch() -> crate::Result<Arch> {
    let raw = inner::arch().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns a string identifying the operating system platform. The value is set at compile time.
#[inline(always)]
pub async fn platform() -> crate::Result<Platform> {
    let raw = inner::platform().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns the operating system's default directory for temporary files.
#[inline(always)]
pub async fn tempdir() -> crate::Result<PathBuf> {
    let raw = inner::tempdir().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns [`OsKind::Linux`] on Linux, [`OsKind::Darwin`] on macOS, and [`OsKind::WindowsNT`] on Windows.
#[inline(always)]
pub async fn kind() -> crate::Result<OsKind> {
    let raw = inner::kind().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

/// Returns a string identifying the kernel version.
#[inline(always)]
pub async fn version() -> crate::Result<String> {
    let raw = inner::version().await?;

    Ok(serde_wasm_bindgen::from_value(raw)?)
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/os.js")]
    extern "C" {
        #[wasm_bindgen(catch)]
        pub async fn arch() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn platform() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn tempdir() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch, js_name = "type")]
        pub async fn kind() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn version() -> Result<JsValue, JsValue>;
    }
}
