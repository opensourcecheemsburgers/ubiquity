use yew::prelude::*;
use markdown::{self, Options, ParseOptions, CompileOptions};
use crate::contexts::markdown::use_markdown;

#[function_component(Pdf)]
pub fn pdf() -> Html {
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
    
    let classes = classes!(
        "prose",
        "prose-img:rounded-xl",
        "prose-pre:bg-base-300",
        "prose-pre:text-base-content",
        "prose-pre:overflow-hidden",
        "prose-code:bg-base-300",
        "prose-code:px-[5.5px]",
        "prose-code:font-normal",
        "prose-code:rounded-[0.3125rem]",
        "prose-code:overflow-hidden",
        "prose-a:no-underline",
        "prose-a:text-info",
        "print:block",
        "hidden",
        "bg-base-100"
    );
    
    html! {
        <article data-theme={"light"} id="preview" class={classes}>
            { md_html }
        </article>
    }
}