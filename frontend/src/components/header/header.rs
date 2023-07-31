use yew::prelude::*;

use crate::{components::header::{mobile::MobileHeader, desktop::DesktopHeader}, contexts::config::use_config};

#[function_component(Header)]
pub fn header() -> Html {
    let mobile_ui = use_config().is_mobile_ui();

    html! {
        if mobile_ui {
            <MobileHeader />
        } else {
            <DesktopHeader />
        }
    }
}