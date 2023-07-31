use crate::components::{modals::utils::close_modal, editor::textarea::textarea::EDITOR_ID};
use yew::prelude::*;
use crate::contexts::{markdown::{use_markdown, Markdown}, toasts::{use_toaster, err_modal}};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use gloo::utils::document;
use wasm_bindgen::JsCast;


pub const ADD_LINK_MODAL_ID: AttrValue = AttrValue::Static("add_link_modal");

const LINK_TITLE_TEXTBOX_ID: AttrValue = AttrValue::Static("link_title_input"); 
const PLACEHOLDER_TITLE: AttrValue = AttrValue::Static("A Super Duper Cool Link");

const LINK_TEXTBOX_ID: AttrValue = AttrValue::Static("link_input"); 
const PLACEHOLDER_LINK: AttrValue = AttrValue::Static("https://youtu.be/wsmHCfSZM70");


#[function_component(AddLinkModal)]
pub fn add_link_modal() -> Html {
    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    
    let key = use_markdown().state().key;
    

    let insert_link = Callback::from(move |_| {
        let title_input: HtmlInputElement = document().get_element_by_id(&LINK_TITLE_TEXTBOX_ID).unwrap().dyn_into().unwrap();
        let title = title_input.value();

        let link_input: HtmlInputElement = document().get_element_by_id(&LINK_TEXTBOX_ID).unwrap().dyn_into().unwrap();
        let link = link_input.value();

        let link_str = format!("[{}]({})", title, link);

        let editor: HtmlTextAreaElement = document().get_element_by_id(&EDITOR_ID).unwrap().dyn_into().unwrap();
        let mut current_value = editor.value();

        if let Some(end) = editor.selection_end().unwrap() {
            let end_usize = end as usize;
            current_value.insert_str(end_usize, &link_str);
        } else {
            current_value.push_str(&link_str);
        }

        let new_md = Markdown::from(AttrValue::from(current_value), key.clone());
        markdown_ctx.update_markdown(new_md).unwrap_or_else(|err| err_modal(err, toaster.clone()));;
        close_modal(&ADD_LINK_MODAL_ID);
    });

    html! {
        <>
            <input type="checkbox" id={ADD_LINK_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Insert Link"}</h3>
                    <div class="flex flex-col space-y-4 mt-4">
                        <p>{"Insert a link into your markdown using the textboxes below."}</p>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Link Title"}</span>
                            </label>
                            <input id="link_title_input" type="text" placeholder={PLACEHOLDER_TITLE}
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Link"}</span>
                            </label>
                            <input id="link_input" type="text" placeholder={PLACEHOLDER_LINK}
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                    </div>

                    <div class="modal-action">
                        <label for={ADD_LINK_MODAL_ID} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={insert_link} class="btn">{"Insert"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}