use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use crate::components::tooltip::Tooltip;
use crate::icons::RedoIcon;
use gloo::utils::document;

use super::header::HeaderBtnProps;

#[function_component(RedoBtn)]
pub fn redo_btn(props: &HeaderBtnProps) -> Html {
    let redo = Callback::from(|_| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("redo").unwrap();
    });

    html! {
        <Tooltip tip={"Redo"}>
            <btn onclick={redo} class={props.btn_classes}>
                <RedoIcon />
            </btn>
        </Tooltip>
    }
}
