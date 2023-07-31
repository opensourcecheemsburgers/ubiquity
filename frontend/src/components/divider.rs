use yew::prelude::*;

#[function_component(DividerYAxis)]
pub fn divider_y_axis() -> Html {
    html! {
        <div class="divider divider-horizontal mx-1"/>
    }
}

#[function_component(DividerXAxis)]
pub fn divider_x_axis() -> Html {
    html! {
        <div class="divider divider-vertical"/>
    }
}