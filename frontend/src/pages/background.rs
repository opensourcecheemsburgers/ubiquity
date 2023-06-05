use yew::prelude::*;

use crate::contexts::{config::use_config, modals::ModalsHtml};


#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    let theme = use_config().state().theme;
    
    html! {
        <div data-theme={theme} class="h-screen">
            { props.children.clone() }
            <ModalsHtml />
        </div>
    }
}
