use config::View;
use yew::prelude::*;

use crate::{icons::{PreviewDisabledIcon, PreviewEnabledIcon}, contexts::config::use_config};

#[function_component(Footer)]
pub fn footer() -> Html {
    let config = use_config();
    let config_clone = config.clone();

    let swap_to_preview = Callback::from(move |_| {
        config.set_view(View::Preview);
    });

    let swap_to_editor = Callback::from(move |_| {
        config_clone.set_view(View::Input);
    });

    let footer_classes = classes!(
        "flex",
        "flex-row",
        "min-w-screen",
        "py-2",
        "bg-base-300",
    );

    let footer_section_classes = classes!(
        "flex",
        "flex-col",
        "w-[50vw]",
        "items-center",
        "self-center"
    );

    let footer_item_classes = classes!(
        "focus:neutral-focus",
        "flex",
        "flex-col",
        "items-center"
    );
    
    let footer_item_text_classes = classes!(
        "font-sans",
        "text-xs",
        "text-center"
    );

    html! {
        <div class={footer_classes}>
            <div onclick={swap_to_editor} class={footer_section_classes.clone()}>
                <div class={footer_item_classes.clone()}>
                    <PreviewDisabledIcon />
                    <p class={footer_item_text_classes.clone()}>{"Editor"}</p>
                </div>
            </div>
            <div onclick={swap_to_preview} class={footer_section_classes}>
                <div class={footer_item_classes}>
                    <PreviewEnabledIcon />
                    <p class={footer_item_text_classes}>{"Preview"}</p>
                </div>
            </div>
        </div>
    }
}