use std::fmt::format;

use yew::prelude::*;
use crate::contexts::markdown::{use_markdown, Markdown};
use web_sys::{HtmlInputElement, HtmlAnchorElement, HtmlTextAreaElement};
use gloo::{utils::document};
use wasm_bindgen::JsCast;

#[function_component(Modals)]
pub fn modals() -> Html {
    html! {
        <>
            <CreateFileModal />
            <SelectNameModal />
            <AddLinkModal />
            <AddImageModal />
        </>
    }
}

#[function_component(CreateFileModal)]
pub fn create_file_modal() -> Html {
    let markdown_ctx = use_markdown();

    let create = Callback::from(move |_| {
        let input: HtmlInputElement = document().get_element_by_id("create_name_input").unwrap().dyn_into().unwrap();
        if input.value().len() > 0 {
            let text = AttrValue::from("");
            let key = Some(AttrValue::from(input.value()));
            let markdown = Markdown::from(text, key);
            markdown_ctx.add_markdown(markdown);
        }
        let modal: HtmlInputElement = document().get_element_by_id("create-file").unwrap().dyn_into().unwrap();
        modal.set_checked(false);
    });

    let modal_id = AttrValue::from("create-file");

    html! {
        <>
        <input type="checkbox" id={&modal_id} class="modal-toggle" />
        <div class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-2xl">{"Create File"}</h3>
                <p class="py-4">{"Choose a name for your new markdown file."}</p>
                <input id="create_name_input" type="text" placeholder="superdupercoolfile.md" class="input input-bordered input-primary w-full max-w-xs" />
                <div class="modal-action">
                    // <button class="btn">{"Cancel"}</button>
                    <label for={&modal_id} class="btn btn-ghost">{"Cancel"}</label>
                    <button onclick={create} class="btn">{"Create"}</button>
                </div>
            </div>
        </div>
        </>
    }
}

#[function_component(SelectNameModal)]
pub fn select_name_modal() -> Html {
    let markdown_ctx = use_markdown();
    let text = use_markdown().state().text;
    let set_name = Callback::from(move |_| {
        let text = text.clone();
        let anchor: HtmlInputElement = document().get_element_by_id("name_input").unwrap().dyn_into().unwrap();
        let key = Some(AttrValue::from(anchor.value()));
        let md = Markdown::from(text, key);
        markdown_ctx.add_markdown(md);
        let modal: HtmlInputElement = document().get_element_by_id("save-modal").unwrap().dyn_into().unwrap();
        modal.set_checked(false);
    });

    let modal_id = AttrValue::from("save-modal");

    html! {
        <>
        <input type="checkbox" id={&modal_id} class="modal-toggle" />
        <div class="modal">
            <div class="modal-box">
                <h3 class="font-bold text-2xl">{"Name"}</h3>
                <p class="py-4">{"Choose a name using the textbox."}</p>
                <input id="name_input" type="text" placeholder="superdupercoolfile.md" class="input input-bordered input-primary w-full max-w-xs" />
                <div class="modal-action">
                    <label for={&modal_id} class="btn btn-ghost">{"Cancel"}</label>
                    <button onclick={set_name} class="btn">{"Set"}</button>
                </div>
            </div>
        </div>
        </>
    }
}

#[function_component(AddLinkModal)]
pub fn add_link_modal() -> Html {
    let markdown_ctx = use_markdown();
    let key = use_markdown().state().key;

    let modal_id = AttrValue::from("add_link_modal");
    let insert_link = Callback::from(move |_| {
        let link_title_input: HtmlInputElement = document().get_element_by_id("link_title_input").unwrap().dyn_into().unwrap();
        let link_title = link_title_input.value();

        let link_input: HtmlInputElement = document().get_element_by_id("link_input").unwrap().dyn_into().unwrap();
        let link = link_input.value();

        let link_str = format!("[{}]({})", link_title, link);

        let editor: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = editor.value();

        if let Some(end) = editor.selection_end().unwrap() {
            let end_usize = end as usize;
            current_value.insert_str(end_usize, &link_str);
        } else {
            current_value.push_str(&link_str);
        }
        let new_md = Markdown::from(AttrValue::from(current_value), key.clone());
        markdown_ctx.update_markdown(new_md);
        let modal: HtmlInputElement = document().get_element_by_id(&modal_id).unwrap().dyn_into().unwrap();
        modal.set_checked(false);
    });

    let modal_id = AttrValue::from("add_link_modal");
    html! {
        <>
            <input type="checkbox" id={&modal_id} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Insert Link"}</h3>
                    <div class="flex flex-col space-y-4 mt-4">
                        <p>{"Insert a link into your markdown using the textboxes below."}</p>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Link Title"}</span>
                            </label>
                            <input id="link_title_input" type="text" placeholder="A Super Duper Cool Link"
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Link"}</span>
                            </label>
                            <input id="link_input" type="text" placeholder="https://www.youtube.com/watch?v=Hc_td_w7dvc"
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                    </div>

                    <div class="modal-action">
                        <label for={&modal_id} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={insert_link} class="btn">{"Insert"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}

#[function_component(AddImageModal)]
pub fn add_link_modal() -> Html {
    let markdown_ctx = use_markdown();
    let key = use_markdown().state().key;

    let modal_id = AttrValue::from("add_image_modal");
    let insert_link = Callback::from(move |_| {
        let link_title_input: HtmlInputElement = document().get_element_by_id("image_link_title_input").unwrap().dyn_into().unwrap();
        let link_title = link_title_input.value();

        let link_input: HtmlInputElement = document().get_element_by_id("image_link_input").unwrap().dyn_into().unwrap();
        let link = link_input.value();

        let link_str = format!("![{}]({})", link_title, link);

        let editor: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = editor.value();

        if let Some(end) = editor.selection_end().unwrap() {
            let end_usize = end as usize;
            current_value.insert_str(end_usize, &link_str);
        } else {
            current_value.push_str(&link_str);
        }
        let new_md = Markdown::from(AttrValue::from(current_value), key.clone());
        markdown_ctx.update_markdown(new_md);
        let modal: HtmlInputElement = document().get_element_by_id(&modal_id).unwrap().dyn_into().unwrap();
        modal.set_checked(false);
    });

    let modal_id = AttrValue::from("add_image_modal");
    html! {
        <>
            <input type="checkbox" id={&modal_id} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h3 class="font-bold text-2xl">{"Insert Link"}</h3>
                    <div class="flex flex-col space-y-4 mt-4">
                        <p>{"Insert a link into your markdown using the textboxes below."}</p>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Image Title"}</span>
                            </label>
                            <input id="link_title_input" type="text" placeholder="A Super Duper Cool Link"
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                        <div class="form-control w-full max-w-xs">
                            <label class="label">
                                <span class="label-text">{"Image Link"}</span>
                            </label>
                            <input id="link_input" type="text" placeholder="https://i.ytimg.com/vi/gvm-QXS3yjY/maxresdefault.jpg"
                                class="input input-bordered w-full max-w-xs" />
                        </div>

                    </div>

                    <div class="modal-action">
                        <label for={&modal_id} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={insert_link} class="btn">{"Insert"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}