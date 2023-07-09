use std::path::PathBuf;
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

Ubiquity is a free and [open-source](https://github.com/opensourcecheemsburgers/ubiquity) markdown editor for Windows, Linux and Mac.

## Author

Stephen Power is the main developer of Ubiquity.

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

### Sample File Size Optimisation Code
    
***
```rust
[profile.release]        
panic = "abort"
codegen-units = 1
lto = true
opt-level = "z"
strip = true
```
***
        
### Platform File Size Table

| Platform | Zipped | Unzipped  |
| --------- | ----- | ----- |
| Web       | 1.4MB | 3.3MB |
| Windows   | 1.4MB | 3.3MB |
| Linux     | 1.4MB | 3.3MB |

[^1]: [Fira Mono](https://fonts.google.com/specimen/Fira+Mono) is a monospace font designed by Carrois Apostrophe.
[^2]: [Comfortaa](https://fonts.google.com/specimen/Comfortaa) is a display font designed by Johan Aakerlund.
[^3]: [Inter](https://fonts.google.com/specimen/Inter) is a sans-serif font designed by Rasmus Andersson."#; 