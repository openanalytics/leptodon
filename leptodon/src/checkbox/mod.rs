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
use leptos::prelude::BindAttribute;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::use_context;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
use leptos_use_math::use_or;

use crate::class_list;
use crate::form_input::FormInputContext;
use crate::form_input::PostfixLabelStyle;
use crate::form_input::resolved_label_id;
use crate::form_input::take_provided_label_id;
use crate::util::shared_id::shared_id;

const CHECKBOX_CLASS: &str = "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600";

#[generate_docs]
/// A simple checkmark-box with optional label on the right.
#[component]
pub fn Checkbox(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Checked state
    #[prop(optional)]
    checked: RwSignal<bool>,
    /// Required to be checked on for form submission.
    #[prop(optional)]
    required: bool,
    /// Whether or not this element is unreachable by tabbing.
    #[prop(optional, into)]
    disable_tab: bool,
    /// Stops label click handling and FormInputContext control.
    #[prop(optional)]
    controlled: bool,
    /// Label goes here.
    children: Children,
) -> impl IntoView {
    // Form context
    let form_context = use_context::<FormInputContext<String>>();
    let form_required = Signal::from(
        form_context
            .clone()
            .map(|ctx| ctx.required)
            .unwrap_or_default(),
    );

    let required = if controlled {
        Signal::derive(move || required)
    } else {
        use_or(required, form_required)
    };

    #[cfg(debug_assertions)]
    leptos::prelude::Effect::watch(
        move || checked.get(),
        |new, _, _| {
            leptos::logging::debug_log!("Checkbox checked state changed to {new}");
        },
        false,
    );

    let fallback = shared_id();
    let label_id = take_provided_label_id();
    let id: Signal<String> = resolved_label_id(id, label_id, "checkbox", fallback.clone());

    view! {
        <div class=class_list!["relative inline-flex items-center cursor-pointer", class]
            on:click={
                move |ev| {
                    if controlled {
                        ev.prevent_default();
                    }
                }
            }
        >
            <input
                id=move || id.get()
                name=name.get()
                type="checkbox"
                bind:checked=checked
                class=CHECKBOX_CLASS
                required=required
                // Non integer values should make tabbing reset to the default behaviour.
                tabindex=move || if disable_tab { "-1" } else { "auto" }
            />
            <label for=move || id.get()>
                <PostfixLabelStyle required=required.get() children />
            </label>
        </div>
    }
}
