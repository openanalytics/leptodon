use itertools::Itertools;
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
use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::ops::Deref;
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
    let group_on = RwSignal::new(vec![]);

    view! {
        <div>Group on: </div>
        <TagPicker placeholder="Selects columns to group on" tags=RwSignal::new(column_tags) selected=group_on />
        <table>
            {move || {
                let group_on = group_on.get();
                let rows = LocalFlowers::new(group_on);
                view! {
                    <TableContent rows row_renderer=GroupTableRowRenderer display_strategy=strat scroll_container="html"></TableContent>
                }
            }}
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
struct LocalFlowers {
    rows: Vec<Flower>,
}
impl LocalFlowers {
    fn new(group_on: Vec<String>) -> Self {
        let group_on = group_on
            .iter()
            .filter_map(|column_name| {
                for col in Flower::columns() {
                    if column_name == Flower::col_name(*col) {
                        return Some(*col);
                    }
                }
                None
            })
            .collect_vec();
        let group_on = Arc::new(group_on);
        let grouped_flowers = group_flowers(get_flowers(), group_on.clone());
        debug_log!("Flower table contains {} flowers.", grouped_flowers.len());
        return LocalFlowers {
            rows: grouped_flowers,
        };
    }
}

impl PaginatedTableDataProvider<Flower, FlowerColumn> for LocalFlowers {
    const PAGE_ROW_COUNT: usize = 10;

    // 0-indexed
    async fn get_page(&self, page_index: usize) -> Result<Vec<Flower>, String> {
        let start_row = Self::PAGE_ROW_COUNT * page_index;
        let flowers = &self.rows;
        let end_row = std::cmp::min(flowers.len(), start_row + Self::PAGE_ROW_COUNT);
        let flowers = &flowers[start_row..end_row];
        Ok(Vec::from(flowers))
    }

    async fn row_count(&self) -> Option<usize> {
        debug_log!("Flowers table has {} rows", &self.rows.len());
        Some(self.rows.len())
    }

    async fn page_count(&self) -> Option<usize> {
        let pages = (&self.rows.len() + Self::PAGE_ROW_COUNT - 1) / Self::PAGE_ROW_COUNT;
        debug_log!(
            "Flowers table has {} rows and {} pages",
            &self.rows.len(),
            pages
        );
        Some(pages)
    }

    fn set_sorting(&mut self, _sorting: &std::collections::VecDeque<(FlowerColumn, ColumnSort)>) {
        // by default do nothing
    }

    fn track(&self) {
        // by default do nothing
    }
}

#[derive(Debug)]
pub struct DBFlower {
    species: String,
    sepal_width: f64,
    sepal_length: f64,
    petal_width: f64,
    petal_length: f64,
}

fn group_flowers(flowers: Vec<DBFlower>, group_on: Arc<Vec<FlowerColumn>>) -> Vec<Flower> {
    // Grouping requires we are sorted first
    let mut flowers = flowers;
    for column in group_on.deref() {
        match column {
            FlowerColumn::Species => {
                flowers.sort_by(|flower1, flower2| flower1.species.cmp(&flower2.species));
            }
            FlowerColumn::SepalWidth => flowers
                .sort_by(|flower1, flower2| flower1.sepal_width.total_cmp(&flower2.sepal_width)),
            FlowerColumn::SepalLength => flowers
                .sort_by(|flower1, flower2| flower1.sepal_length.total_cmp(&flower2.sepal_length)),
            FlowerColumn::PetalWidth => flowers
                .sort_by(|flower1, flower2| flower1.petal_width.total_cmp(&flower2.petal_width)),
            FlowerColumn::PetalLength => flowers
                .sort_by(|flower1, flower2| flower1.petal_length.total_cmp(&flower2.petal_length)),
        }
    }

    let mut groups = vec![];
    let mut group_builder = vec![];

    let mut prev_values = vec![];
    let last_row_idx = flowers.len() - 1;
    // Naive attempt at a grouping algorithm.
    for (row_index, flower) in flowers.into_iter().enumerate() {
        let mut cur_values = vec![];

        for column in group_on.deref() {
            match column {
                FlowerColumn::Species => {
                    let mut hasher = DefaultHasher::new();
                    flower.species.hash(&mut hasher);
                    cur_values.push(hasher.finish())
                }
                FlowerColumn::SepalWidth => cur_values.push(flower.sepal_width.to_bits()),
                FlowerColumn::SepalLength => cur_values.push(flower.sepal_length.to_bits()),
                FlowerColumn::PetalWidth => cur_values.push(flower.petal_width.to_bits()),
                FlowerColumn::PetalLength => cur_values.push(flower.petal_length.to_bits()),
            }
        }
        if prev_values == cur_values && row_index != last_row_idx {
            debug_log!(
                "Pushing a flower {flower:?} onto an existing group {cur_values:?} == {prev_values:?}"
            );
            group_builder.push(flower);
        } else {
            debug_log!("New group {flower:?} {cur_values:?} != {prev_values:?}");
            groups.push(group_builder);
            group_builder = vec![flower];
        }

        prev_values = cur_values;
    }
    let mut mapped = vec![];
    for group in groups {
        let group_size = group.len();
        for (row_index, flower) in group.into_iter().enumerate() {
            mapped.push(Flower {
                sepal_width: flower.sepal_width,
                sepal_length: flower.sepal_length,
                petal_width: flower.petal_width,
                petal_length: flower.petal_length,
                species: flower.species,
                grouping_info: GroupingInfo {
                    row_index: row_index as u32,
                    nb_entries: group_size as u32,
                    grouped_by: group_on.clone(),
                },
            });
        }
    }
    mapped
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
