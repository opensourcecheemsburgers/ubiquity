use yew::prelude::*;

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
        <div data-tip={tip} class="tooltip tooltip-bottom tooltip-info">
            <a role="button" class="btn btn-outline btn-lg" href={props.link.clone()}
                target="_blank">{props.title.clone()}
            </a>
        </div>
    }
}
