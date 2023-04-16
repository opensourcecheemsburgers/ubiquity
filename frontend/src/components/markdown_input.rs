use std::{io::Bytes, str::Chars};

use log::{log, debug};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use yew::{prelude::*, html::IntoPropValue};
use web_sys::{HtmlInputElement, HtmlTextAreaElement, Document, Window};

use crate::contexts::markdown::use_markdown;

#[derive(Properties, PartialEq)]
pub struct MarkdownInputProps {}

/// A HTML textarea element for the user to enter their markdown.
#[function_component(MarkdownInput)]
pub fn markdown_input(props: &MarkdownInputProps) -> Html {
    let md = use_markdown();
    let md_text = use_markdown().state();

    let node_ref: NodeRef = NodeRef::default();
    let node_ref_clone = node_ref.clone(); 
    

    // Some browsers do not accept strings inside of a textarea element.
    use_effect(move || {
        let text_area = node_ref_clone.cast::<HtmlTextAreaElement>().unwrap();
        text_area.set_wrap("off");
        
        debug!("txt: {}", md_text);
        
        text_area.set_value(md_text.as_str());
    });

    let handle_input = Callback::from(move |input_event: InputEvent| {
        let event: Event = input_event.dyn_into().unwrap_throw();
        let input_elem: HtmlTextAreaElement = event.target().unwrap_throw().dyn_into().unwrap_throw();

        input_elem.set_wrap("off");
        
        let text_area_str = input_elem.value();
        
        debug!("Str: {}", text_area_str);
        
        // let cp_str = text_area_str.clone();
        
        // let bytes = cp_str.bytes();
        
        // let char = '1';
        
        // char.escape_unicode()
        
        // bytes.enumerate().for_each(|(index, b)| {
            
        // });
        
        
        // bytes.for_each(|b| {
        //     if (b == 10) {
        //         debug!("Byte: {}", b);                
        //     };
        // });
        
        // let chars = text_area_str.chars();
        
        // chars.for_each(|char| {
        //     debug!("Char: {}", char);
        // });
        md.set(text_area_str);
    });
    
    html! {
        <textarea
            ref={node_ref}
            id={"md"}
            wrap={"off"}
            oninput={handle_input} 
            class="textarea text-2xl whitespace-pre-wrap font-mono resize-none w-full h-[calc(100vh-8.25rem)]" 
            placeholder="Write your markdown here. The preview will appear on the right.">
            // Do not put strings here some browsers won't process them.
        </textarea>
    }
}
