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
use crate::class_list::reactive_class::MaybeReactiveClass;
use leptodon_proc_macros::generate_docs;
use leptos::IntoView;
use leptos::component;
use leptos::prelude::ClassAttribute;
use leptos::view;

use crate::class_list;

const HORIZONTAL_LINE_PADDING_CLASS: &str = "my-4";

#[generate_docs]
#[component]
pub fn HorizontalLine(
    /// Whether to have a default amount of spacing above and below the line.
    #[prop(default = true)]
    default_spacing: bool,
    /// Extra line styling, e.g. custom padding
    #[prop(into, optional)]
    class: MaybeReactiveClass,
) -> impl IntoView {
    view! {
        <hr class=class_list![
            class,
            "dark:border-gray-600",
            (HORIZONTAL_LINE_PADDING_CLASS, move || default_spacing)
        ] />
    }
}
