use yew::prelude::*;
use crate::pages::Page;
use crate::use_navigator;

#[derive(Properties, PartialEq)]
pub struct WelcomeHeroProps {}

#[function_component(WelcomeHero)]
pub fn welcome_hero(props: &WelcomeHeroProps) -> Html {
    let navigator = use_navigator().unwrap();
    
    let open_home = Callback::from(move |_| {
        navigator.push(&Page::Home);
    });
    
    html! {
        <div class="hero min-h-screen bg-base-100">
            <div class="hero-content text-center">
                <div class="max-w-xl">
                    <h1 class="text-6xl font-bold font-display">{"Ubiquity"}</h1>
                    <p class="text-xl py-6 font-mono">{"A markdown editor for Windows, Mac &
                        Linux."}
                    </p>
                    <button onclick={open_home} class="btn btn-primary btn-outline">{"Get Started"}</button>
                </div>
            </div>
        </div>
    }
}
