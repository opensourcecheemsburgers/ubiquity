use std::path::PathBuf;

use error::UbiquityError;
use gloo::console::debug;
use yew::prelude::*;
use crate::contexts::config::use_config;
use crate::contexts::markdown::{use_markdown};
use crate::components::{header::Markdown, tooltip::Tooltip};
use crate::icons::AddFileIcon;
use crate::tauri::read_markdown_from_fs;
use crate::components::toasts::{display_toast, display_toast_error};
use web_sys::{HtmlInputElement, HtmlDivElement, HtmlLabelElement};
use gloo::{utils::document, file::{Blob, futures::read_as_text}};
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use urlencoding::encode;
use serde::{Deserialize, Serialize};
use md::*;


#[function_component(AddFileDropdown)]
pub fn add_file_dropdown() -> Html {
    let markdown_ctx = use_markdown();

    let mut recent_files_html: Vec<Html> = Vec::new();
    let recent_files = Markdown::read_all_markdown_keys();
    recent_files.iter().for_each(|recent_file| {
        let file_name = recent_file.clone();
        let markdown_ctx = markdown_ctx.clone();

        
        let read_file = Callback::from(move |_| {
            if cfg!(feature = "web") {
                let md = Markdown::load_from_storage(file_name.clone());
                markdown_ctx.set_markdown(md);
            } else {
                let markdown_ctx = markdown_ctx.clone();

                let file_name = file_name.clone();
                spawn_local(async move {
                    let key = file_name.clone();
                    let path = file_name.clone().to_string();
                    let read_file: Result<String, UbiquityError> = read_markdown_from_fs(key.clone()).await;
                    
                    match read_file {
                        Ok(file) => {
                            let text = AttrValue::from(file);
                            let key = Some(key);
                            let md = Markdown::from(text, key);
                            markdown_ctx.add_markdown(md.clone());
                            markdown_ctx.set_markdown(md);
                        },
                        Err(error) => {
                            display_toast_error(error);
                        }
                    }
                });
            }
        });

        let file_name = recent_file.clone();
        let html = html! {
            <li>
                <a>
                    <div onclick={read_file}>
                    {file_name}
                    </div>
                </a>
            </li>
        };
        recent_files_html.push(html);
    });

    let is_mobile_ui = use_config().is_mobile_ui();

    let mut dropdown_classes = classes!("dropdown");

    match is_mobile_ui {
        true => dropdown_classes.push("dropdown-end"),
        false => dropdown_classes.push("dropdown-hover"),
    }

    let icon_size = match is_mobile_ui {
        true => AttrValue::from("24"),
        false => AttrValue::from("32"),
    };

    html! {
        <div class={dropdown_classes}>
            <label id="add_file_dropdown" tabindex="0" class="btn btn-ghost">
                <AddFileIcon size={icon_size} />
            </label>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <CreateFileBtn />
                    <AddFileBtn />
                </ul>
                if !recent_files.is_empty() {
                    <ul tabindex="0">
                        <li class="menu-title">{"Recent Files"}</li>
                        {recent_files_html}
                    </ul>
                }
            </div>
        </div>
    }
}

#[cfg(feature = "web")]
#[function_component(CreateFileBtn)]
pub fn create_file_btn() -> Html {
    let markdown_ctx = use_markdown();

    let open_modal = Callback::from(move |_| {
        let input: HtmlInputElement = document().get_element_by_id("create-file").unwrap().dyn_into().unwrap();
        input.set_checked(true);
    });

    html! {
        <li>
            <div onclick={open_modal}>
                {"Create File"}
            </div>
        </li>
    }
}

#[cfg(not(feature = "web"))]
#[function_component(CreateFileBtn)]
pub fn create_file_btn() -> Html {
    use crate::tauri::{self, save_markdown_to_fs, create_new_markdown_file};

    let markdown_ctx = use_markdown();

    let create = Callback::from(move |_| {
        let markdown = markdown_ctx.state();
        let markdown_ctx_clone = markdown_ctx.clone();
        spawn_local(async move {
            let save: Result<String, UbiquityError> = create_new_markdown_file().await;
            match save {
                Ok(key) => {
                    let md = Markdown::from(AttrValue::from(""), Some(AttrValue::from(key)));
                    markdown_ctx_clone.add_markdown(md.clone());
                    markdown_ctx_clone.set_markdown(md.clone());
                },
                Err(error) => { display_toast_error(error) }
            }
        });
    });

    html! {
        <li>
            <div onclick={create}>
                {"Create File"}
            </div>
        </li>
    }
}

#[cfg(feature = "web")]
#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    let markdown_ctx = use_markdown();

    let onfileupload = Callback::from(move |e: Event| {
        let markdown_ctx = markdown_ctx.clone();
        let input: HtmlInputElement = e.target_unchecked_into();
        let filelist = input.files().unwrap();
        let file = filelist.get(0).unwrap();
        let key = Some(AttrValue::from(file.name()));
        let blob: Blob = file.into();

        spawn_local(async move {
            let file_str = read_as_text(&blob).await.unwrap();
            let text = AttrValue::from(file_str);
            let markdown = Markdown { key, text };
            markdown_ctx.add_markdown(markdown);
        });
    });

    html! {
        <li>
            <label for="md_upload">
                {"Import File"}
            </label>
            <input id="md_upload" type="file" accept="text/*" multiple={false} onchange={onfileupload} class="hidden" />
        </li>
    }
}

#[cfg(not(feature = "web"))]
#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    use crate::tauri::{create_new_markdown_file, import_markdown_file};

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let ctx = use_markdown();
    let read_from_fs = Callback::from(move |_| {
        let ctx = ctx.clone();
        spawn_local(async move {
            let create_file: Result<Markdown, UbiquityError> = import_markdown_file().await;
            match create_file {
                Ok(markdown) => {
                    ctx.add_markdown(markdown);
                },
                 Err(error) => {}// display_toast_error(error)
            }
        });
    });

    html! {
        <li>
            <div onclick={read_from_fs}>
                {"Import File"}
            </div>
        </li>
    }
}