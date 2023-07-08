use crate::components::drawer::Drawer;
use crate::components::dual_view::DualView;
use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::modals::Modals;
use crate::components::single_view::SingleView;
use crate::contexts::config::use_config;
use crate::pages::background::Background;
use yew::prelude::*;



#[function_component(Home)]
pub fn home() -> Html {
    let is_mobile_ui = use_config().is_mobile_ui();
    html! {
        if is_mobile_ui {
            <Drawer>
                <div class="min-h-[calc(100svh)]">
                    <Header />
                    <div class="flex flex-col content-center align-center items-center justify-center">
                        <SingleView />
                        <Footer />
                    </div>
                    <Modals />
                </div>
            </Drawer>
        } else {
            <Background>
                <Header />
                <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                    <DualView />
                </div>
                <Modals />
            </Background>
        }
    }
}
