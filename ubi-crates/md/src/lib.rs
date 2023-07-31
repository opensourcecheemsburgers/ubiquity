use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct MarkdownFile {
    pub path: Option<String>,
    pub contents: String
}

#[derive(Deserialize, Serialize)]
pub struct MarkdownPath {
    pub path: String
}

pub const DOCS_KEY: &'static str = "ubiquity_about.md";
pub const DOCS_STR: &'static str = r#"# Ubiquity

Ubiquity is a free and [open-source](https://github.com/opensourcecheemsburgers/ubiquity) markdown editor.

## Installation

Ubiquity is available as a desktop application.

| Platform | Link |
| ------ | ------ |
| Linux (generic) | [ubiquity_0.4.0.tar.gz](https://github.com/opensourcecheemsburgers/ubiquity/releases/download/v0.4.0/ubiquity_0.4.0.tar.gz)
| Linux (appimage) | [ubiquity_0.4.0_amd64.AppImage](https://github.com/opensourcecheemsburgers/ubiquity/releases/download/v0.4.0/ubiquity_0.4.0_amd64.AppImage) 
|  Arch  | [AUR package](https://aur.archlinux.org/packages/ubiquity)
| Debian | [ubiquity_0.4.0_amd64.deb](https://github.com/opensourcecheemsburgers/ubiquity/releases/download/v0.4.0/ubiquity_0.4.0_amd64.deb)
| Windows (exe) | [ubiquity_0.4.0_x64-setup.exe](https://github.com/opensourcecheemsburgers/ubiquity/releases/download/v0.4.0/ubiquity_0.4.0_x64-setup.exe)       
| Windows (msi) | [ubiquity_0.4.0_x64_en-US.msi](https://github.com/opensourcecheemsburgers/ubiquity/releases/download/v0.4.0/ubiquity_0.4.0_x64_en-US.msi)  

What's in the latest release? Find out [here](https://github.com/opensourcecheemsburgers/ubiquity/blob/master/CHANGELOG.md).

**Tip**: Enable `Print backgrounds` to show code blocks when exporting pdfs.

## Author

Hi, I'm Stephen Power - the developer of Ubiquity. Check out my [website](https://stephenpower.ie).

<a href="https://www.buymeacoffee.com/opensourcecheemsburgers" class="cursor-pointer">
<img class="w-48" src="https://www.buymeacoffee.com/assets/img/guidelines/download-assets-sm-1.svg" />
</a>

## Language

Ubiquity is written in Rust.

![Ferris Gif](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif "*SNAP* I got your finger :)")

Ubiquity utilises two Rust frameworks.

* [Yew](https://www.yew.rs) - A web application framework.
* [Tauri](https://www.tauri.app) - A desktop application framework.

## UI Design

Ubiquity use two main components for UI design.

1. [Tailwind](https://www.tailwindcss.com) - A CSS framework.
2. [DaisyUI](https://www.daisyui.com) - A Tailwind CSS component library.

### Fonts

Ubiquity uses three different fonts.

- Fira Mono[^1] - The monospace font of the markdown editor.
- Comfortaa[^2] - The display font used for the "Ubiquity" name.
- Inter[^3] - The sans-serif font used for markdown preview.

### Icons

Ubiquity uses Lucide icons. You can find the full Lucide icon library at [lucide.dev](https://lucide.dev).

## File Size    
            
A note from Stephen Power, the developer of Ubiquity:
> Ubiquity use multiple different techniques for file size optimisation.
>
> For example, platform-specific Rust standard library recompilation, debug stripping, LLVM link time optimisations, WASM file size optimisation, and more.

Here are some examples of the file size optimisation:

- Optimizing Ubiquity for release in the `Cargo.toml` file.
    
```rust
[profile.release]        
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```

- Compiling a Linux-specific Rust standard library and building Ubiquity:

```
$ cargo tauri build --target x86_64-unknown-linux-gnu -- -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort
```

- Configure [`trunk`](https://trunkrs.dev/) to use [`wasm-opt`](https://crates.io/crates/wasm-opt) on the compiled `frontend.wasm`.

```
<link data-trunk rel="rust" data-wasm-opt="z" data-cargo-features="web" />
```

[^1]: [Fira Mono](https://fonts.google.com/specimen/Fira+Mono) is a monospace font designed by Carrois Apostrophe.
[^2]: [Comfortaa](https://fonts.google.com/specimen/Comfortaa) is a display font designed by Johan Aakerlund.
[^3]: [Inter](https://fonts.google.com/specimen/Inter) is a sans-serif font designed by Rasmus Andersson."#;