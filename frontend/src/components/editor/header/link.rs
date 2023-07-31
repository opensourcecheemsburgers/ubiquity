use yew::prelude::*;
use crate::{components::{tooltip::Tooltip, modals::add_link::ADD_LINK_MODAL_ID}, icons::LinkIcon};

use super::header::HeaderBtnProps;

#[function_component(AddLinkBtn)]
pub fn add_link_btn(props: &HeaderBtnProps) -> Html {
    html! {
        <Tooltip tip={"Add link"}>
            <label for={&ADD_LINK_MODAL_ID} class={props.btn_classes}>
                <LinkIcon />
            </label>
        </Tooltip>
    }
}