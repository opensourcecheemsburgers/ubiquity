use yew::prelude::*;

use crate::contexts::modals::use_modals_store;

#[derive(PartialEq, Clone)]
pub enum ModalType {
    Standard,
    Success,
    Error,
    Info,
}

impl Default for ModalType {
    fn default() -> Self {
        ModalType::Standard   
    }
}

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    // pub children: Children,
    pub title: AttrValue,
    pub body: AttrValue,

    #[prop_or_default]
    pub dual_btn: bool,

    #[prop_or_default]
    pub left_btn_text: AttrValue,
    #[prop_or_default]
    pub right_btn_text: AttrValue,

    #[prop_or_default]
    pub right_btn_on_click: Callback<MouseEvent>,
    #[prop_or_default]
    pub left_btn_on_click: Callback<MouseEvent>,
}

/// A link button with a tooltip that displays the link on hover.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_ctx = use_modals_store();
    
    let close_modal: Callback<MouseEvent> = 
        Callback::from(move |_click: MouseEvent| {
            modal_ctx.close_modal();
        }
    );

    html! {
        <div class="modal modal-open">
            <div class={"modal-box"}>
                <h1 class="font-bold text-lg">{&props.title}</h1>
                <p class="py-4">{&props.body}</p>
                // <script src="img/lottie.js" />
                // <lottie-player class="mx-auto" src="img/404.json" background="transparent" speed="1" style="width: 300px; height: 300px;"
                // loop={true} autoplay={true} />
                <div class="modal-action">
                if props.dual_btn {
                    <button onclick={&props.left_btn_on_click} class="btn">{&props.left_btn_text}</button>
                }
                <button onclick={&props.right_btn_on_click} class="btn">{&props.right_btn_text}</button>
                </div>
            </div>
        </div>
    }
}