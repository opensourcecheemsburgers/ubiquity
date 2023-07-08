use gloo::{console::debug, utils::document};
use regex::Regex;
use serde::Serialize;
use wasm_bindgen::JsCast;
use yew::prelude::*;
use web_sys::{HtmlTextAreaElement, HtmlDocument};

use crate::{contexts::{markdown::{use_markdown, Markdown}, config::use_config}, icons::*, components::tooltip::Tooltip};

#[function_component(MarkdownInput)]
pub fn markdown_input() -> Html {
    let md_state = use_markdown();
    let md_text = use_markdown().state().text;

    let handle_input = Callback::from(move |input_event: InputEvent| {
        debug!("Input detected");
        let event: Event = input_event.clone().dyn_into().unwrap();
        let editor: HtmlTextAreaElement = event.target().unwrap().dyn_into().unwrap();
        let text_area_str = editor.value();
        let text = AttrValue::from(text_area_str);
        let key = md_state.state().key;
        let md = Markdown::from(text, key);
        md_state.update_markdown(md);
    });

    let node_ref: NodeRef = NodeRef::default();
    let node_ref_clone = node_ref.clone();
    
    // Some browsers do not accept strings inside of a textarea element.
    use_effect(move || {
        let text_area = node_ref_clone.cast::<HtmlTextAreaElement>().unwrap();
        text_area.set_value(md_text.as_str());
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
    let increase_font_size = Callback::from(move |_| {
        config_ctx.increase_font_size();
    });

    let config_ctx = use_config();
    let decrease_font_size = Callback::from(move |_| {
        config_ctx.decrease_font_size();
    });

    let undo = Callback::from(|_| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("undo").unwrap();
    });

    let redo = Callback::from(|_| {
        let html_doc: HtmlDocument = document().dyn_into().unwrap();
        html_doc.exec_command("redo").unwrap();
    });

    let btn_classes = classes!("btn", "xs:btn-sm", "sm:btn-sm", "btn-ghost", "px-1.5");
    
    html! {
        <div class="flex flex-col h-full overflow-visible">
            <div class="flex justify-evenly md:justify-end -space-x-0.2 md:space-x-0 lg:space-x-1">
                <Tooltip tip={"Undo"}>
                    <btn onclick={undo} class={btn_classes.clone()}>
                        <UndoIcon />
                    </btn>
                </Tooltip>
                <Tooltip tip={"Redo"}>
                    <btn onclick={redo} class={btn_classes.clone()}>
                        <RedoIcon />
                    </btn>
                </Tooltip>
                <HeadingsDropdown />
                <BoldBtn />
                <ItalicsBtn />
                <QuoteBtn />

                <Tooltip tip={"Decrease font size"}>
                <btn onclick={decrease_font_size} class={btn_classes.clone()}>
                    <MinusIcon />
                </btn>
                </Tooltip>

                <Tooltip tip={"Increase font size"}>
                <btn onclick={increase_font_size} class={btn_classes.clone()}>
                    <PlusIcon />
                </btn>
                </Tooltip>

            </div>
            <textarea ref={node_ref} id={"editor"} oninput={handle_input}
                spellcheck={"false"} class={classes}>
                // Do not put strings here some browsers won't process them.
            </textarea>
        </div>
    }
}


#[function_component(BoldBtn)]
pub fn bold_btn() -> Html {
    let md_state = use_markdown();
    let bold = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "**");
            current_value.insert_str(end_usize + 2, "**");
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("****");
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let btn_classes = classes!("btn", "xs:btn-sm", "sm:btn-sm", "btn-ghost", "px-1.5");

    html! {
        <Tooltip tip={"Bold"}>
        <btn onclick={bold} class={btn_classes.clone()}><BoldIcon/>
        </btn>
        </Tooltip>
    }
}

#[function_component(ItalicsBtn)]
pub fn italics_btn() -> Html {
    let md_state = use_markdown();
    let italics = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "*");
            current_value.insert_str(end_usize + 1, "*");
            text_area.set_selection_end(Some(start + 1));
        } else {
            current_value.push_str("**");
            text_area.set_selection_end(Some(1));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let btn_classes = classes!("btn", "xs:btn-sm", "sm:btn-sm", "btn-ghost", "px-1.5");

    html! {
        <Tooltip tip={"Italics"}>
            <btn onclick={italics} class={btn_classes.clone()}>
                <ItalicsIcon/>
            </btn>
        </Tooltip>
    }
}

#[function_component(QuoteBtn)]
pub fn quote_btn() -> Html {
    let md_state = use_markdown();
    let quote = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "> ");
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("> ");
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let btn_classes = classes!("btn", "xs:btn-sm", "sm:btn-sm", "btn-ghost", "px-1.5");

    html! {
        <Tooltip tip={"Quote"}>
            <btn onclick={quote} class={btn_classes.clone()}>
                <QuoteIcon />
            </btn>
        </Tooltip>
    }
}

#[function_component(HeadingsDropdown)]
pub fn headings_dropdown() -> Html {
    let md_state = use_markdown();
    let heading_1 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "# ");
            text_area.set_selection_end(Some(start + 2));
        } else {
            current_value.push_str("#");
            text_area.set_selection_end(Some(2));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_2 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "## ");
            text_area.set_selection_end(Some(start + 3));
        } else {
            current_value.push_str("#");
            text_area.set_selection_end(Some(3));
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_3 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "### ");
        } else {
            current_value.push_str("### ");
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_4 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "#### ");
        } else {
            current_value.push_str("#### ");
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_5 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "##### ");
        } else {
            current_value.push_str("##### ");
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let md_state = use_markdown();
    let heading_6 = Callback::from(move |mouse_event: MouseEvent| {
        let text_area: HtmlTextAreaElement = document().get_element_by_id("editor").unwrap().dyn_into().unwrap();
        let mut current_value = text_area.value();

        if let Some(start) = text_area.selection_start().unwrap() &&
        let Some(end) = text_area.selection_end().unwrap() {
            let start_usize = start as usize;
            let end_usize = end as usize;

            current_value.insert_str(start_usize, "###### ");
        } else {
            current_value.push_str("###### ");
        }
        let key = md_state.state().key;
        let md = Markdown::from(AttrValue::from(current_value), key);
        md_state.update_markdown(md);
    });

    let btn_classes = classes!("btn", "xs:btn-sm", "sm:btn-sm", "btn-ghost", "px-1.5");

    html! {
        <div class="dropdown dropdown-hover">
            <label tabindex="0" class={btn_classes}>
                <HeadingIcon /></label>
            <ul tabindex="0"
                class="dropdown-content menu p-2 menu-lg bg-base-200 rounded-box shadow bg-base-100 rounded-box w-max">
                <li onclick={heading_1}>
                    <div class="flex flex-row">
                        <Heading1Icon />
                        {"Heading 1"}
                    </div>
                </li>
                <li onclick={heading_2}>
                    <div class="flex flex-row">
                        <Heading2Icon />
                        {"Heading 2"}
                    </div>
                </li>
                <li onclick={heading_3}>
                    <div class="flex flex-row">
                        <Heading3Icon />
                        {"Heading 3"}
                    </div>
                </li>
                <li onclick={heading_4}>
                    <div class="flex flex-row">
                        <Heading4Icon />
                        {"Heading 4"}
                    </div>
                </li>
                <li onclick={heading_5}>
                    <div class="flex flex-row">
                        <Heading5Icon />
                        {"Heading 5"}
                    </div>
                </li>
                <li onclick={heading_6}>
                    <div class="flex flex-row">
                        <Heading6Icon />
                        {"Heading 6"}
                    </div>
                </li>
            </ul>
        </div>
    }
}

#[derive(Serialize)]
pub struct FileArgs {
    file_name: String,
    file_contents: String
}