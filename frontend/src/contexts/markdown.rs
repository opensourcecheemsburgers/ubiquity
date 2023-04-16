use std::ops::Deref;
use yew::html::ImplicitClone;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Markdown {
    pub text: String,
}

impl Markdown {
    fn from_attr_value(text: String) -> Self {
        Self { text }
    }

    fn current(&self) -> &Self {
        &self
    }
}

impl ImplicitClone for Markdown {}

impl Default for Markdown {
    fn default() -> Self {
        Self {
            text: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MarkdownContext {
    inner: UseStateHandle<Markdown>,
}

impl MarkdownContext {
    pub fn new(inner: UseStateHandle<Markdown>) -> Self {
        Self { inner }
    }

    pub fn set(&self, text: String) {
        self.inner.set(Markdown::from_attr_value(text));
    }

    pub fn state(&self) -> String {
        self.inner.text.clone()
    }
}

impl Deref for MarkdownContext {
    type Target = Markdown;

    fn deref(&self) -> &Self::Target {
        self.inner.current()
    }
}

impl PartialEq for MarkdownContext {
    fn eq(&self, rhs: &Self) -> bool {
        *self.inner == *rhs.inner
    }
}

#[derive(Debug, PartialEq, Properties)]
pub(crate) struct MarkdownProviderProps {
    pub children: Children,
}

#[function_component]
pub(crate) fn MarkdownProvider(props: &MarkdownProviderProps) -> Html {
    let theme_name = use_state(|| Markdown::default());

    let theme_ctx = MarkdownContext::new(theme_name);

    html! {
        <ContextProvider<MarkdownContext> context={theme_ctx}>
            {props.children.clone()}
        </ContextProvider<MarkdownContext>>
    }
}

#[hook]
pub(crate) fn use_markdown() -> MarkdownContext {
    use_context::<MarkdownContext>().unwrap()
}
