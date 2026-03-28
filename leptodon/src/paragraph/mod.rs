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
use leptos::IntoView;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use leptos::{component, prelude::Children};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

const PARAGRAPH_CLASS: &str = "text-gray-900 dark:text-gray-100";
const PARAGRAPH_SPACING_CLASS: &str = "mb-1";

/// `<p>` replacement component, mainly to add a default amount of padding.
#[generate_docs]
#[component]
pub fn Paragraph(
    /// Extra paragraph classes
    #[prop(optional, into)]
    class: MaybeReactiveClass,
    /// Whether to have a default amount of spacing around the header.
    #[prop(default = true)]
    default_spacing: bool,
    /// Heading text
    children: Children,
) -> impl IntoView {
    view! {
        <p class=class_list!(PARAGRAPH_CLASS, (PARAGRAPH_SPACING_CLASS, default_spacing), class)>
            {children()}
        </p>
    }
}
