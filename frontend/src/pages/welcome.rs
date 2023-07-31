use crate::components::{welcome_hero::WelcomeHero, drawer::Drawer};
use yew::prelude::*;
use crate::pages::background::Background;
use crate::components::modals::modals::Modals;
use crate::components::toasts::Toaster;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    html! {
        <Drawer>
            <Background>
                <WelcomeHero />
            </Background>
            <Modals />
            <Toaster />
        </Drawer>
    }
}
