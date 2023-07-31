use lazy_static::__Deref;
use yew::prelude::*;
use crate::{contexts::{markdown::{use_markdown, Markdown}, toasts::{use_toaster, err_modal}}, components::{editor::textarea::textarea::EDITOR_ID, modals::utils::close_modal}};
use web_sys::{HtmlInputElement, HtmlParagraphElement, HtmlTextAreaElement};
use gloo::{utils::document, console::debug};
use wasm_bindgen::JsCast;


pub const TABLE_MODAL_ID: AttrValue = AttrValue::Static("table_modal");

const COLS_TEXTBOX_ID: AttrValue = AttrValue::Static("col_input");
const ROW_TEXTBOX_ID: AttrValue = AttrValue::Static("row_input");

const TABLE_PREVIEW_ID: AttrValue = AttrValue::Static("table_preview");

#[function_component(TableModal)]
pub fn table_modal() -> Html {
    let columns_state: UseStateHandle<Vec<Column>> = use_state_eq(|| {
        vec![Column::from(Alignment::Center)]
    });
    let col_amount: u32 = columns_state.len() as u32;
    let row_amount: UseStateHandle<u32> = use_state_eq(|| 1);


    let table_input = use_memo(
        |deps| {
        let mut columns_alignment_string = String::from("");
    
        columns_state.iter().for_each(|column| {
            let alignment = match column.alignment {
                Alignment::Left => 'l',
                Alignment::Center => 'c',
                Alignment::Right => 'r',
            };
            columns_alignment_string.push(alignment);
        });

            format!("{}{} x {}", deps.0, columns_alignment_string, deps.1.to_string())
        },
        (col_amount, row_amount.clone(), columns_state.clone()),
    );

    let table = use_memo(
        |table| {
            let table_result = mdtg::get_table(table.to_string());
            if let Ok(table) = table_result {
                table
            } else {
                debug!("Error making table: {}", table_result.unwrap_err().to_string());
                "".to_string()
            }
        },
        table_input.clone(),
    );

    let mut col_alignment_html: Vec<Html> = Vec::new();

    let mut i = 0;
    while i < col_amount {
        let title = format!("Column {}", i + 1);

        let columns_state = columns_state.clone();
        let columns = columns_state.to_vec();
        let col_index = i as usize;
        let current_column = columns[col_index].to_owned();

        let current_alignment = match current_column.alignment {
            Alignment::Left => "0",
            Alignment::Center => "1",
            Alignment::Right => "2",
        };
        
        let oninput = Callback::from({
            let columns_state: UseStateHandle<Vec<Column>> = columns_state.clone();

            move |input_event: InputEvent| {
                let target: HtmlInputElement = input_event.target().unwrap().dyn_into().unwrap();
                let alignment = {
                    if target.value().eq("0") {
                        Alignment::Left
                    }
                    else if target.value().eq("2") {
                        Alignment::Right
                    } else {
                        Alignment::Center
                    }
                };
                let column = Column::from(alignment);
                let mut new_columns = columns_state.deref().to_owned();

                new_columns.remove(i as usize);
                new_columns.insert(i as usize, column);
                columns_state.set(new_columns);

            }
        });

        let html = html! {
            <div class="w-48">
                <input {oninput} type="range" min="0" max="2" value={current_alignment} class="range range-xs" />
                <div class="flex justify-between text-xs px-2 font-light text-sm">
                    <span>{"Left"}</span>
                    <span>{"Center"}</span>
                    <span>{"Right"}</span>
                </div>
            </div>
        };

        col_alignment_html.push(html);
        i = i + 1;
    }

    let on_col_amount_change = Callback::from({
        let columns_state = columns_state.clone();
        move |_| {
            let col_textbox: HtmlInputElement = document().get_element_by_id(&COLS_TEXTBOX_ID).unwrap().dyn_into().unwrap();

            if !col_textbox.value().is_empty() && let Ok(cols) = col_textbox.value().parse::<i8>() {        
                let mut new_columns_state = columns_state.to_vec();
                new_columns_state.resize(cols as usize, Column::from(Alignment::Center));
                columns_state.set(new_columns_state);
            }
        }
    });

    let on_row_amount_change = Callback::from({
        let row_amount = row_amount.clone();

        move |_| {
            let row_textbox: HtmlInputElement = document().get_element_by_id(&ROW_TEXTBOX_ID).unwrap().dyn_into().unwrap();

            if !row_textbox.value().is_empty() && let Ok(rows) = row_textbox.value().parse::<u32>() {
                row_amount.set(rows);
            }
        }
    });


    let markdown_ctx = use_markdown();
    let toaster = use_toaster();
    let table = table.clone();
    let key = markdown_ctx.state().key;
    let insert_table = Callback::from(move |_| {
        let link_title_input: HtmlParagraphElement = document().get_element_by_id(&TABLE_PREVIEW_ID).unwrap().dyn_into().unwrap();
        let table = link_title_input.text_content().unwrap();

        let editor: HtmlTextAreaElement = document().get_element_by_id(&EDITOR_ID).unwrap().dyn_into().unwrap();
        let mut current_value = editor.value();

        if let Some(end) = editor.selection_end().unwrap() {
            let end_usize = end as usize;
            current_value.insert_str(end_usize, &table);
        } else {
            current_value.push_str(&table);
        }

        let new_md = Markdown::from(AttrValue::from(current_value), key.clone());
        markdown_ctx.update_markdown(new_md).unwrap_or_else(|err| err_modal(err, toaster.clone()));
        close_modal(&TABLE_MODAL_ID);
    });

    html! {
        <>
            <input type="checkbox" id={TABLE_MODAL_ID} class="modal-toggle" />
            <div class="modal">
                <div class="modal-box">
                    <h1 class="font-bold text-2xl">{"Add Table"}</h1>
                    <div class="flex flex-col space-y-8 mt-4">

                        <div class="flex flex-row space-x-12">
                            <div class="form-control w-12 max-w-xs">
                                <label class="label">
                                    <span class="label-text">{"Columns"}</span>
                                </label>
                                <input oninput={on_col_amount_change} id={COLS_TEXTBOX_ID} type="text"
                                    value={col_amount.to_string()} class="input input-bordered input-primary w-16" />
                            </div>
                            <div class="form-control w-12">
                                <label class="label">
                                    <span class="label-text">{"Rows"}</span>
                                </label>
                                <input oninput={on_row_amount_change} id={ROW_TEXTBOX_ID} type="text"
                                    value={row_amount.to_string()} class="input input-bordered input-primary w-16" />
                            </div>
                        </div>

                        <div class="flex flex-col space-y-4">
                            <h2 class="text-xl">{"Alignment"}</h2>
                            <div class="flex flex-col space-y-4">
                                {col_alignment_html}
                            </div>
                        </div>

                        <div class="flex flex-col space-y-4">
                            <h2 class="text-xl">{"Preview"}</h2>
                            <div class="bg-base-300 p-8 rounded-xl">
                                <div class="h-24 lg:h-36 2xl:h-48 overflow-auto">
                                    <p class="whitespace-pre-wrap" id={TABLE_PREVIEW_ID}>
                                        {table}
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="modal-action">
                        <label for={TABLE_MODAL_ID} class="btn btn-ghost">{"Cancel"}</label>
                        <button onclick={insert_table} class="btn">{"Generate"}</button>
                    </div>
                </div>
            </div>
        </>
    }
}

// fn update_table(
//     col_amount: UseStateHandle<i8>, 
//     row_amount: UseStateHandle<i8>, 
//     columns_state: UseStateHandle<Vec<Column>>,
//     table_input: UseStateHandle<String>,
//     table: UseStateHandle<String>
// ) {
//         let mut columns_alignment_string = String::from("");
        
//         columns_state.iter().enumerate().for_each(|(position, column)| {
//             let alignment = match column.alignment {
//                 Alignment::Left => 'l',
//                 Alignment::Center => 'c',
//                 Alignment::Right => 'r',
//             };
//             columns_alignment_string.remove(position + 1);
//             columns_alignment_string.insert(position + 1, alignment);
//         });

//         let table_input_string = format!("{}{} x {}", col_amount.to_string(), columns_alignment_string, row_amount.to_string());
//         let table_string = mdtg::get_table(table_input_string.clone()).unwrap();
//         table_input.set(table_input_string);
//         table.set(table_string);
// }

#[derive(PartialEq, Clone)]
struct Column {
    pub alignment: Alignment
}

impl Column {
    pub fn from(alignment: Alignment) -> Self {
        Self { alignment }
    }
}

#[derive(PartialEq, Clone)]
enum Alignment {
    Left,
    Center,
    Right
}

