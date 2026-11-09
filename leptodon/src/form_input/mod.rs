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
use leptos::context::Provider;
use leptos::control_flow::ShowLet;
use leptos::prelude::ElementChild;
use leptos::prelude::{ClassAttribute, Get, IntoAny, MaybeProp, RwSignal, Show};
use leptos::reactive::owner::{StoredValue, take_context};
use leptos::reactive::traits::GetValue;
use leptos::reactive::wrappers::read::Signal;
use leptos::{IntoView, component, prelude::Children, view};
use std::marker::PhantomData;

use crate::class_list;
use crate::util::shared_id::shared_id;

#[derive(Clone)]
pub struct FormInputContext<E: Clone + Send + Sync + std::fmt::Display + 'static> {
    pub required: bool,
    pub label: MaybeProp<String>,
    pub feedback: RwSignal<Option<E>>,
}

#[generate_docs]
/// Use this to add labels, required-indicator and automatic feedback to form inputs (crate::input, crate::select, crate::radio).
#[component]
pub fn FormInput<E>(
    #[prop(into)] label: String,
    /// Whether to have a default amount of spacing below the form-input.
    #[prop(default = true)]
    default_spacing: bool,
    required: bool,
    children: Children,
    #[prop(default = PhantomData)] _phantom: PhantomData<E>,
) -> impl IntoView
where
    E: Clone + Send + Sync + std::fmt::Display + 'static,
{
    view! {
        <MaybeLabelledFormInput<E> label default_spacing required children />
    }
}

// Internal function: when an input field like password need to display an eye next to itself, the GenericInput can no longer simply display feedback under itself.
// For that reason the password input wraps itself internally with the FormInput mechanism even when unlabeled.
#[component]
pub(crate) fn MaybeLabelledFormInput<E>(
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Whether to have a default amount of spacing below the form-input.
    #[prop(default = true)]
    default_spacing: bool,
    required: bool,
    children: Children,
    #[prop(default = PhantomData)] _phantom: PhantomData<E>,
) -> impl IntoView
where
    E: Clone + Send + Sync + std::fmt::Display + 'static,
{
    let feedback: RwSignal<Option<E>> = RwSignal::new(None);
    let form_ctx = FormInputContext {
        required,
        label,
        feedback,
    };
    view! {
        <div class=class_list![("mb-2", default_spacing), "flex flex-col"]>
            <Label label required>
                <Provider<_, _> value=form_ctx>
                    {children()}
                </Provider<_, _>>
            </Label>

            // Feedback
            <ShowLet
                some=move || feedback.get()
                fallback=|| ()
                let:feedback
            >
                <span class="text-red-500">{
                    move || feedback.to_string()
                }</span>
            </ShowLet>
        </div>
    }
}

/// A label component, shown above its children.
#[component]
pub fn Label(
    #[prop(optional, into)]
    /// When a label is given, this used in <label for=propagated_id ..> and passed set on the first child leptodon-input.
    propagated_id: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Displays a red star in front of the label to indicate an input is required to be completed.
    required: bool,
    /// The labeled element
    children: Children,
) -> impl IntoView {
    if let Some(label) = label.get() {
        let label_id_str: String = propagated_id.get().unwrap_or(shared_id().to_string());
        let label_id = LabelId(label_id_str.clone());

        // debug_log!("Suspicious label_id: {label_id:?}");
        view! {
            <label
                for=label_id_str
                class="block"
            >
                <span class="text-sm font-semibold text-heading text-gray-900 dark:text-gray-100">
                    <RequiredStar required/>
                    {label}
                </span>
            </label>
            <Provider value=label_id>
                {children()}
            </Provider>
        }
        .into_any()
    } else {
        view! { {children()} }.into_any()
    }
}

/// Style helper
/// Used for inline labels behind checkbox and toggle.
/// Provide the label via *children*
#[component]
pub fn PostfixLabelStyle(required: bool, children: Children) -> impl IntoView {
    view! {
        <span class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-100"><RequiredStar required=required/>{children()}</span>
    }
}

#[component]
pub fn RequiredStar(required: bool) -> impl IntoView {
    view! {
        <Show
            when=move || required
            fallback=|| ()
        >
            <span class="text-red-500">*</span>
        </Show>
    }
}

#[derive(Clone)]
pub struct LabelId(pub String);

/// Resolves label id via the explicitely provided id or LabelId context.
pub fn opt_resolve_label_id(explicit: Option<String>, label_id: Option<String>) -> Option<String> {
    explicit.or(label_id)
}

pub fn take_provided_label_id() -> StoredValue<Option<String>> {
    StoredValue::new(take_context::<LabelId>().map(|l| l.0))
}

pub fn resolved_label_id(
    explicit: MaybeProp<String>,
    label_id: StoredValue<Option<String>>,
    prefix: &'static str,
    shared_id: String,
) -> Signal<String> {
    Signal::derive(move || {
        opt_resolve_label_id(explicit.get(), label_id.get_value())
            .unwrap_or_else(|| format!("{prefix}-{shared_id}"))
    })
}
