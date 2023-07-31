use gloo::utils::document;
use log::debug;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use crate::contexts::config::use_config;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    let theme = use_config().state().theme;
    let mobile_ui = use_config().state().mobile_ui;

    let touch_start_x = use_mut_ref(|| 0);
    let touch_start_y = use_mut_ref(|| 0);
    let touch_end_x = use_mut_ref(|| 0);
    let touch_end_y = use_mut_ref(|| 0);

    let touch_start_x_clone = touch_start_x.clone();
    let touch_start_y_clone = touch_start_y.clone();
    let touch_end_x_clone = touch_end_x.clone();
    let touch_end_y_clone = touch_end_y.clone();

    let on_touch_start = Callback::from(move |touch_event: TouchEvent| {
        debug!("On touch");
        let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
        touch_start_x.replace(user_touch.client_x());
        touch_start_y.replace(user_touch.client_y());
    });

    let on_touch_end = Callback::from(move |touch_event: TouchEvent| {
        debug!("End touch");
        let user_touch = touch_event.changed_touches().item(0).unwrap_throw();
        touch_end_x_clone.replace(user_touch.client_x());
        touch_end_y_clone.replace(user_touch.client_y());

        let start_x = *touch_start_x_clone.borrow();
        let end_x = *touch_end_x_clone.borrow();
        let start_y = *touch_start_y_clone.borrow();
        let end_y = *touch_end_y_clone.borrow();

        let drawer: HtmlInputElement = document().get_element_by_id("drawer-input").unwrap().dyn_into().unwrap();
        if end_x > (start_x + 50) && (start_y <= end_y + 50 && start_y >= end_y - 50) {
            drawer.set_checked(true);
        }
    });

    // let mut classes = classes!("flex", "flex-col", "justify-between", "min-h-[calc(100svh)]", "max-h-[calc(100svh)]", "min-w-[calc(100svw)]", "max-w-[calc(100svw)]");
    let mut classes = classes!("flex", "flex-col", "justify-between", "max-w-[calc(100svw)]", "print:hidden");
    
    match mobile_ui {
        true => classes.push("min-h-[calc(100svh)]"),
        false => classes.push("min-h-screen")
    }

    html! {
        <div ontouchstart={on_touch_start} ontouchend={on_touch_end} data-theme={theme} class={classes}>
            { props.children.clone() }
        </div>
    }
}