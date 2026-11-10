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
use leptodon::form_input::FormInput;
use leptodon::textarea::TextArea;
use leptos::prelude::ElementChild;
use leptos::reactive::traits::Get;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[component]
pub fn TestTextArea() -> impl IntoView {
    let value = RwSignal::new("text-area-text".to_string());

    view! {
        <Title text="Test TextArea"/>
        <p>
            {move || value.get()}
        </p>
        <FormInput<String> required=true label="A box for text">
            <TextArea placeholder="Test paste here" value />
        </FormInput<String>>

        <TextArea
            id="labeled-textarea"
            label="A second box for text"
            required=false
            placeholder="Test paste here too"
            value
        />
    }
}
