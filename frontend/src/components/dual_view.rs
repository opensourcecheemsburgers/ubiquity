use yew::prelude::*;
use crate::components::container::HalfWidthContainer;
use crate::components::editor::editor::Editor;
use crate::components::markdown_preview::MarkdownPreview;

#[function_component(DualView)]
pub fn dual_view() -> Html {
    let dual_view_classes = classes!(
        "w-[calc(100vw-2.5rem)]", 
        "flex",
        "flex-1", 
        "flex-row", 
        "justify-center", 
        "space-x-8", 
        "items-center", 
        "h-[calc(100vh-8.5rem)]"
    );

    html! {
        <div class={dual_view_classes}>
            <HalfWidthContainer>
                <Editor />
            </HalfWidthContainer>
            <HalfWidthContainer>
                <MarkdownPreview />
            </HalfWidthContainer>
        </div>
    }
}