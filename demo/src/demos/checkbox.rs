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
use leptodon::button::Button;
use leptodon::checkbox::Checkbox;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(CheckboxExample)]
#[component]
pub fn CheckboxDemo() -> impl IntoView {
    let checked = RwSignal::new(true);
    view! {
        <p>
            {move || checked.get().to_string()}
        </p>
        <Checkbox
            class="my-3"
            checked=checked
        >
            Test Label
        </Checkbox>
        <br/>
        <Button on_click=move |_| {
            checked.set(true);
        }>
            On
        </Button>
        <Button on_click=move |_| {
            checked.set(false);
        }>
        Off
        </Button>
    }
}

#[component]
pub fn CheckboxDemoPage() -> impl IntoView {
    view! {
        <Title text="Checkbox"/>

        <FixedCenterColumn>
            <Heading4 anchor="checkbox">"Checkbox"</Heading4>
            <CheckboxExample />

            <leptodon::checkbox::CheckboxDocs />
        </FixedCenterColumn>
    }
}
