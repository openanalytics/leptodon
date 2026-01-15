use leptos::leptos_dom::logging::console_log;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Memo;
use leptos::prelude::NodeRef;
use leptos::prelude::Notify;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Trigger;
use leptos::prelude::Update;
use leptos::{IntoView, component, prelude::MaybeProp, view};
use nucleo_matcher::Config;
use nucleo_matcher::Matcher;
use nucleo_matcher::pattern::CaseMatching;
use nucleo_matcher::pattern::Normalization;
use nucleo_matcher::pattern::Pattern;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::sync::LazyLock;
use std::sync::Mutex;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;

use crate::checkbox::Checkbox;
use crate::class_list;
use crate::icon::Icon;
use crate::input::Input;
use crate::popover::Popover;
use crate::popover::PopoverController;
use crate::popover::PopoverPosition;
use crate::popover::PopoverTrigger;
use crate::popover::PopoverTriggerType;

const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const TAG_LIST_ITEM_CLASSES: &str =
    "hover:bg-oa-gray p-2 rounded-lg flex items-center cursor-pointer";

// Nucleo matcher allocates 135kb memory, so we want to reuse this.
// Try to not run multiple fuzzy matchers at the same time.
static NUCLEO_MATCHER: LazyLock<Mutex<Matcher>> =
    LazyLock::new(|| Mutex::new(Matcher::new(Config::DEFAULT)));

#[component]
pub fn TagPicker<T>(
    /// Shown when no tags are selected.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Subset of tags, containing the selected tags
    #[prop(optional, into)]
    selected: RwSignal<Vec<T>>,
    /// All tags
    #[prop(optional, into)]
    tags: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: AsRef<str> + Display + Eq + Hash + Clone + Send + Sync + 'static,
{
    let search_filter = RwSignal::new(String::new());
    let search_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    // Fuzzy search applier
    let tags_filtersorted: Memo<Vec<T>> = Memo::new(move |_old| {
        let search: String = search_filter.get().to_ascii_lowercase();
        let tags: Vec<T> = tags.get();

        let pattern = Pattern::parse(search.as_str(), CaseMatching::Smart, Normalization::Smart);
        let mut matcher = NUCLEO_MATCHER.lock().expect("Unpoised");
        let sorted_tags = pattern.match_list(tags, &mut matcher);

        sorted_tags.into_iter().map(|(tag, _score)| tag).collect()
    });

    // Partitioned into selected and unselected.
    // (Idx -> (T, is_selected))
    let tags_grouped = Memo::new(move |_old| {
        let selected = selected.get();
        let mut selected_tags = vec![];
        let mut unselected_tags = vec![];
        for tag in tags_filtersorted.get() {
            if selected.contains(&tag) {
                selected_tags.push((tag, true));
            } else {
                unselected_tags.push((tag, false));
            }
        }
        selected_tags.splice(selected_tags.len().., unselected_tags);
        selected_tags
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, (T, bool))>>()
    });

    let on_popover_open = move || {
        let Some(input): Option<HtmlInputElement> = search_ref.get() else {
            return;
        };
        input
            .focus()
            .expect("Tag picker search box should be focusable upon opening.");
    };

    let close_popover = Trigger::new();
    let popover_controller = PopoverController {
        close: close_popover,
        on_open: Some(on_popover_open.into()),
        on_close: None,
    };

    view! {
        <Popover show_arrow=false preferred_pos=PopoverPosition::BottomStart trigger_type=PopoverTriggerType::Click popover_controller>
            <PopoverTrigger slot>
                <div class=class_list!(SELECT_CLASSES, "cursor-default flex justify-between items-center")>
                    <div class="flex gap-2 overflow-scroll">
                        // Selected tags
                        <For
                            each=move || selected.get()
                            key=|tag| tag.clone()
                            let:tag
                            >
                            <div class="p-1.5 bg-oa-gray rounded-lg flex items-center gap-1.5">
                                <span>{tag.to_string()}</span>
                                <div class="p-1 hover:bg-oa-gray-mid hover:cursor-pointer rounded" on:click=move |ev| {
                                    ev.stop_propagation();
                                    selected.update(|vec| {
                                        vec.iter()
                                            .position(|e| e == &tag)
                                            .and_then(|pos| { Some(vec.remove(pos)) });
                                    });
                                }>
                                    <Icon class="w-3 h-3" icon=crate::icon::CloseIcon() />
                                </div>
                            </div>
                        </For>
                        // Placeholder, shown when empty
                        <div class="p-1.5 text-gray-600">
                        {move || {
                            if selected.get().is_empty() {
                                view! { {placeholder.get()} }.into_any()
                            } else {
                                ().into_any()
                            }
                        }}
                        </div>
                    </div>
                    <Icon class="text-oa-gray-darker w-4 h-4 ml-2" icon=crate::icon::DownIcon() />
                </div>
            </PopoverTrigger>

            // Popover Contents VV
            <ul>
                <Input class="mb-2" placeholder="Search..." value=search_filter input_ref=search_ref
                    on:keydown=move |key: KeyboardEvent| {
                        console_log(key.code().as_str());
                        if key.code() == "Escape" || key.code() == "Tab" {
                            close_popover.notify();
                        }
                        if key.code() == "Enter" {
                            let Some((_, (tag, is_selected))) = tags_grouped.get().first().cloned() else {
                                return;
                            };
                            selected.update(|vec| {
                                if is_selected {
                                    vec.iter()
                                        .position(|e| e == &tag)
                                        .map(|pos| { vec.remove(pos); });
                                } else {
                                    vec.push(tag.clone());
                                }
                            });
                        }
                    }
                    {..}
                    role="combobox" // Makes vimium like plugins pass special keys through
                />
                // Tags
                <For
                    each=move || tags_grouped.get()
                    key=|tag| {
                        tag.clone()
                    }
                    let((i, (tag, is_selected)))
                    >
                    <li
                        class=class_list!(
                            TAG_LIST_ITEM_CLASSES,
                            ("outline outline-black outline-2", move || i == 0 && !search_filter.get().is_empty())
                        )
                        on:click={ move |_| {
                            // Toggle selection
                            selected.update(|vec| {
                                if is_selected {
                                    vec.iter()
                                        .position(|e| e == &tag)
                                        .map(|pos| { vec.remove(pos); });
                                } else {
                                    vec.push(tag.clone());
                                }
                            });
                        }}
                    >
                        <Checkbox label=tag.to_string() value=move || is_selected disable_tab=true/>
                    </li>
                </For>
            </ul>
        </Popover>
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
