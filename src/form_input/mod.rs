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
use std::marker::PhantomData;

use attr_docgen::generate_docs;
use leptos::context::Provider;
use leptos::prelude::{ClassAttribute, ElementChild, Get, IntoAny, MaybeProp, RwSignal, Show};
use leptos::{IntoView, component, prelude::Children, view};

#[derive(Clone, Copy)]
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
    required: bool,
    children: Children,
    #[prop(default = PhantomData)] _phantom: PhantomData<E>,
) -> impl IntoView
where
    E: Clone + Send + Sync + std::fmt::Display + 'static,
{
    view! {
        <MaybeLabelledFormInput<E> label required children />
    }
}

// Internal function: when an input field like password need to display an eye next to itself, the GenericInput can no longer simply display feedback under itself.
// For that reason the password input wraps itself internally with the FormInput mechanism even when unlabeled.
#[component]
pub(crate) fn MaybeLabelledFormInput<E>(
    #[prop(optional, into)] label: MaybeProp<String>,
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
        <div class="mb-2 flex flex-col">
            <Label label required>
                <Provider<_, _> value=form_ctx>
                    {children()}
                </Provider<_, _>>
            </Label>

            // Feedback
            <Show
                when=move || feedback.get().is_some()
                fallback=|| ()
            >
                <span class="text-red-500">{
                    move || feedback.get().unwrap().to_string()
                }</span>
            </Show>
        </div>
    }
}

/// A label component, shown above its children.
#[component]
pub fn Label(
    #[prop(optional, into)] label: MaybeProp<String>,
    /// Displays a red star to indicate an input is required to be completed.
    required: bool,
    /// The labeled element
    children: Children,
) -> impl IntoView {
    if let Some(label) = label.get() {
        view! {
            <label class="block text-sm font-medium text-heading text-gray-900 dark:text-gray-100">
                <div>
                    <RequiredStar required/>
                    {label}
                </div>
                {children()}
            </label>
        }
        .into_any()
    } else {
        view! { {children()} }.into_any()
    }
}

/// Style helper
/// Used for inline labels behind checkbox and toggle.
/// Provide the label via [children], caller is expected to wrap this and the input in a <label>.
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
