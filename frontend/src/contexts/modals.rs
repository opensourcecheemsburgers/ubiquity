use std::ops::{Deref, DerefMut};

use yew::prelude::*;

use crate::components::modal::{Modal, ModalProps};

#[derive(Clone, PartialEq)]
pub struct ModalStore {
    pub modal_props_vec: Vec<ModalProps>
}

impl ModalStore {
    pub fn new() -> Self {
        Self { modal_props_vec: Vec::new() }
    }
    
    pub fn from_vec(modal_props_vec: Vec<ModalProps>) -> Self {
        Self { modal_props_vec }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct ModalContext {
    modals_state: UseStateHandle<ModalStore>,
}

impl ModalContext {
    pub fn new(modals_state: UseStateHandle<ModalStore>) -> Self {
        Self { modals_state }
    }
    
    pub fn from(modals_state: UseStateHandle<ModalStore>) -> Self {
        Self { modals_state }
    }

    pub fn set(&self, modals: ModalStore) {
        self.modals_state.set(modals)
    }

    pub fn modals(&self) -> Vec<ModalProps> {
        self.modals_state.modal_props_vec.clone()
    }

    pub fn state(&self) -> &ModalStore {
        &self.modals_state
    }
    
    pub fn display_modal(&self, props: ModalProps) {
        let mut new_vec = self.modals();
        new_vec.push(props);
        
        self.modals_state.set(ModalStore::from_vec(new_vec))
    }
    
    pub fn close_modal(&self) {
        let mut new_vec = self.modals();
        new_vec.pop();
        
        self.modals_state.set(ModalStore::from_vec(new_vec))
    }
    
    pub fn clear(&self) {
        self.modals_state.set(ModalStore::new());
    }
    
    pub fn modals_html(&self) -> Html {
        let modals = self.modals();
        
        let mut html_vec: Vec<Html> = Vec::new();
        
        for props in modals {
            let html = html! {
                <Modal ..props />
            };
            html_vec.push(html);
        }
        
        html! {
            <>
                { html_vec }
            </>
        }
    }
}

impl Deref for ModalContext {
    type Target = ModalStore;

    fn deref(&self) -> &Self::Target {
        self.state()
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ModalProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn ModalProvider(props: &ModalProviderProps) -> Html {
    let modals_store: UseStateHandle<ModalStore> = use_state(|| ModalStore::new());
    let modals_store_ctx: ModalContext = ModalContext::new(modals_store);

    html! {
        <ContextProvider<ModalContext> context={modals_store_ctx}>
            {props.children.clone()}
        </ContextProvider<ModalContext>>
    }
}

#[hook]
pub(crate) fn use_modals_store() -> ModalContext {
    use_context::<ModalContext>().unwrap()
}

#[function_component(ModalsHtml)]
pub fn modals_html() -> Html {
    let modal_store = use_context::<ModalContext>().unwrap();
    let html = modal_store.modals_html();
    html! {
        html
    }
}