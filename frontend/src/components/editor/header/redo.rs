use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::components::tooltip::Tooltip;
use crate::icons::RedoIcon;
use gloo::utils::document;

#[function_component(RedoBtn)]
pub fn redo_btn() -> Html {
    let redo = Callback::from(|_| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("redo").unwrap();
    });


    html! {
        <Tooltip tip={"Redo"}>
            <btn onclick={redo} class={HEADER_BTN_CLASSES}>
                <RedoIcon />
            </btn>
        </Tooltip>
    }
}
