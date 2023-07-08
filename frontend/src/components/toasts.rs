use error::UbiquityError;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::{HtmlDivElement, Node};
use yew::prelude::*;
use gloo_timers::future::TimeoutFuture;
use wasm_bindgen_futures::spawn_local;

#[function_component(Toasts)]
pub fn toasts() -> Html {
    html! {
        <div id="toasts" class="toast toast-end">
        </div>
    }
}

pub struct Toast {
    html: String
}

impl From<UbiquityError> for Toast {
    fn from(error: UbiquityError) -> Self {
        let mut toast_string = format!(r#"<div class="alert alert-error"><h1>{}</h1><p>{}</p>"#, error.title, error.human_description);

        if let Some(description) = error.verbose_description {
            let verbose_description = format!(r#"<p>{}</p>"#, description);
            toast_string.push_str(&verbose_description);
        }

        let closing_div = format!(r#"</div>"#);

        toast_string.push_str(&closing_div);

        Self { html: toast_string }
    }
}

impl Toast {
    pub fn file_saved(path: AttrValue) -> Self {
        let html = format!(r#"<div class="alert alert-success"><h1>File Saved!</h1><p>{}</p></div>"#, path.to_string());
        Self { html }
    }
}

pub fn display_toast(toast: Toast) {
    let toasts: HtmlDivElement = document().get_element_by_id("toasts").unwrap().dyn_into().unwrap();
    let container = document().create_element("div").unwrap();
    container.set_inner_html(&toast.html);
    toasts.append_child(&container);
}

pub fn display_toast_error(error: UbiquityError) {
    let toast = Toast::from(error);
    let toasts: HtmlDivElement = document().get_element_by_id("toasts").unwrap().dyn_into().unwrap();
    let container = document().create_element("div").unwrap();
    container.set_inner_html(&toast.html);
    toasts.append_child(&container);
}