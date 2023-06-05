use yew::prelude::*;

use crate::contexts::config::{use_config, ConfigContext};

#[derive(Properties, PartialEq)]
pub struct ThemeCardProps {
    pub name: AttrValue,
}

#[function_component(ThemeCard)]
pub fn theme_card_button(props: &ThemeCardProps) -> Html {
    let config_context: ConfigContext = use_config();
    let card_theme = props.name.to_string();

    let switch_theme = Callback::from(move |_| {
        config_context.set_theme(card_theme.clone());
    });

    let mut card_classes = classes!(
        "w-48",
        "overflow-hidden",
        "rounded-lg",
        "border-2",
    );

    if use_config().state().theme.eq_ignore_ascii_case(&props.name) {
        card_classes.push("border-primary");
        card_classes.push("hover:border-primary");
        card_classes.push("outline-primary");
        card_classes.push("outline-8");
        card_classes.push("outline-offset-8");
        
    } else {
        card_classes.push("border-base-content/20");
        card_classes.push("hover:border-base-content/40");
        card_classes.push("outline-base-content");
        card_classes.push("outline-2");
        card_classes.push("outline-offset-2");
    }

    html! {
        <div onclick={switch_theme}
            class={card_classes}>
            <div data-theme={&props.name} class="bg-base-100 text-base-content cursor-pointer font-sans">
                <div class="grid grid-cols-5 grid-rows-3">
                    <div class="bg-base-200 col-start-1 row-span-2 row-start-1" />
                    <div class="bg-base-300 col-start-1 row-start-3" />
                    <div class="bg-base-100 col-span-4 col-start-2 row-span-3 row-start-1 flex flex-col gap-1 p-2">
                        <div class="font-bold">{&props.name}</div>
                        <div class="flex flex-wrap gap-1">
                            <div class="bg-primary flex aspect-square w-5 items-center justify-center rounded lg:w-6">
                                <div class="text-primary-content text-sm font-bold">{"A"}</div>
                            </div>
                            <div class="bg-secondary flex aspect-square w-5 items-center justify-center rounded lg:w-6">
                                <div class="text-secondary-content text-sm font-bold">{"A"}</div>
                            </div>
                            <div class="bg-accent flex aspect-square w-5 items-center justify-center rounded lg:w-6">
                                <div class="text-accent-content text-sm font-bold">{"A"}</div>
                            </div>
                            <div class="bg-neutral flex aspect-square w-5 items-center justify-center rounded lg:w-6">
                                <div class="text-neutral-content text-sm font-bold">{"A"}</div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[function_component(ThemeDropdownItem)]
pub fn theme_dropdown_item(props: &ThemeCardProps) -> Html {
    let config_context: ConfigContext = use_config();
    let card_theme = props.name.to_string();

    let switch_theme = Callback::from(move |_| {
        config_context.set_theme(card_theme.clone());
    });

    html! {
        <li>
            <a class="capitalize" onclick={switch_theme}>{&props.name}</a>
        </li>
    }
}