use yew::prelude::*;
use crate::components::modals::add_image::ADD_IMAGE_MODAL_ID;
use crate::components::tooltip::Tooltip;
use crate::icons::AddImageIcon;

use super::header::HeaderBtnProps;

#[function_component(AddImageBtn)]
pub fn add_image_btn(props: &HeaderBtnProps) -> Html {
    html! {
        <Tooltip tip={"Add image"}>
            <label for={&ADD_IMAGE_MODAL_ID} class={props.btn_classes}>
                <AddImageIcon />
            </label>
        </Tooltip>
    }
}
