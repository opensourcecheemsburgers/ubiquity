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
    pub title: AttrValue,
    pub body: AttrValue,
    #[prop_or_default]
    pub btn_txt: AttrValue,
    #[prop_or_default]
    pub modal_type: ModalType,
}

/// A link button with a tooltip that displays the link on hover.
#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    let modal_ctx = use_modals_store();
    
    
    //  "w-11/12", "max-w-5xl"
    let mut modal_box_classes = classes!("modal-box");
    let mut modal_title_classes = classes!("font-bold", "text-lg");
    let mut modal_body_classes = classes!("py-4");
    let mut modal_btn_classes = classes!("btn");
    
    // match props.modal_type {
    //     ModalType::Standard => {},
    //     ModalType::Success => {
    //         modal_box_classes.push("bg-success");
    //         modal_btn_classes.push("btn-success-content");
    //         modal_body_classes.push("text-success-content");
    //         modal_title_classes.push("text-success-content");
    //     },
    //     ModalType::Error=> {
    //         modal_box_classes.push("bg-error");
    //         modal_btn_classes.push("btn-error-content");
    //         modal_body_classes.push("text-error-content");
    //         modal_title_classes.push("text-error-content");
    //     },
    //     ModalType::Info => {
    //         modal_box_classes.push("bg-info");
    //         modal_btn_classes.push("btn-info-content");
    //         modal_body_classes.push("text-info-content");
    //         modal_title_classes.push("text-info-content");
    //     },
    // }
    
    let close_modal: Callback<MouseEvent> = 
        Callback::from(move |_click: MouseEvent| {
            modal_ctx.close_modal();
    });

    html! {
        <div onclick={close_modal.clone()} class="modal modal-open">
            <div onclick={|_| {
                
            }} class={modal_box_classes}>
                <h1 class={modal_title_classes}>{&props.title}</h1>
                <p class={modal_body_classes}>{&props.body}</p>
                <script src="img/lottie.js" />
                <lottie-player class="mx-auto" src="img/404.json" background="transparent" speed="1" style="width: 300px; height: 300px;"
                loop={true} autoplay={true} />
                <div class="modal-action">
                    // <button class={modal_btn_classes.clone()}>{&props.btn_txt}</button>
                    <button onclick={close_modal} class={modal_btn_classes}>{&props.btn_txt}</button>
                </div>
            </div>
        </div>
    }
}
