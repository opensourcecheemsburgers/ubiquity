use config::View;
use gloo::utils::document;
use wasm_bindgen::JsCast;
use web_sys::HtmlDocument;
use yew::prelude::*;
use markdown::{self, Options, ParseOptions, CompileOptions};

use crate::{contexts::{markdown::use_markdown, config::use_config}, icons::*, components::tooltip::Tooltip};

/// A HTML preview of the user's markdown.
#[function_component(MarkdownPreview)]
pub fn markdown_preview() -> Html {
    let markdown = use_markdown().state();
    let md = markdown::to_html_with_options(&markdown.text, &Options { parse: ParseOptions::gfm(), compile: CompileOptions::gfm() }).unwrap();
    let md_html = Html::from_html_unchecked(AttrValue::from(md));

    let prose_size = use_config().state().md_preview_font_size;
    let classes = classes!(
        "prose",
        "prose-img:rounded-xl",
        prose_size,
    );

    let btn_classes = classes!(
        "btn",
        "btn-sm",
        "btn-ghost"
    );

    let config_ctx = use_config();
    let increase_prose_size = Callback::from(move |_| {
        config_ctx.increase_preview_font_size().unwrap();
    });

    let config_ctx = use_config();
    let decrease_prose_size = Callback::from(move |_| {
        config_ctx.decrease_preview_font_size().unwrap();
    });

    html! {
        <div class="flex flex-col h-full overflow-visible">
            <div class="flex justify-end">
                <Tooltip tip={"Decrease preview size"}>
                <btn onclick={decrease_prose_size} class={btn_classes.clone()}><MinusIcon/></btn>
                </Tooltip>
                <Tooltip tip={"Increase preview size"}>
                <btn onclick={increase_prose_size} class={btn_classes.clone()}><PlusIcon/></btn>
                </Tooltip>
            </div>
            <div class="overflow-auto">
                <article class={classes}>
                    { md_html }
                </article>
            </div>
        </div>
    }
}