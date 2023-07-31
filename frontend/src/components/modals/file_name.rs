use yew::prelude::*;
use crate::contexts::{markdown::{use_markdown, Markdown}, toasts::{use_toaster, err_modal}};
use web_sys::HtmlInputElement;
use gloo::utils::document;
use wasm_bindgen::JsCast;

pub const SAVE_MODAL_ID: AttrValue = AttrValue::Static("save-modal");

const TEXTBOX_ID: AttrValue = AttrValue::Static("name_input"); 
const PLACEHOLDER_NAME: AttrValue = AttrValue::Static("superdupercoolfile.md");


#[function_component(SelectNameModal)]
pub fn select_name_modal() -> Html {
    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    let text = use_markdown().state().text;

    let set_name = Callback::from(move |_| {
        let text = text.clone();
        
        let file_name_textbox: HtmlInputElement = document().get_element_by_id(&TEXTBOX_ID).unwrap().dyn_into().unwrap();
        let file_name = file_name_textbox.value();
        let key = Some(AttrValue::from(file_name));

        let markdown = Markdown::from(text, key);
        markdown_ctx.add_markdown(markdown.clone()).unwrap_or_else(|err| err_modal(err, toaster.clone()));
        markdown_ctx.set_markdown(markdown).unwrap_or_else(|err| err_modal(err, toaster.clone()));
        
        let modal: HtmlInputElement = document().get_element_by_id(&SAVE_MODAL_ID).unwrap().dyn_into().unwrap();
        modal.set_checked(false);
    });

    html! {
        <>
            <input type="checkbox" id={SAVE_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Name"}</h3>
                    <p class="py-4">{"Choose a name using the textbox."}</p>
                    <input id={TEXTBOX_ID} type="text" placeholder={PLACEHOLDER_NAME} class="input input-bordered input-primary w-full max-w-xs" />
                    <div class="modal-action">
                        <label for={SAVE_MODAL_ID} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={set_name} class="btn">{"Set"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}