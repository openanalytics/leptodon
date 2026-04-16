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
use leptodon_proc_macros::generate_docs;
use leptos::ev::EventCallback;
use leptos::html::Div;
use leptos::logging::debug_log;
use leptos::logging::error;
use leptos::logging::warn;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::Memo;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::Notify;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Trigger;
use leptos::prelude::Update;
use leptos::prelude::Write;
use leptos::prelude::set_timeout;
use leptos::{IntoView, component, prelude::MaybeProp, view};
use nucleo_matcher::Config;
use nucleo_matcher::Matcher;
use nucleo_matcher::pattern::CaseMatching;
use nucleo_matcher::pattern::Normalization;
use nucleo_matcher::pattern::Pattern;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;
use web_sys::Element;
use web_sys::HtmlDivElement;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;
use web_sys::ScrollIntoViewContainer;
use web_sys::ScrollIntoViewOptions;
use web_sys::ScrollLogicalPosition;

use crate::checkbox::Checkbox;
use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::icon::Icon;
use crate::input::PLACEHOLDER_TEXT_CLASS;
use crate::input::TextInput;
use crate::popover::Popover;
use crate::popover::PopoverAnchor;
use crate::popover::PopoverController;
use crate::popover::PopoverHeader;
use crate::popover::PopoverTrigger;
use crate::popover::PopoverTriggerType;
use crate::radio::FormValue;

const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const TAG_LIST_ITEM_CLASSES: &str =
    "hover:bg-oa-gray hover:dark:bg-gray-600 p-2 rounded-lg flex items-center cursor-pointer";

// Nucleo matcher allocates 135kb memory, so we want to reuse this.
// Try to not run multiple fuzzy matchers at the same time.
static NUCLEO_MATCHER: LazyLock<Mutex<Matcher>> =
    LazyLock::new(|| Mutex::new(Matcher::new(Config::DEFAULT)));

#[generate_docs]
#[component]
/// T's AsRef<str> will be shown to the user and will be fuzzy searched on.
pub fn TagPicker<T>(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeReactiveClass,
    /// Shown when no tags are selected.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Subset of tags, containing the selected tags
    #[prop(optional)]
    selected: RwSignal<Vec<T>>,
    /// All tags
    #[prop(optional)]
    tags: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: AsRef<str> + FormValue + Eq + Hash + Clone + Send + Sync + 'static,
{
    let search_filter = RwSignal::new(String::new());
    // Refs used for focus transfer
    let tag_picker_ref: NodeRef<leptos::html::Div> = NodeRef::new();
    let content_scroll_container = NodeRef::new();
    let search_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    let checkboxes = RwSignal::new(HashMap::<T, RwSignal<bool>>::new());
    let inside_selected = RwSignal::new(selected.get_untracked());
    // Index of which element the select box is focusing, defaults to 0/first element, can be moved via arrowUp, arrowDown.
    let focus_ith = RwSignal::new(0);

    // When the outside tags change, update selected tags as some may have become non-options.
    Effect::watch(
        move || tags.get(),
        move |new, old, _| {
            let selected_value = inside_selected.get_untracked();
            let leftover_selected = selected_value
                .into_iter()
                .filter(|selected_value| new.contains(selected_value))
                .collect::<Vec<_>>();

            let mut checkboxes = checkboxes.write();
            for tag in new {
                if let Some(old) = old
                    && old.contains(tag)
                {
                    // leave it the same
                } else {
                    // new tag
                    checkboxes.insert(tag.clone(), RwSignal::new(leftover_selected.contains(tag)));
                    debug_log!("Added {}'s checkbox", tag.as_ref());
                }
            }
            if let Some(old) = old {
                for tag in old {
                    if new.contains(tag) {
                        // leave it the same
                    } else {
                        // removed tag
                        checkboxes.remove(tag);
                        debug_log!("Removed {}'s checkbox", tag.as_ref());
                    }
                }
            }
            drop(checkboxes);
            inside_selected.set(leftover_selected);
        },
        true,
    );

    // When the inside selection change, propagate to outside.
    Effect::new(move || selected.set(inside_selected.get()));

    // When the outside selection change, Sanity check the selection, purge unselectable tags..
    Effect::watch(
        move || selected.get(),
        move |new, _, _| {
            // Make sure this is not the value we just wrote to the outside
            if new != &inside_selected.get_untracked() {
                let checkboxes = checkboxes.get_untracked();
                let tags = tags.get_untracked();
                let leftover_selected = new
                    .clone()
                    .into_iter()
                    .filter(|selected_value| tags.contains(selected_value))
                    .collect::<Vec<_>>();

                // Update signals, could do less updates but that requires a lot more code, seemed like premature optimisation.
                for (checkbox_tag, checkbox_signal) in checkboxes {
                    checkbox_signal.set(leftover_selected.contains(&checkbox_tag));
                }

                inside_selected.set(leftover_selected);
            }
        },
        false,
    );

    // Fuzzy search applier
    let tags_filtersorted: Memo<Vec<T>> = Memo::new(move |_old| {
        let search: String = search_filter.get().to_ascii_lowercase();
        let tags: Vec<T> = tags.get();

        debug_log!("Filtering tags on \"{search}\"");

        let pattern = Pattern::parse(search.as_str(), CaseMatching::Smart, Normalization::Smart);
        let mut matcher = NUCLEO_MATCHER.lock().expect("Unpoised");
        let filtersorted_tags = pattern.match_list(tags, &mut matcher);

        /* Pigibacking code snippet, unrelated to fuzzy search, keeps focus_ith in-bounds based on # visible entries */
        let focus_ith_value = focus_ith.get();
        if focus_ith_value >= filtersorted_tags.len() {
            focus_ith.set(filtersorted_tags.len().saturating_sub(1));
        }
        /* end */

        filtersorted_tags
            .into_iter()
            .map(|(tag, _score)| tag)
            .collect()
    });

    // Sorted into selected and unselected.
    // [selected_T|unselected_T]
    let tags_grouped = Memo::new(move |_old| {
        if checkboxes.get().is_empty() {
            return vec![];
        }
        let selected = inside_selected.get();
        let search = search_filter.get();
        // [all] will contain: selected|unselected.
        let mut all = vec![];
        let mut unselected_group = vec![];
        // Group/parition by selected status, unless there is a filter
        for tag in tags_filtersorted.get() {
            if selected.contains(&tag) || !search.is_empty() {
                all.push(tag);
            } else {
                unselected_group.push(tag);
            }
        }

        all.append(&mut unselected_group);
        all.into_iter().collect::<Vec<T>>()
    });

    let on_popover_open = move || {
        let Some(input): Option<HtmlInputElement> = search_ref.get_untracked() else {
            return;
        };

        // Run next tick such that the input box can first unhide itself. Can't focus invisible elements.
        set_timeout(
            move || {
                input
                    .focus()
                    .expect("Tag picker search box should be focusable upon opening.");
            },
            Duration::ZERO,
        );
    };

    let open_popover = Trigger::new();
    let close_popover = Trigger::new();
    let popover_controller = PopoverController {
        open: Some(open_popover),
        close: Some(close_popover),
        on_open: Some(on_popover_open.into()),
        on_close: None,
    };

    view! {
        <Popover id class
            show_arrow=false
            preferred_pos=PopoverAnchor::BottomStart
            trigger_type=PopoverTriggerType::Click
            popover_controller
            content_scroll_container_ref=content_scroll_container
        >
            <PopoverTrigger slot>
                <div
                    id=id.get().map(|id| format!("{id}-trigger"))
                    tabindex="0" // Make this element tab-reachable
                    node_ref=tag_picker_ref
                    class=class_list!(SELECT_CLASSES, "cursor-default flex justify-between items-center")
                    on:keydown=move |key: KeyboardEvent| {
                        debug_log!("keypress on popover-div: {}", key.code().as_str());
                        if key.code() == "Enter" || key.code() == "Space" {
                            open_popover.notify();
                        }
                    }
                >
                    <div class="flex gap-2 overflow-scroll">
                        // Selected tags
                        <For
                            each=move || inside_selected.get()
                            key=|tag| tag.clone()
                            let:tag
                            >
                            <div class="p-1.5 bg-oa-gray dark:bg-gray-800 rounded-lg flex items-center gap-1.5">
                                <span>{tag.as_ref().to_string()}</span>
                                <div class="p-1 hover:bg-oa-gray-mid hover:dark:bg-gray-600 hover:cursor-pointer rounded" on:click=move |ev| {
                                    ev.stop_propagation();
                                    let tag = tag.clone();
                                    let checkboxes = checkboxes.get();
                                    let Some(checked) = checkboxes.get(&tag) else {
                                        return ;
                                    };
                                    toggle_tag(inside_selected, tag, *checked).invoke(());
                                }>
                                    <Icon class="w-3 h-3" icon=crate::icon::CloseIcon() />
                                </div>
                            </div>
                        </For>
                        // Placeholder, shown when empty
                        <div class=class_list!("p-1.5", PLACEHOLDER_TEXT_CLASS)>
                        {move || {
                            if inside_selected.get().is_empty() {
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
            <PopoverHeader slot>
                // Search Inputbox
                <TextInput
                    id=id.get().map(|id| format!("{id}-search"))
                    class="mb-2"
                    placeholder="Search..."
                    value=search_filter
                    input_ref=search_ref
                    on:keydown=move |key: KeyboardEvent| {
                        debug_log!("keypress in popover-search: {}", key.code().as_str());
                        if key.code() == "Escape" || key.code() == "Tab" {
                            close_popover.notify();
                            let Some(tag_picker_ref): Option<HtmlDivElement> = tag_picker_ref.get() else {
                                error!("tag_picker_ref is None");
                                return;
                            };
                            tag_picker_ref.focus().expect("Tag_picker should be focus-able.");
                        } else if key.code() == "Enter" {
                            let Some(tag) = tags_grouped.get().get(focus_ith.get()).cloned() else {
                                return;
                            };
                            let checkboxes = checkboxes.get();
                            let Some(checked) = checkboxes.get(&tag) else {
                                return ;
                            };

                            toggle_tag(inside_selected, tag, *checked).invoke(());
                        } else if key.code() == "ArrowUp" {
                            focus_ith.update(|old_value| *old_value = old_value.saturating_sub(1));
                        } else if key.code() == "ArrowDown" {
                            focus_ith.update(|old_value| *old_value = old_value.saturating_add(1));
                        }
                    }
                    {..}
                    role="combobox" // Makes vimium like plugins pass special keys through
                />
            </PopoverHeader>

            // Popover Contents VV
            <div id=id.get().map(|id| format!("{id}-content")) class="max-h-full">
                // Tags
                <For
                    each=move || tags_grouped.get().into_iter().enumerate()
                    key=move |tag_indexed| {
                        let checkboxes = checkboxes.get();
                        if let Some(checkbox) = checkboxes.get(&tag_indexed.1) {
                            (tag_indexed.clone(), checkbox.get())
                        } else {
                            warn!("Checkbox signal not found in tag_picker ");
                            (tag_indexed.clone(), false)
                        }
                    }
                    children=move |(i, tag)| {
                        let checkboxes = checkboxes.get();
                        let Some(checked) = checkboxes.get(&tag) else {
                            return ().into_any()
                        };

                        debug_log!("Creating tag item");
                        let div_ref: NodeRef<Div> = NodeRef::new();

                        // Handle scroll for the focus-ring so it stays in-view.
                        Effect::new(move || {
                            if i != focus_ith.get() {
                                return;
                            }

                            if let Some(content_scroll_container) = content_scroll_container.get() &&
                                let Some(div) = div_ref.get() {
                                scroll_into_view_smart(&content_scroll_container, &div);
                            }
                        });

                        view! {
                            <div
                                class=class_list!(
                                    TAG_LIST_ITEM_CLASSES,
                                    ("outline outline-black dark:outline-white outline-2", move || i == focus_ith.get())
                                )
                                node_ref=div_ref
                                on:click={
                                    let tag = tag.clone();
                                    toggle_tag(inside_selected, tag, *checked)
                                }
                            >
                                {let tag=tag.clone(); {
                                    view! {
                                        <Checkbox disable_tab=true checked=*checked controlled=true>
                                            {tag.as_ref().to_string()}
                                        </Checkbox>
                                    }
                                }}

                            </div>
                        }.into_any()
                    }
                    >
                </For>
            </div>
        </Popover>
    }
}

pub enum ElementOccludedBy {
    Top,
    Bottom,
}

/// Only scrolls when *elem* is occluded by *container*.
/// Will scroll minimally to no longer occlude *elem*.
pub fn scroll_into_view_smart(container: &Element, elem: &Element) {
    match get_element_occlusion(container, elem) {
        Some(ElementOccludedBy::Top) => {
            let options = ScrollIntoViewOptions::new();
            options.set_container(ScrollIntoViewContainer::Nearest);
            options.set_inline(ScrollLogicalPosition::Start);
            elem.scroll_into_view_with_scroll_into_view_options(&options);
        }
        Some(ElementOccludedBy::Bottom) => {
            let options = ScrollIntoViewOptions::new();
            options.set_container(ScrollIntoViewContainer::Nearest);
            options.set_inline(ScrollLogicalPosition::End);
            elem.scroll_into_view_with_scroll_into_view_options(&options);
        }
        // Nothing to do
        None => {}
    };
}

/// Returns by which part of *container* that *elem* is occluded by.
/// Or `None` when *elem* is not occluded;
pub fn get_element_occlusion(container: &Element, elem: &Element) -> Option<ElementOccludedBy> {
    let container_rect = container.get_bounding_client_rect();
    let elem_rect = elem.get_bounding_client_rect();

    let element_top = elem_rect.top() - container_rect.top();
    if element_top < 0.0 {
        return Some(ElementOccludedBy::Top);
    }

    let element_bottom = element_top + elem_rect.height();
    let container_height = container_rect.height();
    if element_bottom > container_height {
        return Some(ElementOccludedBy::Bottom);
    }

    None
}

fn toggle_tag<T, Event>(
    inside_selected: RwSignal<Vec<T>>,
    tag: T,
    checked: RwSignal<bool>,
) -> impl FnMut(Event) + 'static
where
    T: AsRef<str> + Eq + Clone + Send + Sync + 'static,
{
    move |_| {
        // Toggle selection
        inside_selected.update(|old_sel| {
            debug_log!("Toggling {}", tag.as_ref());
            if old_sel.contains(&tag) {
                if let Some(pos) = old_sel.iter().position(|sel_tag| sel_tag == &tag) {
                    old_sel.remove(pos);
                }
            } else {
                old_sel.push(tag.clone());
            }
        });

        checked.update(|old| {
            debug_log!("Flipping chekcbox to {}", !*old);
            *old = !*old
        });
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
