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
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::icon;
use crate::icon::Icon;
use leptodon_proc_macros::generate_docs;
use leptos::IntoView;
use leptos::children::Children;
use leptos::component;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;

#[generate_docs]
#[component]
pub fn InfoLabel(
    #[prop(into, optional)] class: MaybeReactiveClass,
    // Content of the InfoLabel, shown inside the tooltip.
    children: Children,
) -> impl IntoView {
    view! {
        <button class=class_list!(class, "info-label-anchor align-[center]")>
            <Icon class="h-4 w-4" icon=icon::InfoIcon() />
            <div class="info-label m-1 p-2 rounded-lg">
                {children()}
            </div>
        </button>
    }
}
