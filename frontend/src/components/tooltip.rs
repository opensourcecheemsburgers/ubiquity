use yew::prelude::*;

use crate::contexts::config::use_config;

#[derive(Properties, PartialEq)]
pub struct TooltipProps {
    pub tip: AttrValue,
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub position: TooltipPosition
}

#[derive(PartialEq, Clone, Copy)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right
}

impl Default for TooltipPosition {
    fn default() -> Self {
        TooltipPosition::Bottom
    }
}

#[function_component(Tooltip)]
pub fn tooltip(props: &TooltipProps) -> Html {
    let mut tooltip_classes = classes!("overflow-visible", "tooltip", "tooltip-info", "block");

    match &props.position {
        TooltipPosition::Top => tooltip_classes.push("tooltip-top"),
        TooltipPosition::Bottom => tooltip_classes.push("tooltip-bottom"),
        TooltipPosition::Left => tooltip_classes.push("tooltip-left"),
        TooltipPosition::Right => tooltip_classes.push("tooltip-right"),
    }

    let is_mobile_ui = use_config().is_mobile_ui();

    html! {
        if !is_mobile_ui {
            <div data-tip={&props.tip} class={tooltip_classes} onclick={&props.onclick}>
                { for props.children.clone() }
            </div>
        } else {
            <div>
                { for props.children.clone() }
            </div>
        }
    }
}