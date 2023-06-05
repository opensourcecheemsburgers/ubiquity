use config::View;
use yew::prelude::*;
use crate::components::tooltip::{TooltipPosition, Tooltip};
use crate::components::{theme_card::ThemeCard, header::Header};
use crate::contexts::config::{use_config, THEMES};
use crate::pages::background::Background;

#[function_component(Settings)]
pub fn settings() -> Html {
    let config_context = use_config();
    let config = config_context.state();
    let view = config.view;
    let is_single_view = view != View::Dual;

    let toggle_view = Callback::from(move |_| {
        let _ = config_context.toggle_view();
    });

    let mut theme_btns_html: Vec<Html> = Vec::new(); 
    
    for theme in THEMES {
        let att = AttrValue::from(theme.clone());
        let html = html!{ <ThemeCard name={att} /> };
        theme_btns_html.push(html);
    };

    html! {
        <Background>
            <Header />
            <div class="flex justify-center h-[calc(100vh-64px)] bg-base-200">
                <div class="flex flex-col
                w-screen
                2xl:w-[50vw] xl:w-[50vw]
                lg:w-[85vw] md:w-[90vw]
                sm:w-[95vw] my-[3vh] px-6
                overscroll-contain overflow-visible overflow-y-auto">
                    <div class="flex flex-col gap-3">
                        <h1 class="font-display text-3xl cursor-default">{"Theme"}</h1>
                        <div class="flex flex-wrap flex-row gap-4">
                            { for theme_btns_html }
                        </div>
                    </div>
                    <div class="mt-16">
                        <h1 class="font-display text-3xl cursor-default">{"Layout"}</h1>
                        <div class="flex flex-col">
                            <div class="divider" />
                            <div class="form-control w-full">
                                <label class="cursor-pointer label">
                                    <Tooltip tip={"Swap between the editor and the preview."} position={TooltipPosition::Right}>
                                        <span class="font-mono text-2xl">{"Single View"}</span>
                                    </Tooltip>
                                    <input type="checkbox" class="toggle toggle-primary" checked={is_single_view} onclick={toggle_view}/>
                                </label>
                            </div>
                            <div class="divider" />
                        </div>
                    </div>           
                </div>
            </div>
        </Background>
    }
}

