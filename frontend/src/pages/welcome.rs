use crate::components::welcome_hero::WelcomeHero;
use yew::prelude::*;
use crate::pages::background::Background;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <Background>
            <WelcomeHero />
        </Background>
    }
}
