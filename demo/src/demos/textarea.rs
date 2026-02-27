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
use leptodon::heading::Heading6;
use leptodon::layout::FixedCenterColumn;
use leptodon::textarea::TextArea;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::RwSignal;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(TextAreaExample)]
#[component]
pub fn TextAreaDemo() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    view! {
        <Heading6>Synced textareas!</Heading6>
        <TextArea
            label="Magically linked"
            class="mb-2"
            required=true
            placeholder="Enter your magic words."
            value
        />
        <TextArea value />
    }
}

#[component]
pub fn TextAreaDemoPage() -> impl IntoView {
    view! {
        <Title text="TextArea"/>

        <FixedCenterColumn>
            <Heading4 anchor="textarea">"TextArea"</Heading4>
            <TextAreaExample />

            <leptodon::textarea::TextAreaDocs />
        </FixedCenterColumn>
    }
}
