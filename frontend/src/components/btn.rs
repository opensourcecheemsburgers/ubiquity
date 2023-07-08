use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct BtnProps {
    pub onclick: Callback<MouseEvent>,
    pub size: BtnSize,
    pub style: BtnStyle,
    pub outline: bool,
    pub children: Children
}

#[derive(Debug, PartialEq)]
pub enum BtnSize {
    VerySmall,
    Small,
    Normal,
    Large
}

#[derive(Debug, PartialEq)]
pub enum BtnShape {
    Normal,
    Wide,
    Block,
    Circle
}

#[derive(Debug, PartialEq)]
pub enum BtnWidth {
    Normal,
    Wide,
    Full
}

#[derive(Debug, PartialEq)]
pub enum BtnStyle {
    Ghost,
    Normal,
    Neutral,
    Primary,
    Secondary,
    Accent,
    Info,
    Success,
    Error,
}



#[derive(Debug, PartialEq)]
pub enum Color {
    Base100,
    Base200,
    Base300,
    Ghost,
    Normal,
    Neutral,
    NeutralFocus,
    NeutralContent,
    Primary,
    PrimaryFocus,
    PrimaryContent,
    Secondary,
    SecondaryFocus,
    SecondaryContent,
    Accent,
    AccentFocus,
    AccentContent,
    Info,
    InfoFocus,
    InfoContent,
    Success,
    SuccessFocus,
    SuccessContent,
    Error,
    ErrorFocus,
    ErrorContent,
}

#[function_component(Btn)]
pub fn btn(props: &BtnProps) -> Html {
    // let btn_size = match props.size {
    //     BtnSize::VerySmall => "btn-xs",
    //     BtnSize::Small =>  "btn-sm",
    //     BtnSize::Normal =>  "",
    //     BtnSize::Large =>  "btn-lg",
    // };

    let btn_style = match props.style {
        BtnStyle::Normal => "",
        BtnStyle::Neutral => "btn-neutral",
        BtnStyle::Primary => "btn-primary-content",
        BtnStyle::Secondary => "btn-secondary",
        BtnStyle::Accent => "btn-accent",
        BtnStyle::Info => "btn-info",
        BtnStyle::Success => "btn-success",
        BtnStyle::Error => "btn-error",
        BtnStyle::Ghost => "btn-ghost",
    };

    let mut container_classes = classes!(
        btn_style,
        "xs:btn-xs",
        "sm:btn-sm",
        "lg:btn-lg",
    );

    if props.outline {
        container_classes.push("btn-outline")
    }

    html! {
        <button onclick={&props.onclick} class={container_classes}>
            { props.children.clone() }
        </button>
    }
}