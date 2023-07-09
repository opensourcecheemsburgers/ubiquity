use yew::prelude::*;
use crate::components::editor::header::header::HEADER_BTN_CLASSES;
use crate::contexts::config::use_config;
use crate::components::tooltip::Tooltip;
use crate::icons::PlusIcon;

#[function_component(FontIncreaseBtn)]
pub fn font_increase_btn() -> Html {
    let config_ctx = use_config();
    let increase_font_size = Callback::from(move |_| {
        config_ctx.increase_font_size();
    });

    html! {
        <Tooltip tip={"Increase font size"}>
            <btn onclick={increase_font_size} class={HEADER_BTN_CLASSES}>
                <PlusIcon />
            </btn>
        </Tooltip>
    }
}