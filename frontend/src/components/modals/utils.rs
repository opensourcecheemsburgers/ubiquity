use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

pub fn close_modal(modal_id: &str) {
    let modal: HtmlInputElement = document().get_element_by_id(&modal_id).unwrap().dyn_into().unwrap();
    modal.set_checked(false);
}

pub fn open_modal(modal_id: &str) {
    let modal: HtmlInputElement = document().get_element_by_id(&modal_id).unwrap().dyn_into().unwrap();
    modal.set_checked(true);
}