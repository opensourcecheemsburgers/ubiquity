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
    icons::{AddFileIcon, RedoIcon, UndoIcon, SaveIcon, HamburgerIcon, WrenchIcon},
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
                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                        <svg width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="13.5" cy="6.5" r=".5"></circle>
                            <circle cx="17.5" cy="10.5" r=".5"></circle>
                            <circle cx="8.5" cy="7.5" r=".5"></circle>
                            <circle cx="6.5" cy="12.5" r=".5"></circle>
                            <path
                                d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z">
                            </path>
                        </svg>
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
                        <ThemeDropdownItem name={"aqua"} />
                        <ThemeDropdownItem name={"night"} />
                        <ThemeDropdownItem name={"synthwave"} />
                        <ThemeDropdownItem name={"winter"} />
                        <li>
                            <a onclick={settings_cb.clone()}>{"More..."}</a>
                        </li>
                    </ul>
                </div>

                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                            class="inline-block w-5 h-5 stroke-current">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="1"
                                d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z" />
                        </svg>
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
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

    let header_icon_size = AttrValue::from("32");

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