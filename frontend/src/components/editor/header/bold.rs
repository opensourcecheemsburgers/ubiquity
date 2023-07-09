use wasm_bindgen::JsCast;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::components::tooltip::Tooltip;
use crate::contexts::markdown::{use_markdown, Markdown};
use crate::icons::BoldIcon;
use gloo::utils::document;

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
        <Tooltip tip={"Bold"}>
            <btn onclick={bold} class={HEADER_BTN_CLASSES}>
                <BoldIcon />
            </btn>
        </Tooltip>
    }
}
