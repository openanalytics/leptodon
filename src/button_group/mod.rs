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
use crate::input_group::GroupItemClassContext;
use attr_docgen::generate_docs;
use leptos::context::Provider;
use leptos::prelude::ChildrenFn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::slot;
use leptos::{IntoView, component, prelude::Children, view};

mod variations;
pub use crate::button_group::variations::*;

#[derive(Clone)]
pub struct InGroupContext {
    pub in_group: bool,
}

#[slot]
pub struct First {
    children: ChildrenFn,
}

#[slot]
pub struct Last {
    children: ChildrenFn,
}

#[generate_docs]
/// Blocked on passing context to first and last child.
#[component]
pub fn ButtonGroup(
    first: First,
    #[prop(default=Box::new(move || { ().into_any() }))] children: Children,
    last: Last,
) -> impl IntoView {
    view! {
        <div class="inline-flex rounded-lg shadow-sm -space-x-px" role="group">
            <Provider<InGroupContext, _> value=InGroupContext { in_group: true }>
                <Provider<GroupItemClassContext, _> value=GroupItemClassContext{ class: "rounded-l-lg".to_string() }>
                    {(first.children)().into_any()}
                </Provider<GroupItemClassContext, _>>
                {children()}
                <Provider<GroupItemClassContext, _> value=GroupItemClassContext{ class: "!border-r rounded-r-lg".to_string() }>
                    {(last.children)().into_any()}
                </Provider<GroupItemClassContext, _>>
            </Provider<InGroupContext, _>>
        </div>
    }
}
