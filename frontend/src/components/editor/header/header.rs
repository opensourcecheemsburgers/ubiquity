use yew::prelude::*;

use crate::components::editor::header::bold::BoldBtn;
use crate::components::editor::header::formatting::FormattingDropdown;
use crate::components::editor::header::headings::HeadingsDropdown;
use crate::components::editor::header::italics::ItalicsBtn;
use crate::components::editor::header::add::AddDropdown;
use crate::components::editor::header::quote::QuoteBtn;
use crate::components::editor::header::redo::RedoBtn;
use crate::components::editor::header::undo::UndoBtn;
use crate::components::editor::header::font_decrease::FontDecreaseBtn;
use crate::components::editor::header::font_increase::FontIncreaseBtn;
use crate::components::editor::header::image::AddImageBtn;
use crate::components::editor::header::table::AddTableBtn;
use crate::components::editor::header::link::AddLinkBtn;
use crate::components::divider::DividerYAxis;
use crate::contexts::config::use_config;

pub const MOBILE_HEADER_BTN_CLASSES: &'static str = "btn btn-xs btn-ghost";
pub const DESKTOP_HEADER_BTN_CLASSES: &'static str = "btn btn-ghost btn-xs 2xl:btn-sm";

#[derive(Debug, PartialEq, Properties)]
pub struct HeaderBtnProps {
    pub btn_classes: &'static str
}

#[function_component(EditorHeader)]
pub fn input_header() -> Html {
    let is_mobile_ui = use_config().is_mobile_ui();
    
    let btn_classes = match is_mobile_ui {
        true => MOBILE_HEADER_BTN_CLASSES,
        false => DESKTOP_HEADER_BTN_CLASSES
    };

    html! {
        if is_mobile_ui {
            <div class="flex justify-end -space-x-0.2 lg:space-x-1">
                <UndoBtn btn_classes={btn_classes} />
                <RedoBtn btn_classes={btn_classes}/>
                <HeadingsDropdown btn_classes={btn_classes}/>
                <FormattingDropdown btn_classes={btn_classes}/>
                <AddDropdown btn_classes={btn_classes}/>
                <FontDecreaseBtn btn_classes={btn_classes}/>
                <FontIncreaseBtn btn_classes={btn_classes}/>
            </div>
        } else {
            <div class="flex flex-row flex-wrap justify-end w-full">
                <UndoBtn btn_classes={btn_classes}/>
                <RedoBtn btn_classes={btn_classes}/>
                <DividerYAxis />
                <HeadingsDropdown btn_classes={btn_classes}/>
                <BoldBtn btn_classes={btn_classes}/>
                <ItalicsBtn btn_classes={btn_classes}/>
                <QuoteBtn btn_classes={btn_classes}/>
                <DividerYAxis />
                <AddLinkBtn btn_classes={btn_classes}/>
                <AddImageBtn btn_classes={btn_classes}/>
                <AddTableBtn btn_classes={btn_classes}/>
                <DividerYAxis />
                <FontDecreaseBtn btn_classes={btn_classes}/>
                <FontIncreaseBtn btn_classes={btn_classes}/>
            </div>
        }
    }
}