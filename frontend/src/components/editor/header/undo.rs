use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use crate::components::tooltip::Tooltip;
use crate::icons::UndoIcon;
use gloo::utils::document;

use super::header::HeaderBtnProps;

#[function_component(UndoBtn)]
pub fn undo_btn(props: &HeaderBtnProps) -> Html {
    let undo = Callback::from(|_| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("undo").unwrap();
    });
    
    html! {
        <Tooltip tip={"Undo"}>
            <btn onclick={undo} class={props.btn_classes}>
                <UndoIcon />
            </btn>
        </Tooltip>
    }
}
