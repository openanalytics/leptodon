use leptos::component;
use leptos::either::Either;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::prelude::Set;
use leptos::{IntoView, logging::debug_log, view};
use leptos_components::button_group::Pagination;
use leptos_components::table::grouping::{GroupRow, GroupTableRowRenderer, GroupingInfo};
use leptos_components::tag_picker::TagPicker;
use leptos_struct_table::ColumnSort;
use leptos_struct_table::DisplayStrategy;
use leptos_struct_table::PaginationController;
use leptos_struct_table::TableClassesProvider;
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
    let rows = ServerFlowers;
    let pagination_controller = PaginationController::default();

    let strat = DisplayStrategy::Pagination {
        row_count: 10,
        controller: pagination_controller,
    };

    let visible_current_page = RwSignal::new(1);
    Effect::new(move || {
        let visible_page = visible_current_page.get();
        debug_log!("Moving to table page {visible_page}");
        pagination_controller.current_page.set(visible_page - 1);
    });
    let column_tags = Flower::columns()
        .iter()
        .map(|c| Flower::col_name(*c).to_string())
        .collect::<Vec<_>>();
    view! {
        <div>Group on: </div>
        <TagPicker placeholder="Selects columns to group on" tags=RwSignal::new(column_tags)/>
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
struct ServerFlowers;
impl PaginatedTableDataProvider<Flower, FlowerColumn> for ServerFlowers {
    const PAGE_ROW_COUNT: usize = 10;

    // 0-indexed
    async fn get_page(&self, page_index: usize) -> Result<Vec<Flower>, String> {
        let start_row = Self::PAGE_ROW_COUNT * page_index;
        let flowers = &get_flowers();
        let end_row = std::cmp::min(flowers.len(), start_row + Self::PAGE_ROW_COUNT);
        let flowers = &flowers[start_row..end_row];
        // Ok(Vec::from(flowers));
        Ok(vec![])
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

pub struct DBFlower {
    species: String,
    sepal_width: f64,
    sepal_length: f64,
    petal_width: f64,
    petal_length: f64,
}

// Table Data
fn get_flowers() -> Vec<DBFlower> {
    vec![
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 2.2,
            petal_length: 5.0,
            petal_width: 1.5,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 2.5,
            petal_length: 4.5,
            petal_width: 1.7,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 2.5,
            petal_length: 5.8,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.5,
            petal_length: 5.0,
            petal_width: 1.9,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 2.5,
            petal_length: 5.0,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 2.6,
            petal_length: 5.6,
            petal_width: 1.4,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.7,
            sepal_width: 2.6,
            petal_length: 6.9,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.7,
            petal_length: 4.9,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 2.7,
            petal_length: 5.3,
            petal_width: 1.9,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.9,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.9,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.8,
            petal_length: 5.1,
            petal_width: 1.5,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.2,
            sepal_width: 2.8,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.4,
            sepal_width: 2.8,
            petal_length: 6.1,
            petal_width: 1.9,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.7,
            sepal_width: 2.8,
            petal_length: 6.7,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 2.8,
            petal_length: 4.9,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 2.8,
            petal_length: 5.6,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 2.8,
            petal_length: 5.6,
            petal_width: 2.2,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.8,
            petal_length: 5.1,
            petal_width: 2.4,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.9,
            petal_length: 5.6,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.3,
            sepal_width: 2.9,
            petal_length: 6.3,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.2,
            sepal_width: 3.0,
            petal_length: 5.8,
            petal_width: 1.6,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.5,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 3.0,
            petal_length: 4.9,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 3.0,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.9,
            sepal_width: 3.0,
            petal_length: 5.1,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.2,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.8,
            sepal_width: 3.0,
            petal_length: 5.5,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.6,
            sepal_width: 3.0,
            petal_length: 6.6,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.1,
            sepal_width: 3.0,
            petal_length: 5.9,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.5,
            sepal_width: 3.0,
            petal_length: 5.8,
            petal_width: 2.2,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.7,
            sepal_width: 3.0,
            petal_length: 6.1,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.0,
            petal_length: 5.2,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 3.1,
            petal_length: 5.5,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 5.4,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 5.1,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 5.6,
            petal_width: 2.4,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.2,
            sepal_width: 3.2,
            petal_length: 6.0,
            petal_width: 1.8,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.5,
            sepal_width: 3.2,
            petal_length: 5.1,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 3.2,
            petal_length: 5.3,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.9,
            sepal_width: 3.2,
            petal_length: 5.7,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.8,
            sepal_width: 3.2,
            petal_length: 5.9,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.3,
            petal_length: 5.7,
            petal_width: 2.1,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 3.3,
            petal_length: 6.0,
            petal_width: 2.5,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.3,
            petal_length: 5.7,
            petal_width: 2.5,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.2,
            sepal_width: 3.4,
            petal_length: 5.4,
            petal_width: 2.3,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 3.4,
            petal_length: 5.6,
            petal_width: 2.4,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.2,
            sepal_width: 3.6,
            petal_length: 6.1,
            petal_width: 2.5,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.9,
            sepal_width: 3.8,
            petal_length: 6.4,
            petal_width: 2.0,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 7.7,
            sepal_width: 3.8,
            petal_length: 6.7,
            petal_width: 2.2,
            species: "Virginica".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 2.0,
            petal_length: 3.5,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 2.2,
            petal_length: 4.0,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.2,
            sepal_width: 2.2,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 2.3,
            petal_length: 3.3,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.3,
            petal_length: 4.4,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 2.3,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 2.4,
            petal_length: 3.7,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 2.4,
            petal_length: 3.3,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 2.4,
            petal_length: 3.8,
            petal_width: 1.1,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 2.5,
            petal_length: 3.9,
            petal_width: 1.1,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 2.5,
            petal_length: 3.0,
            petal_width: 1.1,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 2.5,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 2.5,
            petal_length: 4.9,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 2.6,
            petal_length: 3.5,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.6,
            petal_length: 4.0,
            petal_width: 1.2,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 2.6,
            petal_length: 4.4,
            petal_width: 1.2,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 4.1,
            petal_width: 1.0,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 2.7,
            petal_length: 3.9,
            petal_width: 1.2,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 2.7,
            petal_length: 4.2,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.2,
            sepal_width: 2.7,
            petal_length: 3.9,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 2.7,
            petal_length: 5.1,
            petal_width: 1.6,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 2.8,
            petal_length: 4.7,
            petal_width: 1.2,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 2.8,
            petal_length: 4.0,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 2.8,
            petal_length: 4.1,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 2.8,
            petal_length: 4.5,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.8,
            sepal_width: 2.8,
            petal_length: 4.8,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.5,
            sepal_width: 2.8,
            petal_length: 4.6,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 2.9,
            petal_length: 4.3,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 2.9,
            petal_length: 3.6,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 2.9,
            petal_length: 4.2,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.2,
            sepal_width: 2.9,
            petal_length: 4.3,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.6,
            sepal_width: 2.9,
            petal_length: 4.6,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 2.9,
            petal_length: 4.7,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 2.9,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 3.0,
            petal_length: 4.2,
            petal_width: 1.2,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 3.0,
            petal_length: 4.1,
            petal_width: 1.3,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.1,
            sepal_width: 3.0,
            petal_length: 4.6,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.6,
            sepal_width: 3.0,
            petal_length: 4.4,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.9,
            sepal_width: 3.0,
            petal_length: 4.2,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.0,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.6,
            sepal_width: 3.0,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.0,
            petal_length: 5.0,
            petal_width: 1.7,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 4.4,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.7,
            sepal_width: 3.1,
            petal_length: 4.7,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.9,
            sepal_width: 3.1,
            petal_length: 4.9,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 7.0,
            sepal_width: 3.2,
            petal_length: 4.7,
            petal_width: 1.4,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.4,
            sepal_width: 3.2,
            petal_length: 4.5,
            petal_width: 1.5,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 5.9,
            sepal_width: 3.2,
            petal_length: 4.8,
            petal_width: 1.8,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.3,
            sepal_width: 3.3,
            petal_length: 4.7,
            petal_width: 1.6,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 6.0,
            sepal_width: 3.4,
            petal_length: 4.5,
            petal_width: 1.6,
            species: "Versicolor".into(),
        },
        DBFlower {
            sepal_length: 4.5,
            sepal_width: 2.3,
            petal_length: 1.3,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.4,
            sepal_width: 2.9,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.3,
            sepal_width: 3.0,
            petal_length: 1.1,
            petal_width: 0.1,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.8,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.1,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.0,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.4,
            sepal_width: 3.0,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.8,
            sepal_width: 3.0,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.1,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.6,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.8,
            sepal_width: 3.1,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 3.1,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.6,
            sepal_width: 3.2,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.4,
            sepal_width: 3.2,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.7,
            sepal_width: 3.2,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.7,
            sepal_width: 3.2,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.2,
            petal_length: 1.2,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.3,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.3,
            petal_length: 1.7,
            petal_width: 0.5,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.2,
            sepal_width: 3.4,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.8,
            sepal_width: 3.4,
            petal_length: 1.9,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.4,
            petal_length: 1.7,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.8,
            sepal_width: 3.4,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.6,
            sepal_width: 3.4,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.4,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.4,
            petal_length: 1.6,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 3.5,
            petal_length: 1.3,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.2,
            sepal_width: 3.5,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.5,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.5,
            petal_length: 1.3,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.5,
            petal_length: 1.4,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.5,
            petal_length: 1.6,
            petal_width: 0.6,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.9,
            sepal_width: 3.6,
            petal_length: 1.4,
            petal_width: 0.1,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.0,
            sepal_width: 3.6,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 4.6,
            sepal_width: 3.6,
            petal_length: 1.0,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.3,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.7,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.6,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 3.8,
            petal_length: 1.7,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.5,
            petal_width: 0.3,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.1,
            sepal_width: 3.8,
            petal_length: 1.9,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.9,
            petal_length: 1.3,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.4,
            sepal_width: 3.9,
            petal_length: 1.7,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.8,
            sepal_width: 4.0,
            petal_length: 1.2,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.2,
            sepal_width: 4.1,
            petal_length: 1.5,
            petal_width: 0.1,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.5,
            sepal_width: 4.2,
            petal_length: 1.4,
            petal_width: 0.2,
            species: "Setosa".into(),
        },
        DBFlower {
            sepal_length: 5.7,
            sepal_width: 4.4,
            petal_length: 1.5,
            petal_width: 0.4,
            species: "Setosa".into(),
        },
    ]
}
