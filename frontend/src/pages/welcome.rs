use crate::components::welcome_hero::WelcomeHero;
use crate::contexts::theme::use_theme;
use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let theme = use_theme().state();

    html! {
        <div class="min-h-screen" data-theme={theme}>
            <WelcomeHero />
        </div>
    }
}
