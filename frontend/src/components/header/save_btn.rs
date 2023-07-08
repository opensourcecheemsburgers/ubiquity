use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use crate::contexts::markdown::use_markdown;
use crate::components::tooltip::Tooltip;
use crate::contexts::{markdown::Markdown, config::use_config};
use crate::icons::SaveIcon;
use urlencoding::encode;
use wasm_bindgen::JsCast;
use md::DOCS_KEY;

#[cfg(feature = "web")]
#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    use gloo::utils::document;
    use web_sys::{HtmlInputElement, HtmlAnchorElement};

    let markdown = use_markdown().state();
    let encoded_md = encode(&markdown.text).to_string();

    let mut text_dl = String::from("data:attachment/text,");
    text_dl.push_str(&encoded_md);

    let download_name = use_markdown().state().key;

    let key = use_markdown().state().key;
    let save = Callback::from(move |_| {
        let key = key.clone();
        if let Some(key) = key && !key.eq(&DOCS_KEY) {
            let anchor: HtmlAnchorElement = document().get_element_by_id("dl").unwrap().dyn_into().unwrap();
            anchor.click();
        } else {
            let input: HtmlInputElement = document().get_element_by_id("save-modal").unwrap().dyn_into().unwrap();
            input.set_checked(true);
        }
    });

    let is_mobile_ui = use_config().is_mobile_ui();

    let icon_size = match is_mobile_ui {
        true => AttrValue::from("24"),
        false => AttrValue::from("32"),
    };

    
    html! {
        <Tooltip tip={"Save"}>
            <button onclick={save} class="btn btn-ghost rounded-btn">
                <SaveIcon size={icon_size} />
            </button>
        </Tooltip>
    }
}

#[cfg(not(feature = "web"))]
#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    use crate::{tauri::{save_markdown_to_fs}};

    let md_ctx = use_markdown();
    let save_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        let markdown = clone.state();
        let key = clone.state().key;
        spawn_local(async move {
            let path: String = save_markdown_to_fs(markdown).await.unwrap();
            let key = AttrValue::from(path);
            clone.update_key(key.clone());
        });
    });

    let md_ctx = use_markdown();
    let save_as_fs: Callback<MouseEvent> = Callback::from(move |_| {
        let clone = md_ctx.clone();
        let markdown = clone.state();
        spawn_local(async move {
            let save_as_markdown = Markdown::from(markdown.text.clone(), None);
            let path: String = save_markdown_to_fs(save_as_markdown).await.unwrap();
            let key = Some(AttrValue::from(path));
            let new_md = Markdown::from(markdown.text, key);
            clone.add_markdown(new_md.clone());
            clone.set_markdown(new_md);
        });
    });

    let mut dropdown_classes = classes!("dropdown");

    let is_mobile_ui = use_config().is_mobile_ui();

    if !is_mobile_ui {
        dropdown_classes.push("dropdown-hover");
    }

    let icon_size = match is_mobile_ui {
        true => AttrValue::from("24"),
        false => AttrValue::from("32"),
    };

    html! {
        <div class={dropdown_classes}>
            <label tabindex="0" class="btn btn-ghost">
                <SaveIcon size={icon_size} />
            </label>
            <div class="dropdown-content z-[1] menu p-2 shadow bg-base-100 rounded-box w-52 lg:w-max">
                <ul tabindex="0">
                    <li>
                        <div onclick={save_fs}>
                            {"Save"}
                        </div>
                    </li>
                    <li>
                        <div onclick={save_as_fs}>
                            {"Save As"}
                        </div>
                    </li>
                </ul>
            </div>
        </div>
    }
}