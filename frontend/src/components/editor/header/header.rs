use yew::prelude::*;

use crate::components::editor::header::bold::BoldBtn;
use crate::components::editor::header::formatting::FormattingDropdown;
use crate::components::editor::header::headings::HeadingsDropdown;
use crate::components::editor::header::italics::ItalicsBtn;
use crate::components::editor::header::link::AddLinkBtn;
use crate::components::editor::header::quote::QuoteBtn;
use crate::components::editor::header::redo::RedoBtn;
use crate::components::editor::header::undo::UndoBtn;
use crate::components::editor::header::font_decrease::FontDecreaseBtn;
use crate::components::editor::header::font_increase::FontIncreaseBtn;
use crate::contexts::config::use_config;

pub const HEADER_BTN_CLASSES: &'static str = "btn btn-xs lg:btn-sm btn-ghost px-1.0 lg:px-1.5 xl:px-2.0";


#[function_component(EditorHeader)]
pub fn input_header() -> Html {
    let is_mobile_ui = use_config().is_mobile_ui();

    html! {
        <div class="flex justify-evenly md:justify-end -space-x-0.2 lg:space-x-1">
            <UndoBtn />
            <RedoBtn />
            <HeadingsDropdown />
            if is_mobile_ui {
                <FormattingDropdown />
            } else {
                <BoldBtn />
                <ItalicsBtn />
                <QuoteBtn />
            }
            <AddLinkBtn />
            <FontDecreaseBtn />
            <FontIncreaseBtn />
        </div>
    }
}