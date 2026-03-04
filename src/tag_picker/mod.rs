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
use leptos::leptos_dom::logging::console_log;
use leptos::logging::debug_log;
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
use std::fmt::Display;
use std::hash::Hash;
use std::sync::LazyLock;
use std::sync::Mutex;
use std::time::Duration;
use web_sys::HtmlInputElement;
use web_sys::KeyboardEvent;

use crate::checkbox::Checkbox;
use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::icon::Icon;
use crate::input::PLACEHOLDER_TEXT_CLASS;
use crate::input::TextInput;
use crate::popover::Popover;
use crate::popover::PopoverAnchor;
use crate::popover::PopoverController;
use crate::popover::PopoverTrigger;
use crate::popover::PopoverTriggerType;

const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";
const TAG_LIST_ITEM_CLASSES: &str =
    "hover:bg-oa-gray hover:dark:bg-gray-600 p-2 rounded-lg flex items-center cursor-pointer";

// Nucleo matcher allocates 135kb memory, so we want to reuse this.
// Try to not run multiple fuzzy matchers at the same time.
static NUCLEO_MATCHER: LazyLock<Mutex<Matcher>> =
    LazyLock::new(|| Mutex::new(Matcher::new(Config::DEFAULT)));

#[generate_docs]
#[component]
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
    T: AsRef<str> + Display + Eq + Hash + Clone + Send + Sync + 'static,
{
    let search_filter = RwSignal::new(String::new());
    let search_ref: NodeRef<leptos::html::Input> = NodeRef::new();
    let checkboxes = RwSignal::new(HashMap::<T, RwSignal<bool>>::new());
    let inside_selected = RwSignal::new(selected.get_untracked());

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
                    debug_log!("Added {tag}'s checkbox");
                }
            }
            if let Some(old) = old {
                for tag in old {
                    if new.contains(tag) {
                        // leave it the same
                    } else {
                        // removed tag
                        checkboxes.remove(tag);
                        debug_log!("Removed {tag}'s checkbox");
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
        let sorted_tags = pattern.match_list(tags, &mut matcher);

        sorted_tags.into_iter().map(|(tag, _score)| tag).collect()
    });

    // Partitioned into selected and unselected.
    // (Idx -> (T, is_selected))
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
        all.into_iter().enumerate().collect::<Vec<(usize, T)>>()
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

    let close_popover = Trigger::new();
    let popover_controller = PopoverController {
        close: close_popover,
        on_open: Some(on_popover_open.into()),
        on_close: None,
    };

    view! {
        <Popover id class show_arrow=false preferred_pos=PopoverAnchor::BottomStart trigger_type=PopoverTriggerType::Click popover_controller>
            <PopoverTrigger slot>
                <div
                    id=id.get().map(|id| format!("{id}-trigger"))
                    class=class_list!(SELECT_CLASSES, "cursor-default flex justify-between items-center")
                >
                    <div class="flex gap-2 overflow-scroll">
                        // Selected tags
                        <For
                            each=move || inside_selected.get()
                            key=|tag| tag.clone()
                            let:tag
                            >
                            <div class="p-1.5 bg-oa-gray dark:bg-gray-800 rounded-lg flex items-center gap-1.5">
                                <span>{tag.to_string()}</span>
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

            // Popover Contents VV
            <ul id=id.get().map(|id| format!("{id}-dropdown"))>
                <TextInput class="mb-2" placeholder="Search..." value=search_filter input_ref=search_ref
                    on:keydown=move |key: KeyboardEvent| {
                        console_log(key.code().as_str());
                        if key.code() == "Escape" || key.code() == "Tab" {
                            close_popover.notify();
                        }
                        if key.code() == "Enter" {
                            if search_filter.get().is_empty() {
                                return;
                            }
                            let Some((_, tag)) = tags_grouped.get().first().cloned() else {
                                return;
                            };
                            let checkboxes = checkboxes.get();
                            let Some(checked) = checkboxes.get(&tag) else {
                                return ;
                            };

                            toggle_tag(inside_selected, tag, *checked).invoke(());
                        }
                    }
                    {..}
                    role="combobox" // Makes vimium like plugins pass special keys through
                />
                // Tags
                <For
                    each=move || tags_grouped.get()
                    key=move |tag| {
                        let checkboxes = checkboxes.get();
                        if let Some(checkbox) = checkboxes.get(&tag.1) {
                            (tag.clone(), checkbox.get())
                        } else {
                            warn!("Checkbox signal not found in tag_picker ");
                            (tag.clone(), false)
                        }
                    }
                    children=move |(i, tag)| {
                        let checkboxes = checkboxes.get();
                        let Some(checked) = checkboxes.get(&tag) else {
                            return ().into_any()
                        };

                        debug_log!("Recreating tag item");

                        view! {
                            <li
                                class=class_list!(
                                    TAG_LIST_ITEM_CLASSES,
                                    ("outline outline-black outline-2", move || i == 0 && !search_filter.get().is_empty())
                                )
                                on:click={
                                    let tag = tag.clone();
                                    toggle_tag(inside_selected, tag, *checked)
                                }
                            >
                                {let tag=tag.clone(); {
                                    view! {
                                        <Checkbox disable_tab=true checked=*checked prevent_label=true>
                                            {tag.to_string()}
                                        </Checkbox>
                                    }
                                }}

                            </li>
                        }.into_any()
                    }
                    >
                </For>
            </ul>
        </Popover>
    }
}

fn toggle_tag<T, Event>(
    inside_selected: RwSignal<Vec<T>>,
    tag: T,
    checked: RwSignal<bool>,
) -> impl FnMut(Event) + 'static
where
    T: Display + Eq + Clone + Send + Sync + 'static,
{
    move |_| {
        // Toggle selection
        inside_selected.update(|old_sel| {
            debug_log!("Toggling {}", tag);
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
