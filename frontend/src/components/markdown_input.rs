use config::View;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::prelude::*;
use web_sys::HtmlTextAreaElement;

use crate::{contexts::{markdown::use_markdown, config::use_config}, icons::*};

/// A HTML textarea element for the user to enter their markdown.
#[function_component(MarkdownInput)]
pub fn markdown_input() -> Html {
    let md_state = use_markdown();
    let md_text = use_markdown().state().text;

    let is_single_view = use_config().is_single_view();

    let node_ref: NodeRef = NodeRef::default();
    let node_ref_clone = node_ref.clone();
    
    // Some browsers do not accept strings inside of a textarea element.
    use_effect(move || {
        let text_area = node_ref_clone.cast::<HtmlTextAreaElement>().unwrap();
        text_area.set_value(md_text.as_str());
    });

    let handle_input = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.clone().dyn_into().unwrap_throw();
        let input_elem: HtmlTextAreaElement = event.target().unwrap_throw().dyn_into().unwrap_throw();
        let text_area_str = input_elem.value();
        let text = AttrValue::from(text_area_str);
        md_state.update_markdown(text);
    });

    let font_size = use_config().state().md_input_font_size;

    let classes = classes!(
        "textarea",
        "whitespace-pre-wrap", 
        font_size,
        "font-mono", 
        "resize-none", 
        "border-none", 
        "outline-none", 
        "focus:outline-none", 
        "w-full",
        "h-full"
    );

    let config_ctx = use_config();
    let config_ctx_1 = config_ctx.clone();
    let config_ctx_2 = config_ctx_1.clone();
    let increase_font_size = Callback::from(move |_| {
        config_ctx.increase_font_size();
    });
    let decrease_font_size = Callback::from(move |_| {
        config_ctx_1.decrease_font_size();
    });
    let swap_view = Callback::from(move |_| {
        config_ctx_2.set_view(View::Preview);
    });
    
    html! {
        <div class="flex flex-col h-full overflow-auto">
            <div class="flex justify-end space-x-1">
                <btn onclick={decrease_font_size} class="btn btn-sm btn-ghost"><MinusIcon/></btn>
                <btn onclick={increase_font_size} class="btn btn-sm btn-ghost"><PlusIcon/></btn>
                if is_single_view {
                    <btn onclick={swap_view} class="btn btn-sm btn-ghost"><PreviewDisabledIcon/></btn>
                }
            </div>
            <textarea
                ref={node_ref}
                id={"md"}
                oninput={handle_input}
                spellcheck={"false"}
                class={classes}>
                // Do not put strings here some browsers won't process them.
            </textarea>
        </div>
    }
}
