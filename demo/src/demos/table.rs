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
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::table::StyledHeadDragHandler;
use leptodon::table::TailwindClassesPreset;
use leptodon::table::grouping::GroupRow;
use leptodon::table::grouping::GroupTableRowRenderer;
use leptodon::table::grouping::GroupingInfo;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;
use leptos_struct_table::HeadDragHandler;
use leptos_struct_table::TableContent;
use leptos_struct_table::TableDataProvider;
use leptos_struct_table::TableRow;
use std::sync::Arc;

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
                "When using paginated tables there is currently a bug that causes a scroll-to-top. To reduce the frequency of this issue you can increase your data-load chunks."
            </p>
            <GroupedTableExample />
        </FixedCenterColumn>
    }
}
