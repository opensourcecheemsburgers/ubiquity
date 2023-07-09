use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::contexts::config::use_config;
use crate::contexts::markdown::{use_markdown, Markdown};
use crate::icons::{BoldIcon, ItalicsIcon, FormatIcon, QuoteIcon};
use gloo::utils::document;

#[function_component(FormattingDropdown)]
pub fn bold_btn() -> Html {    
    let is_mobile_ui = use_config().is_mobile_ui();
    let mut dropdown_classes = classes!("dropdown");
    match is_mobile_ui {
        true => dropdown_classes.push("dropdown-end"),
        false => dropdown_classes.push("dropdown-hover"),
    }

    html! {
        <div class={dropdown_classes}>
            <label class={HEADER_BTN_CLASSES} id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                <FormatIcon size={AttrValue::from("24")} />
            </label>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-200 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <BoldBtn />
                    <ItalicsBtn />
                    <QuoteBtn />
                </ul>
            </div>
        </div>
    }
}

#[function_component(BoldBtn)]
pub fn bold_btn() -> Html {
    let md_state = use_markdown();
    let bold = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "**");
            current_value.insert_str(end_usize + 2, "**");
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("****");
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    html! {
        <li>
            <div onclick={bold}>
                <BoldIcon />
                {"Bold"}
            </div>
        </li>
    }
}

#[function_component(ItalicsBtn)]
pub fn italics_btn() -> Html {
    let md_state = use_markdown();
    let italics = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "*");
            current_value.insert_str(end_usize + 1, "*");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 1));
        } else {
            current_value.push_str("**");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(1));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    html! {
        <li>
            <div onclick={italics}>
                <ItalicsIcon />
                {"Italics"}
            </div>
        </li>
    }
}

#[function_component(QuoteBtn)]
pub fn quote_btn() -> Html {
    let md_state = use_markdown();
    let quote = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "> ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("> ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    html! {
        <li>
            <btn onclick={quote}>
                <QuoteIcon />
                {"Quote"}
            </btn>
        </li>
    }
}
