use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::{pages::Page};

#[function_component(WelcomeHero)]
pub fn welcome_hero() -> Html {
    let navigator = use_navigator().unwrap();
    
    let open_home = Callback::from(move |_| {
        navigator.push(&Page::Home);
    });
    
    html! {
        <div class="w-screen h-screen flex flex-col items-center justify-center">
            <div class="flex flex-col items-center space-y-14">
                <h1 class="text-6xl md:text-7xl lg:text-8xl xl:text-8xl 2xl:text-8xl 3xl:text-8xl font-bold font-display tracking-wide">{"Ubiquity"}</h1>
                <p class="text-2xl md:text-2xl lg:text-2xl xl:text-4xl 2xl:text-4xl 3xl:text-4xl font-mono">{"A markdown editor."}
                </p>
                <button onclick={open_home} class="btn btn-lg btn-wide btn-primary btn-outline">{"Get Started"}</button>
            </div>
        </div>
    }
}