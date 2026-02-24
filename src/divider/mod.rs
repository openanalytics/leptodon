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
use attr_docgen::generate_docs;
use leptos::IntoView;
use leptos::component;
use leptos::prelude::ClassAttribute;
use leptos::view;

const HEADER_CLASS: &str = "font-bold relative color-gray-900";

#[generate_docs]
#[component]
pub fn HorizontalLine() -> impl IntoView {
    view! {
        <hr class="my-4" />
    }
}
