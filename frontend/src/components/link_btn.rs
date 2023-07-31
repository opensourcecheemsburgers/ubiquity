use yew::prelude::*;

use crate::components::tooltip::Tooltip;

#[derive(Properties, PartialEq)]
pub struct LinkBtnProps {
    pub title: AttrValue,
    pub link: AttrValue,
}

/// A link button with a tooltip that displays the link on hover.
#[function_component(LinkBtn)]
pub fn link_btn(props: &LinkBtnProps) -> Html {
    let tip = props.link.clone().replace("https://", "");

    html! {
        <Tooltip tip={tip}>
            <a role="button" class="btn btn-outline btn-lg normal-case" href={props.link.clone()}
                target="_blank">{props.title.clone()}
            </a>
        </Tooltip>
    }
}
