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
use std::sync::Arc;

use leptodon_proc_macros::generate_docs;
use leptos::logging::warn;
use leptos::prelude::ChildrenFn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::IntoAny;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::TypedChildren;
use leptos::prelude::Update;
use leptos::prelude::provide_context;
use leptos::prelude::use_context;
use leptos::{IntoView, component, view};

use crate::class_list;
use crate::util::shared_id::shared_id;

#[derive(Clone)]
struct TabContext {
    active: RwSignal<(String, ChildrenFn)>,
    first_tab: RwSignal<bool>,
}

#[generate_docs]
#[component]
pub fn Tabs<T>(
    children: TypedChildren<T>,
    #[prop(default = true)] default_spacing: bool,
) -> impl IntoView
where
    T: IntoView + Send + 'static,
{
    let selected_tab: RwSignal<(String, ChildrenFn)> =
        RwSignal::new(("1".to_string(), Arc::new(|| ().into_any())));

    provide_context(TabContext {
        active: selected_tab,
        first_tab: RwSignal::new(true),
    });

    view! {
        <div class=class_list!(
            "text-sm font-medium text-center text-body border-b border-default",
            ("mb-4", move || default_spacing)
        )>
            <ul class="flex flex-wrap -mb-px">
                {children.into_inner()()}
            </ul>
        </div>
        {move || (selected_tab.get().1)()}
    }
}

const TAB_CLASS: &str = "inline-block p-4 border-b rounded-t-base";
const INACTIVE_TAB_CLASS: &str = "border-transparent hover:text-oa-blue hover:border-oa-blue";
const ACTIVE_TAB_CLASS: &str = "text-oa-blue border-oa-blue active";

#[generate_docs]
#[component]
pub fn Tab(
    #[prop(into)] title: String,
    #[prop(optional)] default: bool,
    children: ChildrenFn,
) -> impl IntoView {
    let tab_ctx = use_context::<TabContext>();
    match tab_ctx {
        None => {
            warn!(
                "Tab used outside Tabs component!\n{}",
                std::backtrace::Backtrace::capture()
            );
            view! {"Tab Error, see console"}.into_any()
        }
        Some(tab_ctx) => {
            let first = tab_ctx.first_tab.get_untracked();
            if first {
                tab_ctx.first_tab.set(false);
            }

            let id = shared_id();
            let id2 = id.clone();
            let id3 = id.clone();

            if default || first {
                let children = children.clone();
                let id = id2.clone();

                tab_ctx.active.update(move |(value, children_fn)| {
                    *value = id.to_string();
                    *children_fn = children.clone();
                });
            };

            view! {
                <li class="me-2">
                    <a
                        href="#"
                        on:click=move |e| {
                            let children = children.clone();
                            let id = id2.clone();
                            tab_ctx.active.update(move |(value, children_fn)| {
                                *value = id.to_string();
                                *children_fn = children.clone();
                            });
                            e.prevent_default();
                        }
                        class=class_list!(
                            TAB_CLASS,
                            (ACTIVE_TAB_CLASS, move || id.to_string() == tab_ctx.active.get().0),
                            (INACTIVE_TAB_CLASS, move || id3 != tab_ctx.active.get().0)
                        )
                    >{title}</a>
                </li>
            }
            .into_any()
        }
    }
}
