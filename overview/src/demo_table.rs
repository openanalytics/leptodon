use ::chrono::NaiveDate;
use leptos::IntoView;
use leptos::component;
use leptos::either::Either;
use leptos::prelude::ElementChild;
use leptos::view;

use leptos::prelude::*;
use leptos_components::button::ControlButton;
use leptos_components::icon::EditIcon;
use leptos_struct_table::*;

// Tables are implemented in the leptos-struct-table crate
// This file contains an example table for demonstrative purposes.

#[derive(Clone, Copy, Default, Debug)]
struct AdminEditButtonCell {
    is_admin: bool,
}

#[derive(TableRow, Clone, Default, Debug)]
#[table(
    sortable,
    impl_vec_data_provider,
    classes_provider = "TailwindClassesPreset",
    column_index_type = "enum"
)]
pub struct Person {
    id: u32,
    name: String,
    age: u32,
    date: NaiveDate,
    #[table(renderer = "AdminEditButtonCellRenderer", skip_sort)]
    _admin_editable: FieldGetter<AdminEditButtonCell>,
}

impl Person {
    fn _admin_editable(&self) -> AdminEditButtonCell {
        AdminEditButtonCell { is_admin: true }
    }
}

#[component]
fn AdminEditButtonCellRenderer(
    class: String,
    value: Signal<AdminEditButtonCell>,
    row: RwSignal<Person>,
    index: PersonColumn,
) -> impl IntoView {
    let (_, _) = (row, index);
    view! {
        <td class=class>
            {move || if value.get().is_admin {
                Either::Left(view! {
                    <ControlButton icon=EditIcon() on_click=move |_| {
                    }
                    />
                })
            } else {
                Either::Right(())
            }}
        </td>
    }
}

#[component]
pub fn DemoTable() -> impl IntoView {
    let date = NaiveDate::from_ymd_opt(2025, 10, 10).unwrap();
    let admin_editable = FieldGetter::default();
    let rows = vec![
        Person {
            id: 1,
            name: "John".to_string(),
            age: 32,
            date,
            _admin_editable: admin_editable,
        },
        Person {
            id: 2,
            name: "Jane".to_string(),
            age: 28,
            date,
            _admin_editable: admin_editable,
        },
        Person {
            id: 3,
            name: "Bob".to_string(),
            age: 45,
            date,
            _admin_editable: admin_editable,
        },
        Person {
            id: 4,
            name: "Bob".to_string(),
            age: 46,
            date,
            _admin_editable: admin_editable,
        },
        Person {
            id: 5,
            name: "Bob".to_string(),
            age: 47,
            date,
            _admin_editable: admin_editable,
        },
        Person {
            id: 6,
            name: "Bob".to_string(),
            age: 45,
            date,
            _admin_editable: admin_editable,
        },
    ];

    view! {
        <div> hello </div>
        <table>
            <TableContent rows scroll_container="html"></TableContent>
        </table>
    }
}

#[derive(Clone, Copy)]
pub struct TailwindClassesPreset;

impl TableClassesProvider for TailwindClassesPreset {
    fn new() -> Self {
        Self
    }

    fn thead_row(&self, template_classes: &str) -> String {
        format!(
            "{} {}",
            "text-xs text-gray-700 uppercase bg-gray-200 dark:bg-gray-700 dark:text-gray-300",
            template_classes
        )
    }

    fn thead_cell(&self, sort: ColumnSort, template_classes: &str) -> String {
        let sort_class = match sort {
            ColumnSort::None => "",
            _ => "text-black dark:text-white",
        };

        format!(
            "cursor-pointer px-5 py-2 {} {}",
            sort_class, template_classes
        )
    }

    fn thead_cell_inner(&self) -> String {
        "flex items-center after:content-[--sort-icon] after:pl-1 after:opacity-40 before:content-[--sort-priority] before:order-last before:pl-0.5 before:font-light before:opacity-40".to_string()
    }

    fn row(&self, row_index: usize, selected: bool, template_classes: &str) -> String {
        let bg_color = if row_index.is_multiple_of(2) {
            if selected {
                "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
            } else {
                "bg-white dark:bg-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800"
            }
        } else if selected {
            "bg-sky-300 text-gray-700 dark:bg-sky-700 dark:text-gray-400"
        } else {
            "bg-gray-50 dark:bg-gray-800 hover:bg-gray-100 dark:hover:bg-gray-700"
        };

        format!(
            "{} {} {}",
            "border-b dark:border-gray-700", bg_color, template_classes
        )
    }

    fn loading_cell(&self, _row_index: usize, _col_index: usize, prop_class: &str) -> String {
        format!("{} {}", "px-5 py-2", prop_class)
    }

    fn loading_cell_inner(&self, row_index: usize, _col_index: usize, prop_class: &str) -> String {
        let width = match row_index % 4 {
            0 => "w-[calc(85%-2.5rem)]",
            1 => "w-[calc(90%-2.5rem)]",
            2 => "w-[calc(75%-2.5rem)]",
            _ => "w-[calc(60%-2.5rem)]",
        };
        format!(
            "animate-pulse h-2 bg-gray-200 rounded-full dark:bg-gray-700 inline-block align-middle {} {}",
            width, prop_class
        )
    }

    fn cell(&self, template_classes: &str) -> String {
        format!("{} {}", "px-5 py-2", template_classes)
    }
}
