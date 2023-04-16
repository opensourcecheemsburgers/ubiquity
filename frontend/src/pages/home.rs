use crate::components::header::Header;
use crate::components::markdown::Markdown;
use crate::components::markdown_input::MarkdownInput;
use crate::contexts::modals::use_modals_store;
use crate::contexts::theme::use_theme;
use yew::prelude::*;



#[function_component(Home)]
pub fn home() -> Html {
    let theme = use_theme().state();
    
    let modals_html = use_modals_store().modals_html();
    
    html! {
        <div data-theme={theme} class="h-screen">
                <Header />
                <div class="flex flex-row space-x-16 mt-8 px-8 h-[calc(100vh-8.25rem)]">
                    <div class="w-1/2 h-[calc(100vh-8.25rem)]">
                        <MarkdownInput />
                    </div>
                    <div class="w-1/2 h-[calc(100vh-8.25rem)] overflow-y-auto border-2 rounded-xl p-8">
                        <Markdown />
                    </div>
                </div>
            { modals_html }
        </div>
    }
}
