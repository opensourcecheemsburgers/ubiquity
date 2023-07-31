use crate::components::{modals::utils::close_modal, editor::textarea::textarea::EDITOR_ID};
use yew::prelude::*;
use crate::contexts::{markdown::{use_markdown, Markdown}, toasts::{use_toaster, err_modal}};
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use gloo::utils::document;
use wasm_bindgen::JsCast;


pub const ADD_IMAGE_MODAL_ID: AttrValue = AttrValue::Static("add_image_modal");

const TITLE_TEXTBOX_ID: AttrValue = AttrValue::Static("image_title_input"); 
const PLACEHOLDER_TITLE: AttrValue = AttrValue::Static("A Super Duper Cool Image");

const LINK_TEXTBOX_ID: AttrValue = AttrValue::Static("image_link_input"); 
const PLACEHOLDER_LINK: AttrValue = AttrValue::Static("https://i.ytimg.com/vi/gvm-QXS3yjY/maxresdefault.jpg");


#[function_component(AddImageModal)]
pub fn add_image_modal() -> Html {
    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    
    let key = use_markdown().state().key;
    let modal_id = AttrValue::from("add_image_modal");
    
    let insert_link = Callback::from(move |_| {
        let link_title_input: HtmlInputElement = document().get_element_by_id(&TITLE_TEXTBOX_ID).unwrap().dyn_into().unwrap();
        let link_title = link_title_input.value();

        let link_input: HtmlInputElement = document().get_element_by_id(&LINK_TEXTBOX_ID).unwrap().dyn_into().unwrap();
        let link = link_input.value();

        let link_str = format!("![{}]({})", link_title, link);

        let editor: HtmlTextAreaElement = document().get_element_by_id(&EDITOR_ID).unwrap().dyn_into().unwrap();
        let mut current_value = editor.value();

        if let Some(end) = editor.selection_end().unwrap() {
            let end_usize = end as usize;
            current_value.insert_str(end_usize, &link_str);
        } else {
            current_value.push_str(&link_str);
        }

        let new_md = Markdown::from(AttrValue::from(current_value), key.clone());
        markdown_ctx.update_markdown(new_md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
        close_modal(&ADD_IMAGE_MODAL_ID);
    });

    html! {
        <>
            <input type="checkbox" id={ADD_IMAGE_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Insert Image"}</h3>
                    <div class="flex flex-col space-y-4 mt-4">
                        <p>{"Insert a link into your markdown using the textboxes below."}</p>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Image Title"}</span>
                            </label>
                            <input id={TITLE_TEXTBOX_ID} type="text" placeholder={PLACEHOLDER_TITLE}
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Image Link"}</span>
                            </label>
                            <input id={LINK_TEXTBOX_ID} placeholder={PLACEHOLDER_LINK}
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                    </div>

                    <div class="modal-action">
                        <label for={ADD_IMAGE_MODAL_ID} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={insert_link} class="btn">{"Insert"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}