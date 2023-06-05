use config::View;
use yew::prelude::*;
use crate::components::container::Container;
use crate::components::markdown_input::MarkdownInput;
use crate::components::markdown_preview::MarkdownPreview;
use crate::contexts::config::use_config;

#[function_component(SingleView)]
pub fn single_view() -> Html {
    let preview = use_config().state().view == View::Preview;
    
    html! {
        <Container>
            if preview {
                <MarkdownPreview />
            } else {
                <MarkdownInput />
            }
        </Container>
    }
}