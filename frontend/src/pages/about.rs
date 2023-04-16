use yew::prelude::*;
use crate::{contexts::theme::use_theme, components::{header::Header, link_btn::LinkBtn}};
use crate::contexts::modals::use_modals_store;

#[function_component(About)]
pub fn about() -> Html {
    let theme = use_theme().state();
    
    let modals_html = use_modals_store().modals_html();

    let source_code_link = "https://github.com/opensourcecheemsburgers/Ubiquity";
    let tauri_link = "https://tauri.app";
    let yew_link = "https://yew.rs";
    let daisyui_link = "https://daisyui.com";
    
    html! {
        <div data-theme={theme} class="select-none">
            <Header />
            <div class="hero min-h-[calc(100vh-64px)] bg-base-200">
                <div class="hero-content">
                    <article
                        class="prose-xl prose-a:text-primary hover:prose-a:text-primary-focus font-mono cursor-default">
                        <h1 class="font-display">{"Ubiquity"}</h1>
                        <p class="text-2xl">
                            {"An open-source, cross-platform markdown editor made with Tauri."}
                        </p>
                        <p class="text-2xl">
                            {"Ubiquity is written in Rust and its frontend is built with Yew and DaisyUI."}
                        </p>
                        <h2 class="font-display">{"Links"}</h2>
                        <div class="not-prose flex justify-start space-x-5">
                            <LinkBtn title={"Source Code"} link={source_code_link} />
                            <LinkBtn title={"Tauri"} link={tauri_link} />
                            <LinkBtn title={"Yew"} link={yew_link} />
                            <LinkBtn title={"Daisy UI"} link={daisyui_link} />
                        </div>
                    </article>
                </div>
            </div>
            { modals_html }
        </div>
    }
}
