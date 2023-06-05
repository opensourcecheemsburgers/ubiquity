use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::prelude::use_navigator;
use gloo::file::{Blob, futures::read_as_text};
use urlencoding::encode;

use crate::{
    components::{divider::Divider, theme_card::ThemeDropdownItem, tooltip::Tooltip},
    icons::{AddFileIcon, RedoIcon, UndoIcon, SaveIcon},
    Page, contexts::{markdown::{use_markdown, Markdown}},
};
use wasm_bindgen_futures::spawn_local;

#[function_component(Header)]
pub fn header() -> Html {
    let nav_0 = use_navigator().unwrap();
    let nav_1 = nav_0.clone();
    let nav_2 = nav_1.clone();

    let home_cb= Callback::from(move |_| nav_0.push(&Page::Home));
    let settings_cb = Callback::from(move |_| nav_1.push(&Page::Settings));
    let about_cb = Callback::from(move |_| nav_2.push(&Page::About));

    html! {
        <div class="navbar bg-base-300">
            <div class="navbar-start">
                <SaveBtn />
                <AddFileBtn />
                <Divider />
            </div>

            <div class="navbar-center">
                <btn class="btn btn-ghost normal-case font-display font-normal text-3xl" onclick={home_cb}>
                    {"Ubiquity"}
                </btn>
            </div>

            <div class="navbar-end">
                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                        <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor"
                            stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round">
                            <circle cx="13.5" cy="6.5" r=".5"></circle>
                            <circle cx="17.5" cy="10.5" r=".5"></circle>
                            <circle cx="8.5" cy="7.5" r=".5"></circle>
                            <circle cx="6.5" cy="12.5" r=".5"></circle>
                            <path
                                d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z">
                            </path>
                        </svg>
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
                        <ThemeDropdownItem name={"aqua"} />
                        <ThemeDropdownItem name={"night"} />
                        <ThemeDropdownItem name={"synthwave"} />
                        <ThemeDropdownItem name={"winter"} />
                        <li>
                            <a onclick={settings_cb.clone()}>{"More..."}</a>
                        </li>
                    </ul>
                </div>

                <div class="dropdown dropdown-end">
                    <label tabindex="0" class="btn btn-ghost rounded-btn">
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                            class="inline-block w-5 h-5 stroke-current">
                            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                                d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z" />
                        </svg>
                    </label>
                    <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
                        <li><a onclick={settings_cb}>{"Settings"}</a></li>
                        <li><a onclick={about_cb}>{"About"}</a></li>
                    </ul>
                </div>
            </div>
        </div>
    }
}

#[cfg(not(feature = "desktop"))]
#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    let markdown_ctx = use_markdown();

    let onfileupload = Callback::from(move |e: Event| {
        let markdown_ctx = markdown_ctx.clone();
        let input: HtmlInputElement = e.target_unchecked_into();
        let filelist = input.files().unwrap();
        let file = filelist.get(0).unwrap();
        let key = AttrValue::from(file.name());
        let blob: Blob = file.into();

        spawn_local(async move {
            let file_str = read_as_text(&blob).await.unwrap();
            let text = AttrValue::from(file_str);
            let markdown = Markdown { key, text };
            markdown_ctx.add_markdown(markdown);
        });
    });

    html! {
        <>
        <Tooltip tip={"Import"}>
            <label class="btn btn-ghost rounded-btn" for="md_upload">
                <AddFileIcon />
            </label>
        </Tooltip>
        <input
            id="md_upload"
            type="file"
            accept="text/*"
            multiple={false}
            onchange={onfileupload}
            class="hidden"
        />
        </>
    }
}

#[cfg(feature = "desktop")]
#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let md_ctx = use_markdown();
    let read_from_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        spawn_local(async move {
            clone.read_from_fs().await;
        });
    });

    html! {
        <Tooltip tip={"Import"}>
            <button class="btn btn-ghost rounded-btn" onclick={read_from_fs}>
                <AddFileIcon />
            </button>
        </Tooltip>
    }
}


#[cfg(not(feature = "desktop"))]
#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    
    html! {
        <Tooltip tip={"Save"}>
            <a href={text_dl} download={"test.md"}>
                <button class="btn btn-ghost rounded-btn">
                    <SaveIcon />
                </button>
            </a>
        </Tooltip>
    }
}

#[cfg(feature = "desktop")]
#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    let md_ctx = use_markdown();
    let save_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        spawn_local(async move {
            clone.save_to_fs().await;
        });
    });

    html! {
        <Tooltip tip={"Save"}>
            <button class="btn btn-ghost rounded-btn" onclick={save_fs}>
                <SaveIcon />
            </button>
        </Tooltip>
    }
}