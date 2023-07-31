use crate::components::modals::utils::close_modal;
use yew::prelude::*;
use crate::contexts::{markdown::{use_markdown, Markdown}, toasts::{use_toaster, err_modal}};
use web_sys::HtmlInputElement;
use gloo::utils::document;
use wasm_bindgen::JsCast;


pub const CREATE_FILE_MODAL_ID: AttrValue = AttrValue::Static("create_file_modal");

const NAME_TEXTBOX_ID: AttrValue = AttrValue::Static("create_file_name_input"); 
const PLACEHOLDER_NAME: AttrValue = AttrValue::Static("superdupercoolfile.md");


#[function_component(CreateFileModal)]
pub fn create_file_modal() -> Html {
    let markdown_ctx = use_markdown();
    let toaster = use_toaster();

    let create = Callback::from(move |_| {
        let input: HtmlInputElement = document().get_element_by_id(&NAME_TEXTBOX_ID).unwrap().dyn_into().unwrap();
        if input.value().len() > 0 {
            let text = AttrValue::from("");
            let key = Some(AttrValue::from(input.value()));
            let markdown = Markdown::from(text, key);
            markdown_ctx.add_markdown(markdown.clone()).unwrap_or_else(|err| err_modal(err, toaster.clone()));
            markdown_ctx.set_markdown(markdown).unwrap_or_else(|err| err_modal(err, toaster.clone()));
        }
        close_modal(&CREATE_FILE_MODAL_ID);
    });

    html! {
        <>
            <input type="checkbox" id={CREATE_FILE_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Create File"}</h3>
                    <p class="py-4">{"Choose a name for your new markdown file."}</p>
                    <input id={NAME_TEXTBOX_ID} type="text" placeholder={PLACEHOLDER_NAME} class="input input-bordered input-primary w-full max-w-xs" />
                    <div class="modal-action">
                        // <button class="btn">{"Cancel"}</button>
                        <label for={CREATE_FILE_MODAL_ID} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={create} class="btn">{"Create"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}