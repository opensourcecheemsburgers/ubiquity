use config::View;
use yew::prelude::*;
use crate::components::drawer::Drawer;
use crate::components::{theme_card::ThemeCard, header::header::Header};
use crate::contexts::config::{use_config, THEMES};
use crate::pages::background::Background;

#[function_component(Settings)]
pub fn settings() -> Html {
    html! {
        <Drawer>
            <Background>
                <Header />
                <SettingsPage>
                    <SettingsContainer>
                        <ThemeSettings />
                        <LayoutSettings />
                    </SettingsContainer>
                </SettingsPage>
            </Background>
        </Drawer>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsPageProps {
    pub children: Children,
}

#[function_component(SettingsPage)]
pub fn settings_page(props: &SettingsPageProps) -> Html {
    let page_classes = classes!(
        "flex",
        "justify-center",
        "bg-base-100"
    );

    let mut classes = classes!("flex", "flex-col");

    match use_config().is_mobile_ui() {
        true => classes.push("h-[calc(100svh)]"),
        false => classes.push("h-[calc(100vh-4rem)]")
    }

    html! {
        <div class={page_classes}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsContainerProps {
    pub children: Children,
}

#[function_component(SettingsContainer)]
pub fn settings_container(props: &SettingsContainerProps) -> Html {
    let container_classes = classes!(
        "flex", 
        "flex-col",         
        "w-[97.5vw]",
        "2xl:w-[50vw]",
        "xl:w-[50vw]",
        "lg:w-[85vw]", 
        "md:w-[90vw]",
        "sm:w-[95vw]", 
        "my-[3vh]",
        "px-6",
        "overscroll-contain",
        "overflow-visible",
        "overflow-y-auto",
        "gap-16"
    );

    html! {
        <div class={container_classes}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, PartialEq, Properties)]
pub struct SettingsHeaderProps {
    pub text: AttrValue
}

#[function_component(SettingsHeader)]
pub fn settings_header(props: &SettingsHeaderProps) -> Html {
    let header_classes = classes!("font-display", "text-3xl");

    html! {
        <h1 class={header_classes}>
            { &props.text }
        </h1>
    }
}

#[function_component(ThemeSettings)]
pub fn theme_settings() -> Html {
    let mut theme_btns_html: Vec<Html> = Vec::new(); 
    
    for theme in THEMES {
        let att = AttrValue::from(theme.clone());
        let html = html!{ <ThemeCard name={att} /> };
        theme_btns_html.push(html);
    };

    html! {
        <div class="flex flex-col gap-3">
            <SettingsHeader text={"Theme"}/>
            <div class="flex flex-wrap flex-row gap-4">
                { for theme_btns_html }
            </div>
        </div>
    }
}

#[function_component(LayoutSettings)]
pub fn layout_settings() -> Html {
    let config_context = use_config();
    let config = config_context.state();
    let view = config.view;
    let is_single_view = view != View::Dual;
    let mobile_ui = config.mobile_ui;

    // let toggle_view = Callback::from(move |_| {
    //     let _ = config_context.toggle_view();
    // });

    let toggle_mobile_ui = Callback::from(move |_| {
        let _ = config_context.toggle_mobile_ui();
    });

    let classes = classes!("flex", "flex-col");

    html! {
        <div class={classes}>
            <SettingsHeader text={"Layout"} />
            <div class="divider" />
            <div class="form-control w-full">
                <label class="cursor-pointer label">
                    <span class="font-mono text-2xl">{"Mobile UI"}</span>
                    <input type="checkbox" class="toggle toggle-primary" checked={mobile_ui}
                        onclick={toggle_mobile_ui} />
                </label>
                <div class="divider" />
            </div>
        </div>

    }
}