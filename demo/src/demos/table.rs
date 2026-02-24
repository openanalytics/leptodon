// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use leptodon::table::StyledHeadDragHandler;
use leptodon::table::grouping::GroupTableRowRenderer;
use leptos_struct_table::HeadDragHandler;
use leptos_struct_table::TableDataProvider;
use std::sync::Arc;

use attr_docgen::generate_codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::table::grouping::GroupRow;
use leptodon::table::grouping::GroupingInfo;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;
use leptos_struct_table::ColumnSort;
use leptos_struct_table::TableClassesProvider;
use leptos_struct_table::TableContent;
use leptos_struct_table::TableRow;

#[generate_codeblock(GroupedTableExample)]
#[component]
pub fn GroupedTableDemo() -> impl IntoView {
    // Define row type
    #[derive(TableRow, Clone, Default, Debug)]
    #[table(
        impl_vec_data_provider,
        classes_provider = "TailwindClassesPreset",
        column_index_type = "enum"
    )]
    pub struct Flower {
        species: String,
        sepal_width: f64,
        sepal_length: f64,
        petal_width: f64,
        petal_length: f64,

        #[table(skip)]
        grouping_info: GroupingInfo<FlowerColumn>,
    }

    // Derive GroupRow to make the row groupable
    impl GroupRow<FlowerColumn> for Flower {
        fn group_info(&self) -> &GroupingInfo<FlowerColumn> {
            &self.grouping_info
        }
    }

    let grouped_by = Arc::new(vec![FlowerColumn::SepalWidth]);
    let rows = vec![
        Flower {
            species: "Versicolor".to_string(),
            sepal_width: 2.2,
            sepal_length: 6.0,
            petal_width: 1.6,
            petal_length: 5.0,
            grouping_info: GroupingInfo {
                row_index: 0,
                nb_entries: 2,
                grouped_by: grouped_by.clone(),
            },
        },
        Flower {
            species: "Versicolor".to_string(),
            sepal_width: 2.2,
            sepal_length: 7.0,
            petal_width: 1.5,
            petal_length: 5.0,
            grouping_info: GroupingInfo {
                row_index: 1,
                nb_entries: 2,
                grouped_by,
            },
        },
    ];

    view! {
        <table>
            <TableContent
                rows=rows
                scroll_container="html"
                row_renderer=GroupTableRowRenderer
                drag_handler=HeadDragHandler::new(StyledHeadDragHandler)>
            </TableContent>
        </table>
    }
}

#[component]
pub fn TableDemoPage() -> impl IntoView {
    view! {
        <Title text="Table"/>

        <FixedCenterColumn>
            <Heading4 anchor="group-table">"Grouped Table"</Heading4>
            <p>
                "When using paginated tables there is currenlty a bug that causes a scroll-to-top. To reduce the frequency of this issue you can increase your data-load chunks."
            </p>
            <GroupedTableExample />
        </FixedCenterColumn>
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
