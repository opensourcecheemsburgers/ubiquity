use crate::components::drawer::Drawer;
use crate::components::dual_view::DualView;
use crate::components::footer::Footer;
use crate::components::header::header::Header;
use crate::components::modals::modals::Modals;
use crate::components::single_view::SingleView;
use crate::components::toasts::Toaster;
use crate::contexts::config::use_config;
use crate::pages::background::Background;
use crate::components::pdf::Pdf;
use yew::prelude::*;


#[function_component(Home)]
pub fn home() -> Html {
    let is_mobile_ui = use_config().is_mobile_ui();
    html! {
        if is_mobile_ui {
            <>
                <Drawer>
                    <Background>
                            <Header />
                            <div class="flex flex-col content-center align-center items-center justify-center">
                                <SingleView />
                                <Footer />
                            </div>
                            <Modals />
                            <Toaster />
                    </Background>
                </Drawer>
                <Pdf />
            </>
        } else {
            <>
            <Background>
                <Header />
                <div class="h-[calc(100vh-4rem)] flex flex-col content-center align-center items-center justify-center">
                    <DualView />
                </div>
                <Modals />
                <Toaster />
            </Background>
            <Pdf />
            </>
        }
    }
}
