use yew_router::prelude::*;

pub mod about;
pub mod home;
pub mod settings;
pub mod welcome;
pub mod background;

#[derive(Clone, PartialEq, Routable)]
pub enum Page {
    #[at("/")]
    Welcome,
    #[at("/home")]
    Home,
    #[at("/about")]
    About,
    #[at("/settings")]
    Settings,
}