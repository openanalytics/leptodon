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
use leptodon::divider::HorizontalLine;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(DividerExample)]
#[component]
pub fn DividerDemo() -> impl IntoView {
    view! {
        <p>
            "Section1"
        </p>
        <HorizontalLine />
        <p>
            "Section2"
        </p>
    }
}

#[component]
pub fn DividerDemoPage() -> impl IntoView {
    view! {
        <Title text="Divider"/>

        <FixedCenterColumn>
            <Heading4 anchor="divider">"Divider"</Heading4>
            <DividerExample />
        </FixedCenterColumn>
    }
}
