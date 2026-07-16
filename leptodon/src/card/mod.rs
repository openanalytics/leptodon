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
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::reactive::traits::Get;
use leptos::reactive::wrappers::read::Signal;
use leptos::{IntoView, children::Children, component, view};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="max-w-md mx-auto py-2">
            <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden transition-colors">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CardSection(
    #[prop(into, optional)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list![class, "px-5 py-3 space-y-3"]>
            {children()}
        </div>
    }
}

#[component]
pub fn CardField(#[prop(into)] name: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div>
            <span class="text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">{move || name.get()}</span>
            {children()}
        </div>
    }
}
