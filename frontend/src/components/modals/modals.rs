use yew::prelude::*;
use crate::components::modals::{add_image::AddImageModal, add_link::AddLinkModal, create_file::CreateFileModal, file_name::SelectNameModal, table::TableModal};

#[function_component(Modals)]
pub fn modals() -> Html {
    html! {
        <>
            <CreateFileModal />
            <SelectNameModal />
            <AddLinkModal />
            <AddImageModal />
            <TableModal />
        </>
    }
}