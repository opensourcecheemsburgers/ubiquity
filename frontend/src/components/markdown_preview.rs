use yew::prelude::*;
use markdown::{self, Options, ParseOptions, CompileOptions};

use crate::{contexts::{markdown::use_markdown, config::use_config, toasts::{use_toaster, err_modal}}, icons::*, components::tooltip::Tooltip};

/// A HTML preview of the user's markdown.
#[function_component(MarkdownPreview)]
pub fn markdown_preview() -> Html {
    let markdown = use_markdown().state();

    let compile = CompileOptions {
        allow_dangerous_html: true,
        allow_dangerous_protocol: true,
        ..CompileOptions::default()
    };
    let parse = ParseOptions::gfm();
    let options = &Options { compile, parse };

    let md = markdown::to_html_with_options(&markdown.text, options).unwrap();
    let md_html = Html::from_html_unchecked(AttrValue::from(md));

    let prose_size = use_config().state().md_preview_font_size;
    let classes = classes!(
        "prose",
        "prose-img:rounded-xl",
        "prose-pre:bg-base-300",
        "prose-pre:text-base-content",
        "prose-pre:overflow-auto",
        "prose-code:bg-base-300",
        "prose-code:px-[5.5px]",
        "prose-code:font-normal",
        "prose-code:rounded-[0.3125rem]",
        "prose-code:overflow-auto",
        "prose-a:no-underline",
        "prose-a:text-info",
        prose_size,
        "print:block"
    );

    let btn_classes = classes!(
        "btn",
        "btn-sm",
        "btn-ghost"
    );

    let config_ctx = use_config();
    let toaster_ctx = use_toaster();
    let increase_prose_size = Callback::from(move |_| {
        let toaster_ctx = toaster_ctx.clone();
        config_ctx.increase_preview_font_size().unwrap_or_else(|err| err_modal(err, toaster_ctx));
    });

    let config_ctx = use_config();
    let toaster_ctx = use_toaster();
    let decrease_prose_size = Callback::from(move |_| {
        let toaster_ctx = toaster_ctx.clone();
        config_ctx.decrease_preview_font_size().unwrap_or_else(|err| err_modal(err, toaster_ctx));
    });

    html! {
        <div class="flex flex-col h-full overflow-visible scroll-smooth">
            <div class="flex justify-end">
                <Tooltip tip={"Decrease preview size"}>
                <btn onclick={decrease_prose_size} class={btn_classes.clone()}><FontDecreaseIcon/></btn>
                </Tooltip>
                <Tooltip tip={"Increase preview size"}>
                    <btn onclick={increase_prose_size} class={btn_classes.clone()}><FontIncreaseIcon/></btn>
                </Tooltip>
            </div>
            <div class="overflow-auto">
                <article id="preview" class={classes}>
                    { md_html }
                </article>
            </div>
        </div>
    }
}