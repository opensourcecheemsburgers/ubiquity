use error::UbiquityError;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use crate::{pages::Page, components::toasts::ToastProps, contexts::{config::use_config, toasts::use_toaster}};

#[function_component(WelcomeHero)]
pub fn welcome_hero() -> Html {
    let navigator = use_navigator().unwrap();
    
    let open_home = Callback::from(move |_| {
        navigator.push(&Page::Home);
    });

    let mut classes = classes!("w-screen", "flex", "flex-col", "justify-center", "justify-center", "w-screen");
    match use_config().state().mobile_ui {
        true => classes.push("min-h-[calc(100svh)]"),
        false => classes.push("min-h-screen")
    }
    
    html! {
        <div class={classes}>
            <div class="flex flex-col items-center space-y-14">
                <h1
                    class="text-6xl md:text-7xl lg:text-8xl xl:text-8xl 2xl:text-8xl 3xl:text-8xl font-bold font-display tracking-wide">
                    {"Ubiquity"}</h1>
                <p class="text-2xl md:text-2xl lg:text-2xl xl:text-4xl 2xl:text-4xl 3xl:text-4xl font-mono">{"A markdown
                    editor."}
                </p>
                <button onclick={open_home} class="btn btn-lg btn-wide btn-primary btn-outline">{"Get Started"}</button>
            </div>
        </div>
                

        //     <div class="alert alert-info flex flex-col items-start">
        //         <span class="font-bold text-xl">{"Save Error"}</span>
        //         <span>{"There was no save path selected."}</span>
        //         <div tabindex="0" class="collapse collapse-plus">
        //             <div class="collapse-title text-xl font-medium h-8">
        //                 {"View full error"}
        //             </div>
        //             <div class="collapse-content">
        //                 <p>{"eeeeeeeeeeeeeeeeeeeeeeeeeeeee"}</p>
        //             </div>
        //         </div>
        //     </div>
        // </div>
    }
}