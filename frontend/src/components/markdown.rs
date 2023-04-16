use yew::prelude::*;
use markdown::{self, Options};

use crate::contexts::markdown::use_markdown;

#[derive(Properties, PartialEq)]
pub struct MarkdownProps {}

/// A link button with a tooltip that displays the link on hover.
#[function_component(Markdown)]
pub fn markdown(props: &MarkdownProps) -> Html {
    let markdown_str = use_markdown().state();
    let md = markdown::to_html_with_options(markdown_str.as_str(), &Options::gfm()).unwrap();
    let md_html = Html::from_html_unchecked(AttrValue::from(md));

    html! {
        <article class="prose lg:prose-xl w-full h-[calc(100vh-8.25rem)]">
            { md_html }
        </article>
    }
}
