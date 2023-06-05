pub mod pages;
pub mod contexts;
pub mod icons;
pub mod components;
pub mod errors;

use contexts::modals::ModalProvider;
use contexts::config::ConfigProvider;

use pages::about::About;
use pages::home::Home;
use pages::settings::Settings;
use pages::welcome::Welcome;

use yew::prelude::*;
use yew_router::prelude::*;

use pages::Page;

use crate::contexts::markdown::MarkdownProvider;

#[function_component(App)]
fn app() -> Html {
    html! {
        <ConfigProvider>
            <ModalProvider>
                    <MarkdownProvider>
                        <BrowserRouter>
                            <Switch<Page> render={move |page| {
                                match page {
                                    Page::Welcome => html!(<Welcome/>),
                                    Page::Home => html!(<Home/>),
                                    Page::About => html!(<About/>),
                                    Page::Settings => html!(<Settings/>),
                                }
                            }} />
                        </BrowserRouter>
                    </MarkdownProvider>
            </ModalProvider>
        </ConfigProvider>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
