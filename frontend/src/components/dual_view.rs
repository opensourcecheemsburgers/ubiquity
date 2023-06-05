use yew::prelude::*;
use crate::components::container::HalfWidthContainer;
use crate::components::markdown_input::MarkdownInput;
use crate::components::markdown_preview::MarkdownPreview;

#[function_component(DualView)]
pub fn dual_view() -> Html {
    html! {
        <div class="w-[calc(100vw-2.5rem)] flex flex-1 flex-row justify-center space-x-8 items-center h-[calc(100vh-8.25rem)]">
            <HalfWidthContainer>
            <MarkdownInput />
            </HalfWidthContainer>
            <HalfWidthContainer>
                <MarkdownPreview />
            </HalfWidthContainer>
        </div>
    }
}