use std::ops::Deref;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub name: String,
}

impl Theme {
    fn from_str(theme_str: &str) -> Self {
        Self {
            name: theme_str.to_owned(),
        }
    }
    fn current(&self) -> &Self {
        &self
    }
}

impl ImplicitClone for Theme {}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "synthwave".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ThemeContext {
    inner: UseStateHandle<Theme>,
}

impl ThemeContext {
    pub fn new(inner: UseStateHandle<Theme>) -> Self {
        Self { inner }
    }

    pub fn set(&self, name: String) {
        self.inner.set(Theme::from_str(name.as_str()))
    }

    pub fn state(&self) -> String {
        self.inner.name.clone()
    }
}

impl Deref for ThemeContext {
    type Target = Theme;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

impl PartialEq for ThemeContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct ThemeProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn ThemeProvider(props: &ThemeProviderProps) -> Html {
    let theme_name = use_state(|| Theme::default());

    let theme_ctx = ThemeContext::new(theme_name);

    html! {
        <ContextProvider<ThemeContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<ThemeContext>>
    }
}

#[hook]
pub(crate) fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().unwrap()
}

pub const THEMES: &'static [&'static str] = &[
    "light",
    "dark",
    "cupcake",
    "bumblebee",
    "emerald",
    "corporate",
    "synthwave",
    "retro",
    "cyberpunk",
    "valentine",
    "halloween",
    "garden",
    "forest",
    "aqua",
    "lofi",
    "pastel",
    "fantasy",
    "wireframe",
    "black",
    "luxury",
    "dracula",
    "cmyk",
    "autumn",
    "business",
    "acid",
    "lemonade",
    "night",
    "coffee",
    "winter",
];

// impl Reducible for Theme {
//     type Action = String;

//     fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
//         Theme { name: action }.into()
//     }
// }
