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
use crate::class_list;
use crate::form_input::FormInputContext;
use crate::form_input::PostfixLabelStyle;
use crate::form_input::resolved_label_id;
use crate::form_input::take_provided_label_id;
use crate::util::shared_id::shared_id;
use leptodon_proc_macros::generate_docs;
use leptos::prelude::BindAttribute;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::RwSignal;
use leptos::prelude::use_context;
use leptos::reactive::wrappers::read::Signal;
use leptos::{IntoView, component, prelude::MaybeProp, view};
use leptos_use_math::use_or;

const TOGGLE_CLASS: &str = "inline-block relative w-11 h-6 bg-gray-200 rounded-full
peer peer-focus:ring-4 peer-focus:ring-oa-blue dark:peer-focus:ring-oa-blue
dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white
after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5
after:transition-all dark:border-gray-600 peer-checked:bg-oa-blue";

#[generate_docs]
/// Similar to checkbox, but in a physical-switch-like design.
/// Often used for persistent settings to give the feel of a physical switch.
#[component]
pub fn Toggle(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether this toggle needs to be ON for a form to be submitted.
    #[prop(optional)]
    required: bool,
    /// Checked state
    #[prop(optional, into)]
    checked: RwSignal<bool>,
    /// Labels the toggle, only phrasing_content allowed.
    children: Children,
) -> impl IntoView {
    // Form context
    let form_context = use_context::<FormInputContext<String>>();
    let form_required = form_context
        .clone()
        .map(|ctx| ctx.required)
        .unwrap_or_default();
    let required = use_or(required, form_required);

    let fallback = shared_id();
    let label_id = take_provided_label_id();
    let id: Signal<String> = resolved_label_id(id, label_id, "toggle", fallback.clone());

    view! {
        <label for=id class=class_list!("relative inline-flex items-center mb-4 cursor-pointer", class)>
            <input
                id=id
                name=name
                type="checkbox"
                required=required
                bind:checked=checked
                class="sr-only peer"
            />

            <span
                id=move || format!("{}-switch", id.get())
                role="switch"
                class=TOGGLE_CLASS
            />
            <PostfixLabelStyle required=required.get() children />
        </label>
    }
}
