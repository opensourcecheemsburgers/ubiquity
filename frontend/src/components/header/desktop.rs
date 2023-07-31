use yew::prelude::*;
use yew_router::prelude::use_navigator;
use urlencoding::encode;
use crate::components::tooltip::Tooltip;

use crate::{
    components::{theme_card::ThemeDropdownItem, header::{save_btn::SaveBtn, add_dropdown::AddFileDropdown}},
    icons::{EllipsisIcon, PaletteIcon, RESPONSIVE_ICON_LG},
    Page, contexts::markdown::use_markdown,
};


pub const DOWNLOAD_ANCHOR_ID: AttrValue = AttrValue::Static("dl");
pub const PDF_DOWNLOAD_ANCHOR_ID: AttrValue = AttrValue::Static("pdf_dl");


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

    let text_dl = format!("data:attachment/text,{}", encoded_md);

    let download_name = use_markdown().state().key;

    let dropdown_classes = classes!("dropdown", "dropdown-end");

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
                    <Tooltip tip={"Themes"}>
                        <label tabindex="0" class="btn btn-ghost rounded-btn">
                            <PaletteIcon classes={RESPONSIVE_ICON_LG} />
                        </label>
                    </Tooltip>
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
            <a id={DOWNLOAD_ANCHOR_ID} class="hidden" href={text_dl} download={download_name} target="_blank" />
        </div>
    }
}