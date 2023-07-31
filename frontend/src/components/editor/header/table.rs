use yew::prelude::*;
use crate::components::modals::table::TABLE_MODAL_ID;
use crate::components::tooltip::Tooltip;
use crate::icons::TableIcon;

use super::header::HeaderBtnProps;

#[function_component(AddTableBtn)]
pub fn add_table_btn(props: &HeaderBtnProps) -> Html {
    html! {
        <Tooltip tip={"Create table"}>
            <label for={&TABLE_MODAL_ID} class={props.btn_classes}>
                <TableIcon />
            </label>
        </Tooltip>
    }
}