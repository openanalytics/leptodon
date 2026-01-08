use leptos::{
    either::Either, logging::debug_log, prelude::{ClassAttribute, Effect, ElementChild, For, Get, GetUntracked, IntoAny, OnAttribute, Set, Show}
};
use std::sync::Arc;

use leptos::{IntoView, component, view};
use leptos_struct_table::*;

use crate::button_group::Pagination;

#[cfg(test)]
mod test {
    use std::{fs::File, io::BufReader};

    use polars::{frame::row::Row, prelude::*};

    #[test]
    fn polars_playground() -> PolarsResult<()> {
        let csv_file = File::open("/home/mverstraete/Downloads/iris.csv")?;
        let reader = BufReader::new(csv_file);
        let sorted_csv = CsvReader::new(reader)
            .finish()? // get the final LazyFrame
            .sort(["sepal_width", "petal_width"], Default::default())?;
        let group_columns = vec!["species"];
        let query = sorted_csv.group_by(group_columns.clone())?;

        let groups = query.groups()?;
        // dbg!(query.get_groups());

        for group_idx in 0..groups.height() {
            let grouped_row = groups.get_row(group_idx)?;
            // dbg!(&grouped_row);
            match &grouped_row {
                Row(group_details) => {
                    let mut details = group_details.iter();
                    let group_name = details.next().unwrap().get_str().unwrap();
                    // dbg!(group_name);

                    let group_row = details.next().unwrap();
                    match group_row {
                        AnyValue::List(series) => {
                            // Rows that make up this group
                            let row_ids = series.u32()?;
                            let row_ids: Vec<_> = row_ids.into_no_null_iter().collect();
                            let sliced = sorted_csv.take(&IdxCa::new("idx".into(), &row_ids))?;
                            for row_idx in 0..sliced.height() {
                                let row = sliced.get_row(row_idx)?;
                                let mut row_iter = row.0.iter();
                                let missing_opt_err = PolarsError::ColumnNotFound(
                                    "Missing colunn".to_string().into(),
                                );
                                let sepal_length = row_iter
                                    .next()
                                    .ok_or(missing_opt_err.clone())?
                                    .try_extract::<f64>()?;
                                let sepal_width = row_iter
                                    .next()
                                    .ok_or(missing_opt_err.clone())?
                                    .try_extract::<f64>()?;
                                let petal_length = row_iter
                                    .next()
                                    .ok_or(missing_opt_err.clone())?
                                    .try_extract::<f64>()?;
                                let petal_width = row_iter
                                    .next()
                                    .ok_or(missing_opt_err.clone())?
                                    .try_extract::<f64>()?;
                                let species = row_iter
                                    .next()
                                    .ok_or(missing_opt_err.clone())?
                                    .extract_str()
                                    .unwrap();
                                print!(
                                    "
                                Flower {{
                                    sepal_length: {:?},
                                    sepal_width: {:?},
                                    petal_length: {:?},
                                    petal_width: {:?},
                                    species: {:?}.into(),
                                    grouping_info: GroupingInfo {{
                                        row_index: {},
                                        nb_entries: {},
                                        grouped_by: Arc::new(vec![FlowerColumn::Species]),
                                    }}
                                }},",
                                    sepal_length,
                                    sepal_width,
                                    petal_length,
                                    petal_width,
                                    species,
                                    row_idx,
                                    row_ids.len()
                                );
                            }
                            println!("");
                            // dbg!(sliced);
                        }
                        _ => unreachable!(),
                    }

                    // dbg!(group_row);
                }
            }
        }

        println!(
            "To tell Polars we want to execute a query in streaming mode we pass the streaming=True argument to collect()"
        );
        println!("Note that we have to activate the `streaming` polars feature in Cargo.toml");

        Ok(())
    }
}

/// Custom row renderer that adds a link to the end of the row
#[allow(unused_variables, non_snake_case)]
pub fn GroupTableRowRenderer(
    // The class attribute for the row element. Generated by the classes provider.
    class: Signal<String>,
    // The row to render.
    row: RwSignal<Flower>,
    // The index of the row. Starts at 0 for the first body row.
    index: usize,
    // The selected state of the row. True, when the row is selected.
    selected: Signal<bool>,
    // Event handler callback when this row is selected
    on_select: EventHandler<web_sys::MouseEvent>,
    // Columns to show and their order.
    columns: RwSignal<Vec<FlowerColumn>>,
) -> impl IntoView {
    debug_log!("Index of row: {}", index);
    leptos::view! {
        <tr class=class on:click=move |mouse_event| on_select.run(mouse_event)>
            <For
                each=move || columns.get().into_iter()
                key=|column| column.clone()
                children=move |column| {
                    let grouping_info = row.get().grouping_info;
                    if !grouping_info.grouped_by.is_empty() {
                        if grouping_info.nb_entries > 1 {
                            if grouping_info.row_index == 0 {
                                // Group-heading-row
                                // Split rendering into 2 rows, first one below
                                if grouping_info.grouped_by.contains(&column) {
                                    return Flower::cell_renderer_for_column(row, column)
                                }
                            } else {
                                // Content-row
                                // Skip rendering of grouped columns
                                if !grouping_info.grouped_by.contains(&column) {
                                    return Flower::cell_renderer_for_column(row, column)
                                }
                            }
                        } else {
                            // Single row in the group
                            // TODO: highlight cells in the grouped columns
                            return Flower::cell_renderer_for_column(row, column)
                        }
                    }
                    return view!{ <td/>}.into_any()
                }>
            </For>
        </tr>
        <Show
            when=move || {
                let grouping_info = row.get().grouping_info;
                grouping_info.row_index == 0 && grouping_info.nb_entries > 1 && !grouping_info.grouped_by.is_empty()
            }
            fallback=|| ()
        >
            <For
                each=move || columns.get().into_iter()
                key=|column| column.clone()
                children=move |column| {
                    let grouping_info = row.get().grouping_info;
                    if !grouping_info.grouped_by.contains(&column) {
                        return Flower::cell_renderer_for_column(row, column)
                    }
                    return view!{ <td/>}.into_any()
                }>
            </For>
        </Show>
    }
}

#[derive(TableRow, Clone, Default, Debug)]
#[table(
    sortable,
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
    grouping_info: GroupingInfo,
}

#[derive(Debug, Clone, Default)]
struct GroupingInfo {
    // Row index with respect to the group
    row_index: u32,
    // Number of entries in this group
    nb_entries: u32,
    // Grouping by certain columns
    grouped_by: Arc<Vec<FlowerColumn>>,
}

struct HardcodedFlowers;
impl PaginatedTableDataProvider<Flower, FlowerColumn> for HardcodedFlowers {
    const PAGE_ROW_COUNT: usize = 10;

    // 0-indexed
    async fn get_page(&self, page_index: usize) -> Result<Vec<Flower>, String> {
        let start_row = Self::PAGE_ROW_COUNT * page_index;
        let flowers = &get_flowers();
        let end_row = std::cmp::min(flowers.len(), start_row + Self::PAGE_ROW_COUNT);
        let flowers = &flowers[start_row..end_row];
        Ok(Vec::from(flowers))
    }

    async fn row_count(&self) -> Option<usize> {
        Some(151)
    }

    async fn page_count(&self) -> Option<usize> {
        Some(15)
    }

    fn set_sorting(&mut self, sorting: &std::collections::VecDeque<(FlowerColumn, ColumnSort)>) {
        // by default do nothing
    }

    fn track(&self) {
        // by default do nothing
    }
}

#[component]
pub fn GroupedTable() -> impl IntoView {
    let rows = HardcodedFlowers;
    let pagination_controller = PaginationController::default();
    let strat = DisplayStrategy::Pagination {
        row_count: 10,
        controller: pagination_controller,
    };
    let visible_current_page = RwSignal::new(1);
    Effect::new(move || {
        pagination_controller
            .current_page
            .set(visible_current_page.get() - 1);
    });
    view! {
        <div> hello </div>
        <table>
            <TableContent rows row_renderer=GroupTableRowRenderer display_strategy=strat scroll_container="html"></TableContent>
        </table>
        { move || {
            let opt_pg_count = pagination_controller.page_count().get();
            if let Some(page_count) = opt_pg_count {
                Either::Left(view! {
                    <Pagination page_count=Signal::derive(move || page_count) current_page=visible_current_page jumper=true />
                })
            } else {
                Either::Right(())
            }
        }}
    }
}

fn get_flowers() -> Vec<Flower> {
    vec![
        Flower {
            sepal_length: 6.0,
            sepal_width: 2.2,
            petal_length: 5.0,
            petal_width: 1.5,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 0,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 2.5,
            petal_length: 4.5,
            petal_width: 1.7,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 1,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 2.5,
            petal_length: 5.8,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 2,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.5,
            petal_length: 5.0,
            petal_width: 1.9,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 3,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 2.5,
            petal_length: 5.0,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 4,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 2.6,
            petal_length: 5.6,
            petal_width: 1.4,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 5,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.7,
            sepal_width: 2.6,
            petal_length: 6.9,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 6,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.7,
            petal_length: 4.9,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 7,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 2.7,
            petal_length: 5.3,
            petal_width: 1.9,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 8,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.9,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 9,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.9,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 10,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.8,
            petal_length: 5.1,
            petal_width: 1.5,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 11,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.2,
            sepal_width: 2.8,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 12,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.4,
            sepal_width: 2.8,
            petal_length: 6.1,
            petal_width: 1.9,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 13,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.7,
            sepal_width: 2.8,
            petal_length: 6.7,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 14,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 2.8,
            petal_length: 4.9,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 15,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 2.8,
            petal_length: 5.6,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 16,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 2.8,
            petal_length: 5.6,
            petal_width: 2.2,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 17,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.8,
            petal_length: 5.1,
            petal_width: 2.4,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 18,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.9,
            petal_length: 5.6,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 19,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.3,
            sepal_width: 2.9,
            petal_length: 6.3,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 20,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.2,
            sepal_width: 3.0,
            petal_length: 5.8,
            petal_width: 1.6,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 21,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.5,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 22,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 3.0,
            petal_length: 4.9,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 23,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.0,
            sepal_width: 3.0,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 24,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.9,
            sepal_width: 3.0,
            petal_length: 5.1,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 25,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.2,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 26,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.8,
            sepal_width: 3.0,
            petal_length: 5.5,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 27,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.6,
            sepal_width: 3.0,
            petal_length: 6.6,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 28,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.1,
            sepal_width: 3.0,
            petal_length: 5.9,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 29,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.8,
            petal_width: 2.2,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 30,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.7,
            sepal_width: 3.0,
            petal_length: 6.1,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 31,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.0,
            petal_length: 5.2,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 32,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 3.1,
            petal_length: 5.5,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 33,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 5.4,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 34,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 5.1,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 35,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 5.6,
            petal_width: 2.4,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 36,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.2,
            sepal_width: 3.2,
            petal_length: 6.0,
            petal_width: 1.8,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 37,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.5,
            sepal_width: 3.2,
            petal_length: 5.1,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 38,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 3.2,
            petal_length: 5.3,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 39,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.9,
            sepal_width: 3.2,
            petal_length: 5.7,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 40,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.8,
            sepal_width: 3.2,
            petal_length: 5.9,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 41,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.3,
            petal_length: 5.7,
            petal_width: 2.1,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 42,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 3.3,
            petal_length: 6.0,
            petal_width: 2.5,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 43,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.3,
            petal_length: 5.7,
            petal_width: 2.5,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 44,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.2,
            sepal_width: 3.4,
            petal_length: 5.4,
            petal_width: 2.3,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 45,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 3.4,
            petal_length: 5.6,
            petal_width: 2.4,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 46,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.2,
            sepal_width: 3.6,
            petal_length: 6.1,
            petal_width: 2.5,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 47,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.9,
            sepal_width: 3.8,
            petal_length: 6.4,
            petal_width: 2.0,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 48,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.7,
            sepal_width: 3.8,
            petal_length: 6.7,
            petal_width: 2.2,
            species: "Virginica".into(),
            grouping_info: GroupingInfo {
                row_index: 49,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 2.0,
            petal_length: 3.5,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 0,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.0,
            sepal_width: 2.2,
            petal_length: 4.0,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 1,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.2,
            sepal_width: 2.2,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 2,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 2.3,
            petal_length: 3.3,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 3,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.3,
            petal_length: 4.4,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 4,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 2.3,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 5,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 2.4,
            petal_length: 3.7,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 6,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 2.4,
            petal_length: 3.3,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 7,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 2.4,
            petal_length: 3.8,
            petal_width: 1.1,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 8,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 2.5,
            petal_length: 3.9,
            petal_width: 1.1,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 9,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 2.5,
            petal_length: 3.0,
            petal_width: 1.1,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 10,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 2.5,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 11,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 2.5,
            petal_length: 4.9,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 12,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 2.6,
            petal_length: 3.5,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 13,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.6,
            petal_length: 4.0,
            petal_width: 1.2,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 14,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 2.6,
            petal_length: 4.4,
            petal_width: 1.2,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 15,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 4.1,
            petal_width: 1.0,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 16,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 3.9,
            petal_width: 1.2,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 17,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 2.7,
            petal_length: 4.2,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 18,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.2,
            sepal_width: 2.7,
            petal_length: 3.9,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 19,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.0,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.6,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 20,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 2.8,
            petal_length: 4.7,
            petal_width: 1.2,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 21,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 2.8,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 22,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 2.8,
            petal_length: 4.1,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 23,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 2.8,
            petal_length: 4.5,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 24,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.8,
            sepal_width: 2.8,
            petal_length: 4.8,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 25,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.5,
            sepal_width: 2.8,
            petal_length: 4.6,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 26,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 2.9,
            petal_length: 4.3,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 27,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 2.9,
            petal_length: 3.6,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 28,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 2.9,
            petal_length: 4.2,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 29,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.2,
            sepal_width: 2.9,
            petal_length: 4.3,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 30,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.6,
            sepal_width: 2.9,
            petal_length: 4.6,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 31,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 2.9,
            petal_length: 4.7,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 32,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.0,
            sepal_width: 2.9,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 33,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 3.0,
            petal_length: 4.2,
            petal_width: 1.2,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 34,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 3.0,
            petal_length: 4.1,
            petal_width: 1.3,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 35,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.1,
            sepal_width: 3.0,
            petal_length: 4.6,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 36,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.6,
            sepal_width: 3.0,
            petal_length: 4.4,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 37,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.9,
            sepal_width: 3.0,
            petal_length: 4.2,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 38,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.0,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 39,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.6,
            sepal_width: 3.0,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 40,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.0,
            petal_length: 5.0,
            petal_width: 1.7,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 41,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 4.4,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 42,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 4.7,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 43,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 4.9,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 44,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 7.0,
            sepal_width: 3.2,
            petal_length: 4.7,
            petal_width: 1.4,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 45,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.4,
            sepal_width: 3.2,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 46,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.9,
            sepal_width: 3.2,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 47,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.3,
            sepal_width: 3.3,
            petal_length: 4.7,
            petal_width: 1.6,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 48,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 6.0,
            sepal_width: 3.4,
            petal_length: 4.5,
            petal_width: 1.6,
            species: "Versicolor".into(),
            grouping_info: GroupingInfo {
                row_index: 49,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.5,
            sepal_width: 2.3,
            petal_length: 1.3,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 0,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.4,
            sepal_width: 2.9,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 1,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.3,
            sepal_width: 3.0,
            petal_length: 1.1,
            petal_width: 0.1,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 2,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.8,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.1,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 3,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 4,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.0,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 5,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.4,
            sepal_width: 3.0,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 6,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.8,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 7,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.1,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 8,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.6,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 9,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.8,
            sepal_width: 3.1,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 10,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 11,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.6,
            sepal_width: 3.2,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 12,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.4,
            sepal_width: 3.2,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 13,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.7,
            sepal_width: 3.2,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 14,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.7,
            sepal_width: 3.2,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 15,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.2,
            petal_length: 1.2,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 16,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.3,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 17,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.3,
            petal_length: 1.7,
            petal_width: 0.5,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 18,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 19,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.2,
            sepal_width: 3.4,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 20,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.8,
            sepal_width: 3.4,
            petal_length: 1.9,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 21,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.4,
            petal_length: 1.7,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 22,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 23,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.8,
            sepal_width: 3.4,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 24,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.6,
            sepal_width: 3.4,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 25,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 26,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.4,
            petal_length: 1.6,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 27,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 3.5,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 28,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.2,
            sepal_width: 3.5,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 29,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.5,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 30,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.5,
            petal_length: 1.3,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 31,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.5,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 32,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.5,
            petal_length: 1.6,
            petal_width: 0.6,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 33,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.9,
            sepal_width: 3.6,
            petal_length: 1.4,
            petal_width: 0.1,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 34,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.0,
            sepal_width: 3.6,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 35,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 4.6,
            sepal_width: 3.6,
            petal_length: 1.0,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 36,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 37,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.3,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 38,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 39,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 40,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 3.8,
            petal_length: 1.7,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 41,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.5,
            petal_width: 0.3,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 42,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.9,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 43,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.9,
            petal_length: 1.3,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 44,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.4,
            sepal_width: 3.9,
            petal_length: 1.7,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 45,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.8,
            sepal_width: 4.0,
            petal_length: 1.2,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 46,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.2,
            sepal_width: 4.1,
            petal_length: 1.5,
            petal_width: 0.1,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 47,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.5,
            sepal_width: 4.2,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 48,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
        Flower {
            sepal_length: 5.7,
            sepal_width: 4.4,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
            grouping_info: GroupingInfo {
                row_index: 49,
                nb_entries: 50,
                grouped_by: Arc::new(vec![FlowerColumn::Species]),
            },
        },
    ]
}
