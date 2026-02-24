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
use leptos::IntoView;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::view;
use leptos::{component, prelude::Children};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

/// A flex column which centers its items.
#[component]
pub fn CenteringColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!("flex flex-col justify-center items-center", class)>
            {children()}
        </div>
    }
}

/// Column with a fixed width
#[component]
pub fn FixedColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!("flex flex-col w-full lg:w-[770px] xl:w-[1024px]", class)>
            {children()}
        </div>
    }
}

/// A fixed column but centered.
#[component]
pub fn FixedCenterColumn(
    #[prop(optional, into)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <CenteringColumn class>
            <FixedColumn>
                {children()}
            </FixedColumn>
        </CenteringColumn>
    }
}
