pub mod save_btn;
pub mod add_dropdown;

use wasm_bindgen::JsCast;
use web_sys::{HtmlInputElement, HtmlLabelElement, HtmlDivElement, KeyboardEventInit, HtmlDocument};
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use gloo::{file::{Blob, futures::read_as_text}, utils::{document, window}};
use urlencoding::encode;

use crate::{
    components::{divider::DividerYAxis, theme_card::ThemeDropdownItem, tooltip::Tooltip, header::{save_btn::SaveBtn, add_dropdown::AddFileDropdown}},
    icons::{AddFileIcon, RedoIcon, UndoIcon, SaveIcon, HamburgerIcon, WrenchIcon, EllipsisIcon, PaletteIcon},
    Page, contexts::{markdown::{use_markdown, Markdown}, config::use_config},
};
use wasm_bindgen_futures::spawn_local;

#[function_component(Header)]
pub fn header() -> Html {
    let mobile_ui = use_config().is_mobile_ui();

    html! {
        if mobile_ui {
            <MobileHeader />
        } else {
            <DesktopHeader />
        }
    }
}

#[function_component(DesktopHeader)]
pub fn desktop_header() -> Html {
    let nav = use_navigator().unwrap();
    let home_cb= Callback::from(move |_| nav.replace(&Page::Home));

    let nav = use_navigator().unwrap();
    let settings_cb = Callback::from(move |_| nav.push(&Page::Settings));
    
    let nav = use_navigator().unwrap();
    let about_cb = Callback::from(move |_| nav.push(&Page::About));

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let download_name = use_markdown().state().key;

    let mut dropdown_classes = classes!("dropdown", "dropdown-end");
    if !use_config().is_mobile_ui() {
        dropdown_classes.push("dropdown-hover");
    }

    html! {
        <div class="navbar bg-base-300">
            <div class="navbar-start">
                <AddFileDropdown />
                <SaveBtn />
            </div>

            <div class="navbar-center">
                <btn class="btn btn-ghost normal-case font-display font-normal text-3xl" onclick={home_cb}>
                    {"Ubiquity"}
                </btn>
            </div>

            <div class="navbar-end">
                <div class={dropdown_classes.clone()}>
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                        <PaletteIcon />
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-200 rounded-box w-52">
                        <ThemeDropdownItem name={"aqua"} />
                        <ThemeDropdownItem name={"night"} />
                        <ThemeDropdownItem name={"synthwave"} />
                        <ThemeDropdownItem name={"winter"} />
                        <li>
                            <a onclick={settings_cb.clone()}>{"More..."}</a>
                        </li>
                    </ul>
                </div>

                <div class={dropdown_classes}>
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                    <EllipsisIcon />
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-200 rounded-box w-52">
                        <li><a onclick={settings_cb}>{"Settings"}</a></li>
                        <li><a onclick={about_cb}>{"About"}</a></li>
                    </ul>
                </div>
            </div>
            <a id={"dl"} class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </div>
    }
}

#[function_component(MobileHeader)]
pub fn mobile_header() -> Html {
    let header_classes = classes!(
        "flex",
        "flex-row",
        "h-[48px]",
        "justify-between",
        "items-center",
        "w-screen",
        "bg-base-100",
    );
    let header_end_classes = classes!("flex", "flex-row", "justify-end", "-space-x-1");
    let header_btn_classes = classes!("btn", "btn-ghost");

    let nav = use_navigator().unwrap();
    let settings_cb = Callback::from(move |_| nav.push(&Page::Settings));

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let download_name = use_markdown().state().key;

    html! {
        <div class={header_classes}>
            <label for="drawer-input" class={header_btn_classes.clone()}>
                <HamburgerIcon />
            </label>
            <div class={header_end_classes}>
                <AddFileDropdown />
                <SaveBtn />
                <btn onclick={settings_cb} class={header_btn_classes}>
                    <WrenchIcon />
                </btn>
            </div>
            <a id={"dl"} class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </div>
    }
}