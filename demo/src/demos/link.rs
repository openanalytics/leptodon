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
use attr_docgen::generate_codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::link::Link;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(LinkExample)]
#[component]
pub fn LinkDemo() -> impl IntoView {
    view! {
        "Explore more about OA on the "
        <Link href="https://openanalytics.eu" target="_blank">OA website</Link>
    }
}

#[component]
pub fn LinkDemoPage() -> impl IntoView {
    view! {
        <Title text="Link"/>

        <FixedCenterColumn>
            <Heading4 anchor="link">"Link"</Heading4>
            <LinkExample />

            <leptodon::link::LinkDocs />
        </FixedCenterColumn>
    }
}
