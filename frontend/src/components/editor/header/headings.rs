use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use gloo::utils::document;
use wasm_bindgen::JsCast;

use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::contexts::markdown::{use_markdown, Markdown};
use crate::icons::{HeadingIcon, Heading1Icon, Heading2Icon, Heading3Icon, Heading4Icon, Heading5Icon, Heading6Icon};

#[function_component(HeadingsDropdown)]
pub fn headings_dropdown() -> Html {
    let md_state = use_markdown();
    let heading_1 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "# ");
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("#");
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_2 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "## ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 3));
        } else {
            current_value.push_str("#");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(3));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_3 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;
            current_value.insert_str(start_usize, "### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 4));
        } else {
            current_value.push_str("### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(4));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_4 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "#### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 5));
        } else {
            current_value.push_str("#### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(5));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_5 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "##### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 6));
        } else {
            current_value.push_str("##### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(6));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_6 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "###### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 7));
        } else {
            current_value.push_str("###### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(7));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    html! {
        <div class="dropdown dropdown-hover">
            <label tabindex="0" class={HEADER_BTN_CLASSES}>
                <HeadingIcon /></label>
            <ul tabindex="0"
                class="dropdown-content menu p-2 menu-lg bg-base-200 rounded-box shadow bg-base-100 rounded-box w-max">
                <li onclick={heading_1}>
                    <div class="flex flex-row">
                        <Heading1Icon />
                        {"Heading 1"}
                    </div>
                </li>
                <li onclick={heading_2}>
                    <div class="flex flex-row">
                        <Heading2Icon />
                        {"Heading 2"}
                    </div>
                </li>
                <li onclick={heading_3}>
                    <div class="flex flex-row">
                        <Heading3Icon />
                        {"Heading 3"}
                    </div>
                </li>
                <li onclick={heading_4}>
                    <div class="flex flex-row">
                        <Heading4Icon />
                        {"Heading 4"}
                    </div>
                </li>
                <li onclick={heading_5}>
                    <div class="flex flex-row">
                        <Heading5Icon />
                        {"Heading 5"}
                    </div>
                </li>
                <li onclick={heading_6}>
                    <div class="flex flex-row">
                        <Heading6Icon />
                        {"Heading 6"}
                    </div>
                </li>
            </ul>
        </div>
    }
}