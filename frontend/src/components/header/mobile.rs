use yew::prelude::*;
use yew_router::prelude::use_navigator;
use urlencoding::encode;

use crate::{
    components::header::{save_btn::SaveBtn, add_dropdown::AddFileDropdown, desktop::DOWNLOAD_ANCHOR_ID},
    icons::{HamburgerIcon, WrenchIcon, RESPONSIVE_ICON_LG},
    Page, contexts::markdown::use_markdown,
};

#[function_component(MobileHeader)]
pub fn mobile_header() -> Html {
    let header_classes = classes!(
        "flex",
        "flex-row",
        "h-[48px]",
        "justify-between",
        "items-center",
        "w-screen",
        "bg-base-100",
    );
    let header_end_classes = classes!("flex", "flex-row", "justify-end", "-space-x-1");
    let header_btn_classes = classes!("btn", "btn-ghost");

    let nav = use_navigator().unwrap();
    let settings_cb = Callback::from(move |_| nav.push(&Page::Settings));

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let download_name = use_markdown().state().key;
    let text_dl = format!("data:attachment/text,{}", encoded_md);

    html! {
        <div class={header_classes}>
            <label for="drawer-input" class={header_btn_classes.clone()}>
                <HamburgerIcon classes={RESPONSIVE_ICON_LG}/>
            </label>
            <div class={header_end_classes}>
                <AddFileDropdown />
                <SaveBtn />
                <btn onclick={settings_cb} class={header_btn_classes}>
                    <WrenchIcon classes={RESPONSIVE_ICON_LG} />
                </btn>
            </div>
            <a id={DOWNLOAD_ANCHOR_ID} class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </div>
    }
}