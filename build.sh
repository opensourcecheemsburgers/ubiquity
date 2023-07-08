# This is a file size optimisation script for Ubiquity.
# It compiles a platform-specific std library for Linux instead of using the precompiled one.

cargo tauri build --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
cd target/x86_64-unknown-linux-gnu/release/
echo ubiquity: $(cat ubiquity | wc -c)
