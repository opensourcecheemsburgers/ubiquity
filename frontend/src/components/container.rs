use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct ContainerProps {
    pub children: Children,
}

#[function_component(Container)]
pub fn container(props: &ContainerProps) -> Html {
    html! {
        <div class="2xl:w-[65%] xl:w-[72.5%] lg:w-[80%] md:w-[87.5%] sm-[95%] w-[98%] h-[calc(100vh-8.25rem)] my-auto border-2 border-base-content rounded-xl p-4 pb-8">
            { props.children.clone() }
        </div>
    }
}

#[function_component(HalfWidthContainer)]
pub fn half_width_container(props: &ContainerProps) -> Html {
    html! {
        <div class="w-[48%] flex-none h-[calc(100vh-8.25rem)] border-2 border-base-content rounded-xl py-6 px-8">
            { props.children.clone() }
        </div>
    }
}