use crate::components::{welcome_hero::WelcomeHero, drawer::Drawer};
use yew::prelude::*;
use crate::pages::background::Background;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <Drawer>
            <Background>
                <WelcomeHero />
            </Background>
        </Drawer>
    }
}
