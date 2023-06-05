use crate::components::dual_view::DualView;
use crate::components::header::Header;
use crate::components::single_view::SingleView;
use crate::contexts::config::use_config;
use crate::pages::background::Background;
use crate::contexts::modals::use_modals_store;
use yew::prelude::*;



#[function_component(Home)]
pub fn home() -> Html {
    let modals_html = use_modals_store().modals_html();
    let is_single_view = use_config().is_single_view();
    
    html! {
        <Background>
                <Header />
                <div class="h-[calc(100vh-4rem)] flex items-center justify-center">
                    if is_single_view {
                        <SingleView />
                    } else {
                        <DualView />
                    }
                </div>
            { modals_html }
        </Background>
    }
}
