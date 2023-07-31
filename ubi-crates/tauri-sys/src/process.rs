//! Perform operations on the current process.

/// Exits immediately with the given `exit_code`.
#[inline(always)]
pub async fn exit(exit_code: i32) -> ! {
    inner::exit(exit_code).await;
    unreachable!()
}

/// Exits the current instance of the app then relaunches it.
#[inline(always)]
pub fn relaunch() {
    inner::relaunch();
}

mod inner {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/src/process.js")]
    extern "C" {
        pub async fn exit(exitCode: i32);
        pub fn relaunch();
    }
}
