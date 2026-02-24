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
use leptodon::button::Button;
use leptodon::button::CopyButton;
use leptodon::textarea::TextArea;
use leptos::prelude::ElementChild;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[component]
pub fn TestCopyButton() -> impl IntoView {
    let to_copy = RwSignal::new("test_string1".to_string());

    view! {
        <Title text="Test CopyButton"/>
        <p>
            {to_copy}
        </p>
        <CopyButton id="copy-button" to_copy=to_copy />
        <Button id="set-test-string1" on_click=move |_e| {
            to_copy.set("test_string1".to_string());
        }>"test_string1"</Button>
        <Button id="set-test-string2" on_click=move |_e| {
            to_copy.set("test_string2".to_string());
        }>"test_string2"</Button>
        <TextArea placeholder="Test paste here" value=RwSignal::new(String::new())/>
    }
}
