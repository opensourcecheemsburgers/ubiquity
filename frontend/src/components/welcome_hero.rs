use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::{pages::Page, contexts::modals::use_modals_store, components::modal::ModalProps};

#[function_component(WelcomeHero)]
pub fn welcome_hero() -> Html {
    let navigator = use_navigator().unwrap();

    let modals = use_modals_store();
    
    let open_home = Callback::from(move |_| {
        navigator.push(&Page::Home);
    });
    
    html! {
        <div class="h-screen flex flex-col items-center justify-center">
            <div class="flex flex-col items-center space-y-12">
                <h1 class="text-8xl font-bold font-display tracking-wide">{"Ubiquity"}</h1>
                <p class="text-4xl font-mono">{"A markdown editor."}
                </p>
                <button onclick={open_home} class="btn btn-lg btn-wide btn-primary btn-outline">{"Get Started"}</button>
            </div>
        </div>
    }
}