// use yew::prelude::*;

// use crate::contexts::config::use_config;

// #[derive(Debug, PartialEq, Properties)]
// pub struct DropdownProps {
//     pub children: Children,
// }


// #[function_component(Dropdown)]
// pub fn divider_y_axis(props: &DropdownProps) -> Html {
//     let mut dropdown_classes = classes!("dropdown-content", "z-[1]", "menu", "p-2", "shadow", "bg-base-200", "rounded-box", "w-52", "lg:w-max");
//     if !use_config().state().mobile_ui {
//         dropdown_classes.push("dropdown-hover");
//     }
//     html! {
//         <div class={dropdown_classes}>
//             { props.children.clone() }
//         </div>
//     }
// }