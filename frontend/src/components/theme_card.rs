use yew::prelude::*;

use crate::contexts::theme::{use_theme, ThemeContext};

#[derive(Properties, PartialEq)]
pub struct ThemeCardProps {
    pub name: AttrValue,
}

#[function_component(ThemeCard)]
pub fn theme_card_button(props: &ThemeCardProps) -> Html {
    let theme_context: ThemeContext = use_theme();
    let card_theme = props.name.to_string();

    let switch_theme = Callback::from(move |_| theme_context.set(card_theme.clone()));

    html! {
        <div onclick={switch_theme}
            class="w-48 border-base-content/20 hover:border-base-content/40 outline-base-content overflow-hidden rounded-lg border outline-2 outline-offset-2">
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
    let theme_context: ThemeContext = use_theme();
    let card_theme = props.name.to_string();

    let switch_theme = Callback::from(move |_| theme_context.set(card_theme.clone()));

    html! {
        <li>
            <a onclick={switch_theme}>{&props.name}</a>
        </li>
    }
}