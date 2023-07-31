use error::UbiquityError;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlDivElement;
use yew::prelude::*;
use std::ops::Deref;

use crate::components::toasts::{TOASTER_ID, ToastProps};

#[derive(Clone, Debug, PartialEq)]
pub struct Toast {
    pub html: Html,
}

impl From<UbiquityError> for Toast {
    fn from(error: UbiquityError) -> Self {
        let toast_id= error.title.clone();

        let close = Callback::from(move |_| {
            let toaster: HtmlDivElement = document().get_element_by_id(TOASTER_ID).unwrap().dyn_into().unwrap();
            let toast: HtmlDivElement = document().get_element_by_id(&toast_id).unwrap().dyn_into().unwrap();
            toaster.remove_child(&toast).unwrap();
        });

        let toast_id = error.title.clone();
        let html = match error.verbose_description {
            Some(description) => html! {
                <div id={toast_id} class="toast">
                    <div class="alert alert-info flex flex-col items-start">
                        <span class="font-bold text-xl">{error.title}</span>
                        <span>{error.human_description}</span>
                        <div class="collapse">
                            <input type="checkbox" />
                            <div class="collapse-title p-0 m-0">
                                <button onclick={close} class="btn btn-ghost">{"Close"}</button>
                                <button class="btn btn-error">{"Full Error"}</button>
                            </div>
                            <div class="collapse-content">
                                <p>
                                    {description}
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            },
            None => html! {
                <div id={toast_id} class="toast">
                    <div class="alert alert-info flex flex-col items-start">
                        <span class="font-bold text-xl">{error.title}</span>
                        <span>{error.human_description}</span>
                        <button onclick={close} class="btn btn-ghost">{"Close"}</button>
                    </div>
                </div>
            },
        };
        Toast { html }
    }
}




// pub fn add_toast(toast: Toast)  {
//     let toaster: HtmlDivElement = document().get_element_by_id(TOASTER_ID).unwrap().dyn_into().unwrap();
//     let toast: HtmlDivElement = document().get_element_by_id(&toast_id).unwrap().dyn_into().unwrap();
//     toaster.remove_child(&toast);
// }

// pub fn remove_toast(toast: Toast)  {
//     let toaster: HtmlDivElement = document().get_element_by_id(TOASTER_ID).unwrap().dyn_into().unwrap();
//     let toast: HtmlDivElement = document().get_element_by_id(&toast_id).unwrap().dyn_into().unwrap();
//     toaster.remove_child(&toast);
// }


#[derive(Default, PartialEq, Clone)]
pub struct Toaster {
    pub toasts: Vec<ToastProps>
}

impl Toaster {
    pub fn current(&self) -> &Self {
        &self
    }

    pub fn from(toasts: Vec<ToastProps>) -> Self {
        Self { toasts }
    }
}

#[derive(Clone)]
pub struct ToasterContext {
   inner: UseStateHandle<Toaster>,
}

impl ToasterContext {
    pub fn new(inner: UseStateHandle<Toaster>) -> Self {
        Self { inner }
    }

    pub fn add_toast(&self, toast: ToastProps) {
        let mut new_toast: Vec<ToastProps> = Vec::new();
        let mut mouldy_toast: Vec<ToastProps> = self.toasts.clone();
        new_toast.append(&mut mouldy_toast);
        new_toast.push(toast);
        self.inner.set(Toaster::from(new_toast))
    }

    pub fn remove_toast(&self, toast: ToastProps) {
        let old_toaster = self.toasts.clone();
        let mut new_toaster = old_toaster.clone();
        let toast = new_toaster.iter().position(|popped_toast| popped_toast.to_owned() == toast).unwrap();
        new_toaster.remove(toast);
        self.inner.set(Toaster::from(new_toaster));
    }

    pub fn state(&self) -> Toaster {
        self.inner.current().clone()
    }
}

impl Deref for ToasterContext {
    type Target = Toaster;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for ToasterContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ToasterProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn ToasterProvider(props: &ToasterProviderProps) -> Html {
    let toaster = Toaster::default();
    let toaster_state = use_state(|| toaster);
    let toaster_context = ToasterContext::new(toaster_state);

    html! {
        <ContextProvider<ToasterContext> context={toaster_context}>
            {props.children.clone()}
        </ContextProvider<ToasterContext>>
    }
}

#[hook]
pub(crate) fn use_toaster() -> ToasterContext {
    use_context::<ToasterContext>().unwrap()
}

pub fn err_modal(err: UbiquityError, toaster_ctx: ToasterContext) {
    gloo::console::debug!("Err Title: {}/nErr description", &err.title, &err.human_description);
    let toast_props = ToastProps::from(err);
    toaster_ctx.add_toast(toast_props);
}