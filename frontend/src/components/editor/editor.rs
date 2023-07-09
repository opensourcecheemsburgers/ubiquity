use yew::prelude::*;

use crate::components::editor::{header::header::EditorHeader, textarea::textarea::EditorTextarea};

#[function_component(Editor)]
pub fn editor() -> Html {

    html! {
        <div class="flex flex-col h-full overflow-visible">
            <EditorHeader />
            <EditorTextarea />
        </div>
    }
}