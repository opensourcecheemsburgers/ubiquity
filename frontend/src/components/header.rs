use yew::prelude::*;
use yew_router::prelude::use_navigator;

use crate::{
    components::{divider::Divider, theme_card::ThemeDropdownItem, modal::{ModalProps, ModalType}},
    icons::{FileAddIcon, FolderAddIcon, PlaylistAddIcon, RedoIcon, UndoIcon, SaveIcon},
    Page, contexts::modals::use_modals_store,
};

#[derive(Properties, PartialEq)]
pub struct HeaderProps {}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let navigator = use_navigator().unwrap();
    let nav = navigator.clone();
    
    let modals = use_modals_store();
    let modals_html = use_modals_store().modals_html();
    let toggle_modal = Callback::from(move |click: MouseEvent| {
        let props = ModalProps { body: AttrValue::from("Oopsies, I haven't written anything here yet :("), title: AttrValue::from("Ubiquity"), btn_txt: AttrValue::from("ok"), modal_type: ModalType::Success };
        modals.display_modal(props);
    });

    let settings_cb = Callback::from(move |_| navigator.push(&Page::Settings));
    let about_cb = Callback::from(move |_| nav.push(&Page::About));

    html! {
      <div class="navbar bg-base-300">
          <div class="navbar-start">
            <SaveBtn />
            <AddFileBtn />
            <Divider />
            <UndoBtn />
            <RedoBtn />
          </div>

          <div class="navbar-center">
              <p onclick={toggle_modal} class="font-display text-2xl">{"Ubiquity"}</p>
          </div>

          <div class="navbar-end">
            <input type="text" placeholder="Search..." class="input input-sm input-bordered w-full max-w-xs mr-4" />

              <div class="dropdown dropdown-end">
                  <label tabindex="0" class="btn btn-ghost rounded-btn">
                  <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><circle cx="13.5" cy="6.5" r=".5"></circle><circle cx="17.5" cy="10.5" r=".5"></circle><circle cx="8.5" cy="7.5" r=".5"></circle><circle cx="6.5" cy="12.5" r=".5"></circle><path d="M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.554C21.965 6.012 17.461 2 12 2z"></path>
                  </svg>
                  </label>
                  <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
                    <ThemeDropdownItem name={"aqua"}/>
                    <ThemeDropdownItem name={"dark"}/>
                    <ThemeDropdownItem name={"forest"}/>
                    <ThemeDropdownItem name={"light"}/>
                    <ThemeDropdownItem name={"retro"}/>
                  </ul>
              </div>

              <div class="dropdown dropdown-end">
                  <label tabindex="0" class="btn btn-ghost rounded-btn">
                      <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24"
                          class="inline-block w-5 h-5 stroke-current">
                          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2"
                              d="M5 12h.01M12 12h.01M19 12h.01M6 12a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0zm7 0a1 1 0 11-2 0 1 1 0 012 0z" />
                      </svg>
                  </label>
                  <ul tabindex="0" class="menu dropdown-content p-2 shadow bg-base-300 rounded-box w-52 mt-4">
                      <li><a onclick={settings_cb}>{"Settings"}</a></li>
                      <li><a onclick={about_cb}>{"About"}</a></li>
                  </ul>
              </div>

          </div>
      </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct HeaderBtnProps {
    pub tip: AttrValue,
    pub children: Children,
}

#[function_component(HeaderBtn)]
pub fn header_btn(props: &HeaderBtnProps) -> Html {
    html! {
        <div data-tip={props.tip.clone()} class="tooltip tooltip-bottom tooltip-info">
            <button class="btn btn-ghost rounded-btn">
                {props.children.clone()}
            </button>
        </div>
    }
}

#[function_component(AddFileBtn)]
pub fn add_file_btn() -> Html {
    html! {
        <HeaderBtn tip={"Add File"}>
            <FileAddIcon />
        </HeaderBtn>
    }
}

#[function_component(SaveBtn)]
pub fn save_btn() -> Html {
    html! {
        <HeaderBtn tip={"Save File"}>
            <SaveIcon />
        </HeaderBtn>
    }
}

#[function_component(UndoBtn)]
pub fn undo_button() -> Html {
    html! {
        <HeaderBtn tip={"Undo"}>
            <UndoIcon />
        </HeaderBtn>
    }
}

#[function_component(RedoBtn)]
pub fn redo_button() -> Html {
    html! {
        <HeaderBtn tip={"Redo"}>
            <RedoIcon />
        </HeaderBtn>
    }
}
