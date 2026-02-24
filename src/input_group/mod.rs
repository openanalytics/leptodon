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
use leptos::logging::log;
use leptos::prelude::Children;
use leptos::prelude::ChildrenFragment;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::IntoAny;
use leptos::prelude::MaybeProp;
use leptos::prelude::provide_context;
use leptos::{IntoView, component, view};

mod presets;

// Re-exports
pub use crate::input_group::presets::ControlledNumberInput;

const OA_READONLY_INPUT_CLASSES: &str = "border-0 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";
const OA_INPUT_CLASSES: &str = "shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";

#[derive(Clone)]
pub struct GroupItemClassContext {
    pub class: String,
}

/// Leptos will spread attributes on its own to each top level child.
/// This may be useful when you want to style children.
#[component]
pub fn GroupItemContextProvider(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    if let Some(class) = class.get() {
        log!("Providing the {class} context");
        provide_context::<GroupItemClassContext>(GroupItemClassContext { class });
    }
    view! {
        {children()}
    }
}

#[component]
pub fn InputGroup(children: ChildrenFragment) -> impl IntoView {
    let nodes = children().nodes;
    let nb_children = nodes.iter().len();
    let last_idx = nb_children - 1;
    let children = match nb_children {
        0..1 => nodes
            .into_iter()
            .map(|child| view!({ child }))
            .collect::<Vec<_>>(),
        _n => nodes
            .into_iter()
            .enumerate()
            .map(|(idx, child)| {
                match idx {
                    0 => {
                        view! {
                            // Spread round left corners on first elem
                            <GroupItemContextProvider class="rounded-none rounded-l-lg">{child}</GroupItemContextProvider>
                        }
                    }
                    idx if idx == last_idx => {
                        view! {
                            // Spread round right corners on last elem
                            <GroupItemContextProvider class="rounded-none rounded-r-lg">{child}</GroupItemContextProvider>
                        }
                    }
                    _idx => {
                        view! {
                            // Spread round nothing, remove x borders on inner elems
                            <GroupItemContextProvider class="rounded-none border-x-none m-0">{child}</GroupItemContextProvider>
                        }
                    }
                }
                .into_any()
            })
            .collect::<Vec<_>>(),
    };

    view! {
        <div class="relative flex items-center mb-2">
            {children}
        </div>
    }
}
