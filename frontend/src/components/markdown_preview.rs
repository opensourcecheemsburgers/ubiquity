use config::View;
use yew::prelude::*;
use markdown::{self, Options};

use crate::{contexts::{markdown::use_markdown, config::use_config}, icons::*};

/// A HTML preview of the user's markdown.
#[function_component(MarkdownPreview)]
pub fn markdown_preview() -> Html {
    let markdown = use_markdown().state();
    let md = markdown::to_html_with_options(&markdown.text, &Options::gfm()).unwrap();
    let md_html = Html::from_html_unchecked(AttrValue::from(md));
    
    let is_single_view = use_config().is_single_view();

    let prose_size = use_config().state().md_preview_font_size;
    let classes = classes!(
        "prose",
        "prose-img:rounded-xl",
        prose_size,
    );

    let config_ctx = use_config();
    let config_ctx_1 = config_ctx.clone();
    let config_ctx_2 = config_ctx_1.clone();
    let increase_prose_size = Callback::from(move |_| {
        config_ctx.increase_preview_font_size().unwrap();
    });
    let decrease_prose_size = Callback::from(move |_| {
        config_ctx_1.decrease_preview_font_size().unwrap();
    });
    let swap_view = Callback::from(move |_| {
        config_ctx_2.set_view(View::Input).unwrap();
    });

    html! {
        <div class="flex flex-col h-full overflow-auto">
            <div class="flex justify-end space-x-1">
                <btn onclick={decrease_prose_size} class="btn btn-sm btn-ghost">
                    <MinusIcon/>
                </btn>
                <btn onclick={increase_prose_size} class="btn btn-sm btn-ghost">
                    <PlusIcon/>
                </btn>
                if is_single_view {
                    <btn onclick={swap_view} class="btn btn-sm btn-ghost">
                        <PreviewEnabledIcon/>
                    </btn>
                }
            </div>
            <div class="overflow-auto">
                <article class={classes}>
                    { md_html }
                </article>
            </div>
        </div>
    }
}