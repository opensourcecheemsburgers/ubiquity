use yew::prelude::*;
use crate::contexts::markdown::{use_markdown, Markdown};
use web_sys::{HtmlInputElement, HtmlAnchorElement};
use gloo::{utils::document};
use wasm_bindgen::JsCast;

#[function_component(Modals)]
pub fn modals() -> Html {
    html! {
        <>
            <CreateFileModal />
            <SelectNameModal />
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
                    <label for={&modal_id} class="btn">{"Cancel"}</label>
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
                    <label for={&modal_id} class="btn">{"Cancel"}</label>
                    <button onclick={set_name} class="btn">{"Set"}</button>
                </div>
            </div>
        </div>
        </>
    }
}