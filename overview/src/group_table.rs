use leptos::component;
use leptos_struct_table::TableClassesProvider;
use leptos::either::Either;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::prelude::Set;
use leptos::{IntoView, logging::debug_log, view};
use leptos_components::button_group::Pagination;
use leptos_components::table::grouping::{GroupRow, GroupTableRowRenderer, GroupingInfo};
use leptos_struct_table::ColumnSort;
use leptos_struct_table::DisplayStrategy;
use leptos_struct_table::PaginationController;
use leptos_struct_table::TailwindClassesPreset;
use leptos_struct_table::{PaginatedTableDataProvider, TableContent, TableRow};
use std::sync::Arc;

// Define row type
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
    grouping_info: GroupingInfo<FlowerColumn>,
}

// Derive GroupRow to make the row groupable
impl GroupRow<FlowerColumn> for Flower {
    fn group_info(&self) -> &GroupingInfo<FlowerColumn> {
        &self.grouping_info
    }
}


#[component]
pub fn GroupedTableExample() -> impl IntoView {
    let rows = HardcodedFlowers;
    let pagination_controller = PaginationController::default();

    let strat = DisplayStrategy::Pagination {
        row_count: 15,
        controller: pagination_controller,
    };

    let visible_current_page = RwSignal::new(1);
    Effect::new(move || {
        let visible_page = visible_current_page.get();
        debug_log!("Moving to table page {visible_page}");
        pagination_controller.current_page.set(visible_page - 1);
    });

    view! {
        <div> hello </div>
        <table>
            <TableContent rows row_renderer=GroupTableRowRenderer display_strategy=strat scroll_container="html"></TableContent>
        </table>
        { move || {
            let opt_pg_count = pagination_controller.page_count().get();
            if let Some(page_count) = opt_pg_count {
                debug_log!("Table pagination enabled: {page_count} pages");
                Either::Left(view! {
                    <Pagination page_count=Signal::derive(move || page_count) current_page=visible_current_page jumper=true />
                })
            } else {
                Either::Right(())
            }
        }}
    }
}

// Paginated Data provider
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
        debug_log!("Flowers table has {} rows", get_flowers().len());
        Some(get_flowers().len())
    }

    async fn page_count(&self) -> Option<usize> {
        let pages = (get_flowers().len() + Self::PAGE_ROW_COUNT - 1) / Self::PAGE_ROW_COUNT;
        debug_log!(
            "Flowers table has {} rows and {} pages",
            get_flowers().len(),
            pages
        );
        Some(pages)
    }

    fn set_sorting(&mut self, sorting: &std::collections::VecDeque<(FlowerColumn, ColumnSort)>) {
        // by default do nothing
    }

    fn track(&self) {
        // by default do nothing
    }
}

// Table Data
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
