use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use crate::contexts::markdown::{use_markdown, Markdown};
use crate::contexts::toasts::{use_toaster, err_modal};
use crate::icons::{HeadingIcon, Heading1Icon, Heading2Icon, Heading3Icon, Heading4Icon, Heading5Icon, Heading6Icon};

use super::header::HeaderBtnProps;

#[function_component(HeadingsDropdown)]
pub fn headings_dropdown(props: &HeaderBtnProps) -> Html {
    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_1 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "# ");
            text_area.set_selection_end(Some(start + 2)).unwrap();
        } else {
            current_value.push_str("#");
            text_area.set_selection_end(Some(2)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_2 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "## ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 3)).unwrap();
        } else {
            current_value.push_str("#");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(3)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_3 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;
            
            current_value.insert_str(start_usize, "### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 4)).unwrap();
        } else {
            current_value.push_str("### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(4)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_4 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "#### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 5)).unwrap();
        } else {
            current_value.push_str("#### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(5)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_5 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "##### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 6)).unwrap();
        } else {
            current_value.push_str("##### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(6)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });

    let md_state = use_markdown();
    let toaster = use_toaster();
    let heading_6 = Callback::from(move |_mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let _end_usize = end as usize;

            current_value.insert_str(start_usize, "###### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(start + 7)).unwrap();
        } else {
            current_value.push_str("###### ");
            text_area.set_value(&current_value);
            text_area.set_selection_end(Some(7)).unwrap();
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
    });
    
    html! {
        <div class="dropdown">
            <label tabindex="0" class={props.btn_classes}>
                <HeadingIcon />
            </label>
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