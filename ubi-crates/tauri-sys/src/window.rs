//! Provides APIs to create windows, communicate with other windows and manipulate the current window.
//!
//! The APIs must be added to tauri.allowlist.window in tauri.conf.json:
//! ```json
//! {
//!     "tauri": {
//!         "allowlist": {
//!             "window": {
//!                 "all": true, // enable all window APIs
//!                 "create": true, // enable window creation
//!                 "center": true,
//!                 "requestUserAttention": true,
//!                 "setResizable": true,
//!                 "setTitle": true,
//!                 "maximize": true,
//!                 "unmaximize": true,
//!                 "minimize": true,
//!                 "unminimize": true,
//!                 "show": true,
//!                 "hide": true,
//!                 "close": true,
//!                 "setDecorations": true,
//!                 "setAlwaysOnTop": true,
//!                 "setSize": true,
//!                 "setMinSize": true,
//!                 "setMaxSize": true,
//!                 "setPosition": true,
//!                 "setFullscreen": true,
//!                 "setFocus": true,
//!                 "setIcon": true,
//!                 "setSkipTaskbar": true,
//!                 "setCursorGrab": true,
//!                 "setCursorVisible": true,
//!                 "setCursorIcon": true,
//!                 "setCursorPosition": true,
//!                 "setIgnoreCursorEvents": true,
//!                 "startDragging": true,
//!                 "print": true
//!             }
//!         }
//!     }
//! }
//! ```
//! It is recommended to allowlist only the APIs you use for optimal bundle size and security.

use crate::{
    event::{Event, Listen, Once},
    utils::ArrayIterator,
};
use futures::{
    channel::{mpsc, oneshot},
    Stream,
};
use js_sys::Array;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Display;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize)]
pub enum TitleBarStyle {
    #[default]
    #[serde(rename = "visible")]
    Visible,
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "overlay")]
    Overlay,
}

/// Attention type to request on a window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserAttentionType {
    /// #### Platform-specific
    /// - macOS: Bounces the dock icon until the application is in focus.
    /// - Windows: Flashes both the window and the taskbar button until the application is in focus.
    Critical = 1,
    /// #### Platform-specific
    /// - macOS: Bounces the dock icon once.
    /// - Windows: Flashes the taskbar button until the application is in focus.
    Informational,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Position {
    Physical(PhysicalPosition),
    Logical(LogicalPosition),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Size {
    Physical(PhysicalSize),
    Logical(LogicalSize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CursorIcon {
    Default,
    Crosshair,
    Hand,
    Arrow,
    Move,
    Text,
    Wait,
    Help,
    Progress,
    // something cannot be done
    NotAllowed,
    ContextMenu,
    Cell,
    VerticalText,
    Alias,
    Copy,
    NoDrop,
    // something can be grabbed
    Grab,
    /// something is grabbed
    Grabbing,
    AllScroll,
    ZoomIn,
    ZoomOut,
    // edge is to be moved
    EResize,
    NResize,
    NeResize,
    NwResize,
    SResize,
    SeResize,
    SwResize,
    WResize,
    EwResize,
    NsResize,
    NeswResize,
    NwseResize,
    ColResize,
    RowResize,
}

impl Display for CursorIcon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CursorIcon::Default => write!(f, "default"),
            CursorIcon::Crosshair => write!(f, "crosshair"),
            CursorIcon::Hand => write!(f, "hand"),
            CursorIcon::Arrow => write!(f, "arrow"),
            CursorIcon::Move => write!(f, "move"),
            CursorIcon::Text => write!(f, "text"),
            CursorIcon::Wait => write!(f, "wait"),
            CursorIcon::Help => write!(f, "help"),
            CursorIcon::Progress => write!(f, "progress"),
            CursorIcon::NotAllowed => write!(f, "notAllowed"),
            CursorIcon::ContextMenu => write!(f, "contextMenu"),
            CursorIcon::Cell => write!(f, "cell"),
            CursorIcon::VerticalText => write!(f, "verticalText"),
            CursorIcon::Alias => write!(f, "alias"),
            CursorIcon::Copy => write!(f, "copy"),
            CursorIcon::NoDrop => write!(f, "noDrop"),
            CursorIcon::Grab => write!(f, "grab"),
            CursorIcon::Grabbing => write!(f, "grabbing"),
            CursorIcon::AllScroll => write!(f, "allScroll"),
            CursorIcon::ZoomIn => write!(f, "zoomIn"),
            CursorIcon::ZoomOut => write!(f, "zoomOut"),
            CursorIcon::EResize => write!(f, "eResize"),
            CursorIcon::NResize => write!(f, "nResize"),
            CursorIcon::NeResize => write!(f, "neResize"),
            CursorIcon::NwResize => write!(f, "nwResize"),
            CursorIcon::SResize => write!(f, "sResize"),
            CursorIcon::SeResize => write!(f, "seResize"),
            CursorIcon::SwResize => write!(f, "swResize"),
            CursorIcon::WResize => write!(f, "wResize"),
            CursorIcon::EwResize => write!(f, "ewResize"),
            CursorIcon::NsResize => write!(f, "nsResize"),
            CursorIcon::NeswResize => write!(f, "neswResize"),
            CursorIcon::NwseResize => write!(f, "nwseResize"),
            CursorIcon::ColResize => write!(f, "colResize"),
            CursorIcon::RowResize => write!(f, "rowResize"),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WebviewWindowOptions<'a> {
    url: Option<&'a str>,
    center: bool,
    x: Option<i32>,
    y: Option<i32>,
    width: Option<u32>,
    height: Option<u32>,
    min_width: Option<u32>,
    min_height: Option<u32>,
    max_width: Option<u32>,
    max_height: Option<u32>,
    resizable: bool,
    title: Option<&'a str>,
    fullscreen: bool,
    focus: bool,
    transparent: bool,
    maximized: bool,
    visible: bool,
    decorations: bool,
    always_on_top: bool,
    skip_taskbar: bool,
    file_drop_enabled: bool,
    theme: Option<Theme>,
    title_bar_style: Option<TitleBarStyle>,
    hidden_title: bool,
    accept_first_mouse: bool,
    tabbing_identifier: Option<&'a str>,
    user_agent: Option<&'a str>,
}

impl<'a> Default for WebviewWindowOptions<'a> {
    fn default() -> Self {
        Self {
            url: None,
            center: false,
            x: None,
            y: None,
            width: None,
            height: None,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            resizable: true,
            title: None,
            fullscreen: false,
            focus: true,
            transparent: false,
            maximized: false,
            visible: true,
            decorations: true,
            always_on_top: false,
            skip_taskbar: false,
            file_drop_enabled: true,
            theme: None,
            title_bar_style: None,
            hidden_title: false,
            accept_first_mouse: true,
            tabbing_identifier: None,
            user_agent: None,
        }
    }
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct WebviewWindowBuilder<'a> {
    label: &'a str,
    inner: WebviewWindowOptions<'a>,
}

impl<'a> WebviewWindowBuilder<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            ..Default::default()
        }
    }

    /// Remote URL or local file path to open.
    ///
    /// - URL such as `https://github.com/tauri-apps` is opened directly on a Tauri window.
    /// - data: URL such as `data:text/html,<html>...` is only supported with the `window-data-url` Cargo feature for the `tauri` dependency.
    /// - local file path or route such as `/path/to/page.html` or `/users` is appended to the application URL (the devServer URL on development, or `tauri://localhost/` and `https://tauri.localhost/` on production).
    pub fn set_url(&mut self, url: &'a str) -> &mut Self {
        self.inner.url = Some(url);
        self
    }

    /// Show window in the center of the screen.
    pub fn set_center(&mut self, center: bool) -> &mut Self {
        self.inner.center = center;
        self
    }

    /// The initial position.
    pub fn set_position(&mut self, position: PhysicalPosition) -> &mut Self {
        self.inner.x = Some(position.x());
        self.inner.y = Some(position.y());
        self
    }

    /// The initial size.
    pub fn set_size(&mut self, size: PhysicalSize) -> &mut Self {
        self.inner.width = Some(size.width());
        self.inner.height = Some(size.height());
        self
    }

    /// Minimum window size.
    pub fn set_min_size(&mut self, min_size: PhysicalSize) -> &mut Self {
        self.inner.min_width = Some(min_size.width());
        self.inner.min_height = Some(min_size.height());
        self
    }

    /// Maximum window size.
    pub fn set_max_size(&mut self, max_size: PhysicalSize) -> &mut Self {
        self.inner.max_width = Some(max_size.width());
        self.inner.max_height = Some(max_size.height());
        self
    }

    /// Whether the window is resizable or not.
    pub fn set_resizable(&mut self, resizable: bool) -> &mut Self {
        self.inner.resizable = resizable;
        self
    }

    /// Window title.
    pub fn set_title(&mut self, title: &'a str) -> &mut Self {
        self.inner.title = Some(title);
        self
    }

    /// Whether the window is in fullscreen mode or not.
    pub fn set_fullscreen(&mut self, fullscreen: bool) -> &mut Self {
        self.inner.fullscreen = fullscreen;
        self
    }

    /// Whether the window will be initially focused or not.
    pub fn set_focus(&mut self, focus: bool) -> &mut Self {
        self.inner.focus = focus;
        self
    }

    /// Whether the window is transparent or not.
    ///
    /// Note that on `macOS` this requires the `macos-private-api` feature flag, enabled under `tauri.conf.json > tauri > macOSPrivateApi`.
    /// WARNING: Using private APIs on `macOS` prevents your application from being accepted to the `App Store`.
    pub fn set_transparent(&mut self, transparent: bool) -> &mut Self {
        self.inner.transparent = transparent;
        self
    }

    /// Whether the window should be maximized upon creation or not.
    pub fn set_maximized(&mut self, maximized: bool) -> &mut Self {
        self.inner.maximized = maximized;
        self
    }

    /// Whether the window should be immediately visible upon creation or not.
    pub fn set_visible(&mut self, visible: bool) -> &mut Self {
        self.inner.visible = visible;
        self
    }

    /// Whether the window should have borders and bars or not.
    pub fn set_decorations(&mut self, decorations: bool) -> &mut Self {
        self.inner.decorations = decorations;
        self
    }

    /// Whether the window should always be on top of other windows or not.
    pub fn set_always_on_top(&mut self, always_on_top: bool) -> &mut Self {
        self.inner.always_on_top = always_on_top;
        self
    }

    /// Whether or not the window icon should be added to the taskbar.
    pub fn set_skip_taskbar(&mut self, skip_taskbar: bool) -> &mut Self {
        self.inner.skip_taskbar = skip_taskbar;
        self
    }

    /// Whether the file drop is enabled or not on the webview. By default it is enabled.
    ///
    /// Disabling it is required to use drag and drop on the frontend on Windows.
    pub fn set_file_drop_enabled(&mut self, file_drop_enabled: bool) -> &mut Self {
        self.inner.file_drop_enabled = file_drop_enabled;
        self
    }

    /// The initial window theme. Defaults to the system theme.
    ///
    /// Only implemented on Windows and macOS 10.14+.
    pub fn set_theme(&mut self, theme: Theme) -> &mut Self {
        self.inner.theme = Some(theme);
        self
    }

    /// The style of the macOS title bar.
    pub fn set_title_bar_style(&mut self, title_bar_style: TitleBarStyle) -> &mut Self {
        self.inner.title_bar_style = Some(title_bar_style);
        self
    }

    /// If `true`, sets the window title to be hidden on macOS.
    pub fn set_hidden_title(&mut self, hidden_title: bool) -> &mut Self {
        self.inner.hidden_title = hidden_title;
        self
    }

    /// Whether clicking an inactive window also clicks through to the webview.
    pub fn set_accept_first_mouse(&mut self, accept_first_mouse: bool) -> &mut Self {
        self.inner.accept_first_mouse = accept_first_mouse;
        self
    }

    /// Defines the window [tabbing identifier](https://developer.apple.com/documentation/appkit/nswindow/1644704-tabbingidentifier) on macOS.
    ///
    /// Windows with the same tabbing identifier will be grouped together.
    /// If the tabbing identifier is not set, automatic tabbing will be disabled.
    pub fn set_tabbing_identifier(&mut self, tabbing_identifier: &'a str) -> &mut Self {
        self.inner.tabbing_identifier = Some(tabbing_identifier);
        self
    }

    /// The user agent for the webview.
    pub fn set_user_agent(&mut self, user_agent: &'a str) -> &mut Self {
        self.inner.user_agent = Some(user_agent);
        self
    }

    /// Creates a new webview window.
    ///
    /// Requires [`allowlist > window > create`](https://tauri.app/v1/api/config#windowallowlistconfig.create) to be enabled.
    pub async fn build(&self) -> crate::Result<WebviewWindow> {
        let opts = serde_wasm_bindgen::to_value(&self.inner)?;

        let win = WebviewWindow(inner::WebviewWindow::new(self.label, opts));
        win.once::<()>("tauri://created").await?;
        Ok(win)
    }
}

/// Create new webview windows and get a handle to existing ones.
///
/// Windows are identified by a label a unique identifier that can be used to reference it later. It may only contain alphanumeric characters a-zA-Z plus the following special characters -, /, : and _.
#[derive(Debug, Clone, PartialEq)]
pub struct WebviewWindow(inner::WebviewWindow);

impl WebviewWindow {
    pub fn get_by_label(label: &str) -> Option<Self> {
        inner::WebviewWindow::getByLabel(label).map(Self)
    }

    /// The label of this window.
    pub fn label(&self) -> String {
        self.0.label()
    }

    /// Returns the scale factor that can be used to map logical pixels to physical pixels, and vice versa.
    pub async fn scale_factor(&self) -> crate::Result<f64> {
        let js_val = self.0.scaleFactor().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Returns the position of the top-left hand corner of the window’s client area relative to the top-left hand corner of the desktop.
    pub async fn inner_position(&self) -> crate::Result<PhysicalPosition> {
        Ok(PhysicalPosition(
            self.0.innerPosition().await?.unchecked_into(),
        ))
    }

    /// Returns the position of the top-left hand corner of the window relative to the top-left hand corner of the desktop.
    pub async fn outer_position(&self) -> crate::Result<PhysicalPosition> {
        Ok(PhysicalPosition(
            self.0.outerPosition().await?.unchecked_into(),
        ))
    }

    /// Returns the physical size of the window’s client area.
    ///
    /// The client area is the content of the window, excluding the title bar and borders.
    pub async fn inner_size(&self) -> crate::Result<PhysicalSize> {
        Ok(PhysicalSize(self.0.innerSize().await?.unchecked_into()))
    }

    /// Returns the physical size of the entire window.
    ///
    /// These dimensions include the title bar and borders. If you don’t want that (and you usually don’t), use inner_size instead.
    pub async fn outer_size(&self) -> crate::Result<PhysicalSize> {
        Ok(PhysicalSize(self.0.outerSize().await?.unchecked_into()))
    }

    /// Gets the window’s current fullscreen state.
    pub async fn is_fullscreen(&self) -> crate::Result<bool> {
        let js_val = self.0.isFullscreen().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Gets the window’s current maximized state.
    pub async fn is_maximized(&self) -> crate::Result<bool> {
        let js_val = self.0.isMaximized().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Gets the window’s current decoration state.
    pub async fn is_decorated(&self) -> crate::Result<bool> {
        let js_val = self.0.isDecorated().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Gets the window’s current resizable state.
    pub async fn is_resizable(&self) -> crate::Result<bool> {
        let js_val = self.0.isResizable().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Gets the window’s current visibility state.
    pub async fn is_visible(&self) -> crate::Result<bool> {
        let js_val = self.0.isVisible().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Returns the current window theme.
    ///
    /// #### Platform-specific
    /// - macOS: Only supported on macOS 10.14+.
    pub async fn theme(&self) -> crate::Result<Theme> {
        let js_val = self.0.theme().await?;

        Ok(serde_wasm_bindgen::from_value(js_val)?)
    }

    /// Centers the window.
    ///
    /// Requires [`allowlist > window > center`](https://tauri.app/v1/api/config#windowallowlistconfig.center) to be enabled.
    pub async fn center(&self) -> crate::Result<()> {
        Ok(self.0.center().await?)
    }

    /// Requests user attention to the window, this has no effect if the application is already focused. How requesting for user attention manifests is platform dependent, see UserAttentionType for details.
    ///
    /// Providing None will unset the request for user attention. Unsetting the request for user attention might not be done automatically by the WM when the window receives input.
    ///
    /// #### Platform-specific
    /// - macOS: None has no effect.
    /// - Linux: Urgency levels have the same effect.
    ///
    /// Requires [`allowlist > window > requestUserAttention`](https://tauri.app/v1/api/config#windowallowlistconfig.requestuserattention) to be enabled.
    pub async fn request_user_attention(
        &self,
        request_type: UserAttentionType,
    ) -> crate::Result<()> {
        Ok(self.0.requestUserAttention(request_type as u32).await?)
    }

    /// Opens the dialog to prints the contents of the webview.
    ///
    /// Currently only supported on macOS on wry. window.print() works on all platforms.
    ///
    /// Requires [`allowlist > window > print`](https://tauri.app/v1/api/config#windowallowlistconfig.print) to be enabled.
    pub fn print(&self) -> crate::Result<()> {
        todo!()
    }

    /// Determines if this window should be resizable.
    ///
    /// Requires [`allowlist > window > setResizable`](https://tauri.app/v1/api/config#windowallowlistconfig.setresizable) to be enabled.
    pub async fn set_resizable(&self, resizable: bool) -> crate::Result<()> {
        Ok(self.0.setResizable(resizable).await?)
    }

    /// Set this window’s title.
    ///
    /// Requires [`allowlist > window > setTitle`](https://tauri.app/v1/api/config#windowallowlistconfig.settitle) to be enabled.
    pub async fn set_title(&self, title: impl AsRef<str>) -> crate::Result<()> {
        Ok(self.0.setTitle(title.as_ref()).await?)
    }

    /// Maximizes this window.
    ///
    /// Requires [`allowlist > window > maximize`](https://tauri.app/v1/api/config#windowallowlistconfig.maximize) to be enabled.
    pub async fn maximize(&self) -> crate::Result<()> {
        Ok(self.0.maximize().await?)
    }

    /// Un-maximizes this window.
    ///
    /// Requires [`allowlist > window > unmaximize`](https://tauri.app/v1/api/config#windowallowlistconfig.unmaximize) to be enabled.
    pub async fn unmaximize(&self) -> crate::Result<()> {
        Ok(self.0.unmaximize().await?)
    }

    pub async fn toggle_maximize(&self) -> crate::Result<()> {
        Ok(self.0.toggleMaximize().await?)
    }

    /// Minimizes this window.
    ///
    /// Requires [`allowlist > window > minimize`](https://tauri.app/v1/api/config#windowallowlistconfig.minimize) to be enabled.
    pub async fn minimize(&self) -> crate::Result<()> {
        Ok(self.0.minimize().await?)
    }

    /// Un-minimizes this window.
    ///
    /// Requires [`allowlist > window > unminimize`](https://tauri.app/v1/api/config#windowallowlistconfig.unminimize) to be enabled.
    pub async fn unminimize(&self) -> crate::Result<()> {
        Ok(self.0.unminimize().await?)
    }

    /// Show this window.
    ///
    /// Requires [`allowlist > window > show`](https://tauri.app/v1/api/config#windowallowlistconfig.show) to be enabled.
    pub async fn show(&self) -> crate::Result<()> {
        Ok(self.0.show().await?)
    }

    /// Hide this window.
    ///
    /// Requires [`allowlist > window > hide`](https://tauri.app/v1/api/config#windowallowlistconfig.hide) to be enabled.
    pub async fn hide(&self) -> crate::Result<()> {
        Ok(self.0.hide().await?)
    }

    /// Closes this window.
    ///
    /// Requires [`allowlist > window > close`](https://tauri.app/v1/api/config#windowallowlistconfig.close) to be enabled.
    pub async fn close(&self) -> crate::Result<()> {
        Ok(self.0.close().await?)
    }

    /// Determines if this window should be [decorated](https://en.wikipedia.org/wiki/Window_(computing)#Window_decoration).
    ///
    /// Requires [`allowlist > window > setDecorations`](https://tauri.app/v1/api/config#windowallowlistconfig.setdecorations) to be enabled.
    pub async fn set_decorations(&self, decorations: bool) -> crate::Result<()> {
        Ok(self.0.setDecorations(decorations).await?)
    }

    /// Determines if this window should always be on top of other windows.
    ///
    /// Requires [`allowlist > window > setAlwaysOnTop`](https://tauri.app/v1/api/config#windowallowlistconfig.setalwaysontop) to be enabled.
    pub async fn set_always_on_top(&self, always_on_top: bool) -> crate::Result<()> {
        Ok(self.0.setAlwaysOnTop(always_on_top).await?)
    }

    /// Resizes this window.
    ///
    /// Requires [`allowlist > window > setSize`](https://tauri.app/v1/api/config#windowallowlistconfig.setsize) to be enabled.
    pub async fn set_size(&self, size: impl Into<Size>) -> crate::Result<()> {
        match size.into() {
            Size::Physical(size) => self.0.setSizePhysical(size.0).await?,
            Size::Logical(size) => self.0.setSizeLogical(size.0).await?,
        }

        Ok(())
    }

    /// Sets this window’s minimum size.
    ///
    /// Requires [`allowlist > window > setMinSize`](https://tauri.app/v1/api/config#windowallowlistconfig.setminsize) to be enabled.
    pub async fn set_min_size(&self, size: Option<impl Into<Size>>) -> crate::Result<()> {
        match size.map(Into::into) {
            None => self.0.setMinSizePhysical(None).await?,
            Some(Size::Physical(size)) => self.0.setMinSizePhysical(Some(size.0)).await?,
            Some(Size::Logical(size)) => self.0.setMinSizeLogical(Some(size.0)).await?,
        }

        Ok(())
    }

    /// Sets this window’s maximum size.
    ///
    /// Requires [`allowlist > window > setMaxSize`](https://tauri.app/v1/api/config#windowallowlistconfig.setmaxsize) to be enabled.
    pub async fn set_max_size(&self, size: Option<impl Into<Size>>) -> crate::Result<()> {
        match size.map(Into::into) {
            None => self.0.setMaxSizePhysical(None).await?,
            Some(Size::Physical(size)) => self.0.setMaxSizePhysical(Some(size.0)).await?,
            Some(Size::Logical(size)) => self.0.setMaxSizeLogical(Some(size.0)).await?,
        }

        Ok(())
    }

    /// Sets this window’s position.
    ///
    /// Requires [`allowlist > window > setPosition`](https://tauri.app/v1/api/config#windowallowlistconfig.setposition) to be enabled.
    pub async fn set_position(&self, position: impl Into<Position>) -> crate::Result<()> {
        match position.into() {
            Position::Physical(pos) => self.0.setPositionPhysical(pos.0).await?,
            Position::Logical(pos) => self.0.setPositionLogical(pos.0).await?,
        }

        Ok(())
    }

    /// Determines if this window should be fullscreen.
    ///
    /// Requires [`allowlist > window > setFullscreen`](https://tauri.app/v1/api/config#windowallowlistconfig.setfullscreen) to be enabled.
    pub async fn set_fullscreen(&self, fullscreen: bool) -> crate::Result<()> {
        Ok(self.0.setFullscreen(fullscreen).await?)
    }

    /// Bring the window to front and focus.
    ///
    /// Requires [`allowlist > window > setFocus`](https://tauri.app/v1/api/config#windowallowlistconfig.setfocus) to be enabled.
    pub async fn set_focus(&self) -> crate::Result<()> {
        Ok(self.0.setFocus().await?)
    }

    /// Sets this window’ icon.
    ///
    /// Requires [`allowlist > window > setIcon`](https://tauri.app/v1/api/config#windowallowlistconfig.seticon) to be enabled.
    pub async fn set_icon(&self, icon: &[u8]) -> crate::Result<()> {
        Ok(self.0.setIcon(icon).await?)
    }

    /// Whether to show the window icon in the task bar or not.
    ///
    /// Requires [`allowlist > window > setSkipTaskbar`](https://tauri.app/v1/api/config#windowallowlistconfig.setskiptaskbar) to be enabled.
    pub async fn set_skip_taskbar(&self, skip: bool) -> crate::Result<()> {
        Ok(self.0.setSkipTaskbar(skip).await?)
    }

    /// Grabs the cursor, preventing it from leaving the window.
    ///
    /// There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you want so.
    ///
    /// #### Platform-specific
    /// - Linux: Unsupported.
    /// - macOS: This locks the cursor in a fixed location, which looks visually awkward.
    ///
    /// Requires [`allowlist > window > setCursorGrab`](https://tauri.app/v1/api/config#windowallowlistconfig.setcursorgrab) to be enabled.
    pub async fn set_cursor_grab(&self, grab: bool) -> crate::Result<()> {
        Ok(self.0.setCursorGrab(grab).await?)
    }

    /// Modifies the cursor’s visibility.
    ///
    /// If false, this will hide the cursor. If true, this will show the cursor.
    ///
    /// #### Platform-specific
    /// - Windows: The cursor is only hidden within the confines of the window.
    /// - macOS: The cursor is hidden as long as the window has input focus, even if the cursor is outside of the window.
    ///
    /// Requires [`allowlist > window > setCursorVisible`](https://tauri.app/v1/api/config#windowallowlistconfig.setcursorvisible) to be enabled.
    pub async fn set_cursor_visible(&self, visible: bool) -> crate::Result<()> {
        Ok(self.0.setCursorVisible(visible).await?)
    }

    /// Modifies the cursor icon of the window.
    ///
    /// Requires [`allowlist > window > setCursorIcon`](https://tauri.app/v1/api/config#windowallowlistconfig.setcursoricon) to be enabled.
    pub async fn set_cursor_icon(&self, icon: CursorIcon) -> crate::Result<()> {
        Ok(self.0.setCursorIcon(&icon.to_string()).await?)
    }

    /// Changes the position of the cursor in window coordinates.
    ///
    /// Requires [`allowlist > window > setCursorPosition`](https://tauri.app/v1/api/config#windowallowlistconfig.setcursorposition) to be enabled.
    pub async fn set_cursor_position(&self, position: Position) -> crate::Result<()> {
        match position {
            Position::Physical(pos) => self.0.setCursorPositionPhysical(pos.0).await?,
            Position::Logical(pos) => self.0.setCursorPositionLogical(pos.0).await?,
        }

        Ok(())
    }

    /// Ignores the window cursor events.
    ///
    /// Requires [`allowlist > window > setIgnoreCursorEvents`](https://tauri.app/v1/api/config#windowallowlistconfig.setignorecursorevents) to be enabled.
    pub async fn set_ignore_cursor_events(&self, ignore: bool) -> crate::Result<()> {
        Ok(self.0.setIgnoreCursorEvents(ignore).await?)
    }

    /// Starts dragging the window.
    ///
    /// Requires [`allowlist > window > startDragging`](https://tauri.app/v1/api/config#windowallowlistconfig.startdragging) to be enabled.
    pub async fn start_dragging(&self) -> crate::Result<()> {
        Ok(self.0.startDragging().await?)
    }

    /// Emits an event to the backend, tied to the webview window.
    #[inline(always)]
    pub async fn emit<T: Serialize>(&self, event: &str, payload: &T) -> crate::Result<()> {
        self.0
            .emit(event, serde_wasm_bindgen::to_value(payload).unwrap())
            .await?;

        Ok(())
    }

    /// Listen to an event emitted by the backend that is tied to the webview window.
    ///
    /// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
    /// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
    #[inline(always)]
    pub async fn listen<T, H>(&self, event: &str) -> crate::Result<impl Stream<Item = Event<T>>>
    where
        T: DeserializeOwned + 'static,
    {
        let (tx, rx) = mpsc::unbounded::<Event<T>>();

        let closure = Closure::<dyn FnMut(JsValue)>::new(move |raw| {
            let _ = tx.unbounded_send(serde_wasm_bindgen::from_value(raw).unwrap());
        });
        let unlisten = self.0.listen(event, &closure).await?;
        closure.forget();

        Ok(Listen {
            rx,
            unlisten: js_sys::Function::from(unlisten),
        })
    }

    /// Listen to an one-off event emitted by the backend that is tied to the webview window.
    ///
    /// The returned Future will automatically clean up it's underlying event listener when dropped, so no manual unlisten function needs to be called.
    /// See [Differences to the JavaScript API](../index.html#differences-to-the-javascript-api) for details.
    #[inline(always)]
    pub async fn once<T>(&self, event: &str) -> crate::Result<Event<T>>
    where
        T: DeserializeOwned + 'static,
    {
        let (tx, rx) = oneshot::channel::<Event<T>>();

        let closure: Closure<dyn FnMut(JsValue)> = Closure::once(move |raw| {
            let _ = tx.send(serde_wasm_bindgen::from_value(raw).unwrap());
        });
        let unlisten = self.0.once(event, &closure).await?;
        closure.forget();

        let fut = Once {
            rx,
            unlisten: js_sys::Function::from(unlisten),
        };

        fut.await
    }
}

/// A position represented in logical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalPosition(inner::LogicalPosition);

impl LogicalPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(inner::LogicalPosition::new(x, y))
    }

    pub fn from_physical(physical: impl Into<PhysicalPosition>, scale_factor: f64) -> Self {
        physical.into().to_logical(scale_factor)
    }

    pub fn to_physical(self, scale_factor: f64) -> PhysicalPosition {
        let x = self.x() as f64 * scale_factor;
        let y = self.y() as f64 * scale_factor;

        PhysicalPosition::new(x as i32, y as i32)
    }

    pub fn x(&self) -> i32 {
        self.0.x()
    }
    pub fn set_x(&self, x: i32) {
        self.0.set_x(x)
    }
    pub fn y(&self) -> i32 {
        self.0.y()
    }
    pub fn set_y(&self, y: i32) {
        self.0.set_y(y)
    }
}

impl From<LogicalPosition> for Position {
    fn from(pos: LogicalPosition) -> Self {
        Position::Logical(pos)
    }
}

/// A position represented in physical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalPosition(inner::PhysicalPosition);

impl PhysicalPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(inner::PhysicalPosition::new(x, y))
    }

    #[inline]
    pub fn from_logical(logical: impl Into<LogicalPosition>, scale_factor: f64) -> Self {
        logical.into().to_physical(scale_factor)
    }

    #[inline]
    pub fn to_logical(&self, scale_factor: f64) -> LogicalPosition {
        let x = self.x() as f64 / scale_factor;
        let y = self.y() as f64 / scale_factor;

        LogicalPosition::new(x as i32, y as i32)
    }

    pub fn x(&self) -> i32 {
        self.0.x()
    }
    pub fn set_x(&self, x: i32) {
        self.0.set_x(x)
    }
    pub fn y(&self) -> i32 {
        self.0.y()
    }
    pub fn set_y(&self, y: i32) {
        self.0.set_y(y)
    }
}

impl From<PhysicalPosition> for Position {
    fn from(pos: PhysicalPosition) -> Self {
        Position::Physical(pos)
    }
}

/// A size represented in logical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicalSize(inner::LogicalSize);

impl LogicalSize {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::LogicalSize::new(x, y))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }
    pub fn set_width(&self, x: u32) {
        self.0.set_width(x)
    }
    pub fn height(&self) -> u32 {
        self.0.height()
    }
    pub fn set_height(&self, y: u32) {
        self.0.set_height(y)
    }
}

impl From<LogicalSize> for Size {
    fn from(size: LogicalSize) -> Self {
        Size::Logical(size)
    }
}

/// A size represented in physical pixels.
#[derive(Debug, Clone, PartialEq)]
pub struct PhysicalSize(inner::PhysicalSize);

impl PhysicalSize {
    pub fn new(x: u32, y: u32) -> Self {
        Self(inner::PhysicalSize::new(x, y))
    }

    pub fn to_logical(self, scale_factor: u32) -> LogicalSize {
        LogicalSize(self.0.toLogical(scale_factor))
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }
    pub fn set_width(&self, x: u32) {
        self.0.set_width(x)
    }
    pub fn height(&self) -> u32 {
        self.0.height()
    }
    pub fn set_height(&self, y: u32) {
        self.0.set_height(y)
    }
}

impl From<PhysicalSize> for Size {
    fn from(size: PhysicalSize) -> Self {
        Size::Physical(size)
    }
}

/// Allows you to retrieve information about a given monitor.
#[derive(Debug, Clone, PartialEq)]
pub struct Monitor(JsValue);

impl Monitor {
    /// Human-readable name of the monitor
    pub fn name(&self) -> Option<String> {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("name")).unwrap();

        raw.as_string()
    }

    /// The monitor's resolution.
    pub fn size(&self) -> PhysicalSize {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("size")).unwrap();

        PhysicalSize(raw.unchecked_into())
    }

    /// The Top-left corner position of the monitor relative to the larger full screen area.
    pub fn position(&self) -> PhysicalPosition {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("position")).unwrap();

        PhysicalPosition(raw.unchecked_into())
    }

    /// The scale factor that can be used to map physical pixels to logical pixels.
    pub fn scale_factor(&self) -> u32 {
        let raw = js_sys::Reflect::get(&self.0, &JsValue::from_str("size"))
            .unwrap()
            .as_f64()
            .unwrap();

        raw as u32
    }
}

/// Get an instance of [`WebviewWindow`] for the current webview window.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::window::current_window;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let win = current_window().await?;
/// # Ok(())
/// # }
/// ```
pub fn current_window() -> WebviewWindow {
    WebviewWindow(inner::getCurrent())
}

/// Gets a list of instances of [`WebviewWindow`] for all available webview windows.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::window::all_windows;
/// use web_sys::console;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let windows = all_windows().await?;
///
/// for win in windows {
///     console::log_1(&format!("{:?}", win).into());
/// }
/// # Ok(())
/// # }
/// ```
pub fn all_windows() -> impl IntoIterator<Item = WebviewWindow> {
    let raw = inner::getAll();

    ArrayIterator::new(raw).map(|r| WebviewWindow(inner::WebviewWindow::from(r)))
}

/// Returns the monitor on which the window currently resides.
///
/// Returns `None` if current monitor can't be detected.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::window::current_monitor;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let monitor = current_monitor().await?;
/// # Ok(())
/// # }
/// ```
pub async fn current_monitor() -> crate::Result<Option<Monitor>> {
    let raw = inner::currentMonitor().await?;

    if raw.is_null() {
        Ok(None)
    } else {
        Ok(Some(Monitor(raw)))
    }
}

/// Returns the primary monitor of the system.
///
/// Returns `None` if it can't identify any monitor as a primary one.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::window::primary_monitor;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let monitor = primary_monitor().await?;
/// # Ok(())
/// # }
/// ```
pub async fn primary_monitor() -> crate::Result<Option<Monitor>> {
    let raw = inner::primaryMonitor().await?;

    if raw.is_null() {
        Ok(None)
    } else {
        Ok(Some(Monitor(raw)))
    }
}

/// Returns the list of all the monitors available on the system.
///
/// # Example
///
/// ```rust,no_run
/// use tauri_sys::window::available_monitors;
/// use web_sys::console;
///
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let monitors = available_monitors().await?;
///
/// for monitor in monitors {
///     console::log_1(&format!("{:?}", monitor).into());
/// }
/// # Ok(())
/// # }
/// ```
pub async fn available_monitors() -> crate::Result<impl Iterator<Item = Monitor>> {
    let raw = inner::availableMonitors().await?;
    let raw = Array::try_from(raw).unwrap();

    let monitors = ArrayIterator::new(raw).map(Monitor);

    Ok(monitors)
}

mod inner {
    use js_sys::Array;
    use wasm_bindgen::{
        prelude::{wasm_bindgen, Closure},
        JsValue,
    };

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type LogicalPosition;
        #[wasm_bindgen(constructor)]
        pub fn new(x: i32, y: i32) -> LogicalPosition;
        #[wasm_bindgen(method, getter)]
        pub fn x(this: &LogicalPosition) -> i32;
        #[wasm_bindgen(method, setter)]
        pub fn set_x(this: &LogicalPosition, x: i32);
        #[wasm_bindgen(method, getter)]
        pub fn y(this: &LogicalPosition) -> i32;
        #[wasm_bindgen(method, setter)]
        pub fn set_y(this: &LogicalPosition, y: i32);
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type PhysicalPosition;
        #[wasm_bindgen(constructor)]
        pub fn new(x: i32, y: i32) -> PhysicalPosition;
        #[wasm_bindgen(method)]
        pub fn toLogical(this: &PhysicalPosition, scaleFactor: i32) -> LogicalPosition;
        #[wasm_bindgen(method, getter)]
        pub fn x(this: &PhysicalPosition) -> i32;
        #[wasm_bindgen(method, setter)]
        pub fn set_x(this: &PhysicalPosition, x: i32);
        #[wasm_bindgen(method, getter)]
        pub fn y(this: &PhysicalPosition) -> i32;
        #[wasm_bindgen(method, setter)]
        pub fn set_y(this: &PhysicalPosition, y: i32);
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type LogicalSize;
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32) -> LogicalSize;
        #[wasm_bindgen(method, getter)]
        pub fn width(this: &LogicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_width(this: &LogicalSize, width: u32);
        #[wasm_bindgen(method, getter)]
        pub fn height(this: &LogicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_height(this: &LogicalSize, height: u32);
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type PhysicalSize;
        #[wasm_bindgen(constructor)]
        pub fn new(width: u32, height: u32) -> PhysicalSize;
        #[wasm_bindgen(method)]
        pub fn toLogical(this: &PhysicalSize, scaleFactor: u32) -> LogicalSize;
        #[wasm_bindgen(method, getter)]
        pub fn width(this: &PhysicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_width(this: &PhysicalSize, width: u32);
        #[wasm_bindgen(method, getter)]
        pub fn height(this: &PhysicalSize) -> u32;
        #[wasm_bindgen(method, setter)]
        pub fn set_height(this: &PhysicalSize, height: u32);
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[derive(Debug, Clone, PartialEq)]
        pub type WebviewWindowHandle;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str) -> WebviewWindowHandle;
        #[wasm_bindgen(method, catch)]
        pub async fn listen(
            this: &WebviewWindowHandle,
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn once(
            this: &WebviewWindowHandle,
            event: &str,
            handler: &Closure<dyn FnMut(JsValue)>,
        ) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn emit(
            this: &WebviewWindowHandle,
            event: &str,
            payload: JsValue,
        ) -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[wasm_bindgen(extends = WebviewWindowHandle)]
        #[derive(Debug, Clone, PartialEq)]
        pub type WindowManager;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str) -> WindowManager;
        #[wasm_bindgen(method, getter)]
        pub fn label(this: &WindowManager) -> String;
        #[wasm_bindgen(method, catch)]
        pub async fn scaleFactor(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn innerPosition(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn outerPosition(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn innerSize(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn outerSize(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn isFullscreen(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn isMaximized(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn isDecorated(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn isResizable(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn isVisible(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn theme(this: &WindowManager) -> Result<JsValue, JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn center(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn requestUserAttention(
            this: &WindowManager,
            requestType: u32,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setResizable(this: &WindowManager, resizable: bool) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setTitle(this: &WindowManager, title: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn maximize(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn unmaximize(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn toggleMaximize(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn minimize(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn unminimize(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn show(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn hide(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn close(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setDecorations(this: &WindowManager, decorations: bool)
            -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setAlwaysOnTop(this: &WindowManager, alwaysOnTop: bool)
            -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setSize, catch)]
        pub async fn setSizePhysical(
            this: &WindowManager,
            size: PhysicalSize,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setSize, catch)]
        pub async fn setSizeLogical(this: &WindowManager, size: LogicalSize)
            -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setMinSize, catch)]
        pub async fn setMinSizePhysical(
            this: &WindowManager,
            size: Option<PhysicalSize>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setMinSize, catch)]
        pub async fn setMinSizeLogical(
            this: &WindowManager,
            size: Option<LogicalSize>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setMaxSize, catch)]
        pub async fn setMaxSizePhysical(
            this: &WindowManager,
            size: Option<PhysicalSize>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setMinSize, catch)]
        pub async fn setMaxSizeLogical(
            this: &WindowManager,
            size: Option<LogicalSize>,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setPosition, catch)]
        pub async fn setPositionPhysical(
            this: &WindowManager,
            position: PhysicalPosition,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setPosition, catch)]
        pub async fn setPositionLogical(
            this: &WindowManager,
            position: LogicalPosition,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setFullscreen(this: &WindowManager, fullscreen: bool) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setFocus(this: &WindowManager) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setIcon(this: &WindowManager, icon: &[u8]) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setSkipTaskbar(this: &WindowManager, skip: bool) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setCursorGrab(this: &WindowManager, grab: bool) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setCursorVisible(this: &WindowManager, visible: bool) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setCursorIcon(this: &WindowManager, icon: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setCursorPosition, catch)]
        pub async fn setCursorPositionPhysical(
            this: &WindowManager,
            position: PhysicalPosition,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, js_name = setCursorPosition, catch)]
        pub async fn setCursorPositionLogical(
            this: &WindowManager,
            position: LogicalPosition,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn setIgnoreCursorEvents(
            this: &WindowManager,
            ignore: bool,
        ) -> Result<(), JsValue>;
        #[wasm_bindgen(method, catch)]
        pub async fn startDragging(this: &WindowManager) -> Result<(), JsValue>;
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        #[wasm_bindgen(extends = WindowManager)]
        #[derive(Debug, Clone, PartialEq)]
        pub type WebviewWindow;
        #[wasm_bindgen(constructor)]
        pub fn new(label: &str, options: JsValue) -> WebviewWindow;
        #[wasm_bindgen(static_method_of = WebviewWindow)]
        pub fn getByLabel(label: &str) -> Option<WebviewWindow>;
    }

    #[wasm_bindgen(module = "/src/window.js")]
    extern "C" {
        pub fn getCurrent() -> WebviewWindow;
        pub fn getAll() -> Array;
        #[wasm_bindgen(catch)]
        pub async fn currentMonitor() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn primaryMonitor() -> Result<JsValue, JsValue>;
        #[wasm_bindgen(catch)]
        pub async fn availableMonitors() -> Result<JsValue, JsValue>;
    }
}
