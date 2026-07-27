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
use leptodon::heading::Heading4;
use leptodon::infolabel::InfoLabel;
use leptodon::layout::FixedCenterColumn;
use leptodon::util::lorem::Lorem;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(InfoLabelExample)]
#[component]
pub fn InfoLabelDemo() -> impl IntoView {
    view! {
        "https://openanalytics.eu/ "<InfoLabel>"OA website. "<Lorem sentences=5 /></InfoLabel>
    }
}

#[component]
pub fn InfoLabelDemoPage() -> impl IntoView {
    view! {
        <Title text="InfoLabel"/>

        <FixedCenterColumn>
            <Heading4 anchor="info-label">"InfoLabel"</Heading4>
            <InfoLabelExample />

            <leptodon::infolabel::InfoLabelDocs />
        </FixedCenterColumn>
    }
}
