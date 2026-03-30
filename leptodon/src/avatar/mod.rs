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
use leptos::prelude::ElementChild;
use leptos::prelude::{IntoAny, IntoOptionGetter, MaybeProp};
use leptos::{IntoView, component, prelude::ClassAttribute, view};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

#[generate_docs]
/// Circular avatar component.
#[component]
pub fn Avatar(
    /// Image source for the avatar.
    #[prop(optional, into)]
    src: MaybeProp<String>,
    /// Extra classes to style this component.
    #[prop(optional, into)]
    class: MaybeReactiveClass,
) -> impl IntoView {
    if let Some(src) = src.into_option_getter().run() {
        view! {
            <img class=class_list!["w-10 h-10 rounded-full", class] src=src alt="Rounded avatar"/>
        }
        .into_any()
    } else {
        view! {
            <div class="relative w-10 h-10 overflow-hidden bg-neutral-secondary-medium rounded-full">
                <svg class="absolute w-12 h-12 text-body-subtle -left-1" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd"></path></svg>
            </div>
        }.into_any()
    }
}
