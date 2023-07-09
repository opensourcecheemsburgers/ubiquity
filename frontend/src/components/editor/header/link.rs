use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::components::modals::{AddLinkModal, AddImageModal};
use crate::components::tooltip::Tooltip;
use crate::contexts::config::use_config;
use crate::contexts::markdown::{use_markdown, Markdown};
use crate::icons::{LinkIcon, ImageIcon};
use gloo::utils::document;

#[function_component(AddLinkBtn)]
pub fn add_link_btn() -> Html {
    let is_mobile_ui = use_config().is_mobile_ui();
    let mut dropdown_classes = classes!("dropdown");
    match is_mobile_ui {
        true => dropdown_classes.push("dropdown-end"),
        false => dropdown_classes.push("dropdown-hover"),
    }

    html! {
        <div class={dropdown_classes}>
            <label class={HEADER_BTN_CLASSES} id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                <LinkIcon size={AttrValue::from("24")} />
            </label>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <LinkBtn />
                    <ImageBtn />
                </ul>
            </div>
        </div>
    }
}

#[function_component(LinkBtn)]
pub fn link_btn() -> Html {
    html! {
        <li>
            <label for="add_link_modal">
                <LinkIcon />
                {"Link"}
            </label>
        </li>
    }
}

#[function_component(ImageBtn)]
pub fn image_btn() -> Html {
    html! {
        <li>
            <label for="add_image_modal">
                <ImageIcon />
                {"Image"}
            </label>
        </li>
    }
}