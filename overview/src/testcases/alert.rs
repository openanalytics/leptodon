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
use leptodon::alert::Alert;
use leptodon::button::Button;
use leptodon::util::callback::ArcOneCallback;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn TestAlerts() -> impl IntoView {
    let dismissed = RwSignal::new(false);
    let alert_text = RwSignal::new("test-string");
    view! {
        <Title text="Test Alerts"/>
        <p data-testid="dismissed-status">
            {move || dismissed.get().to_string()}
        </p>
        <Alert
            theme=leptodon::alert::AlertTheme::Success
            prefix="prefix"
            border=true
            dismissable=true
            default_spacing=false
            on_dismiss=ArcOneCallback::new(move |_| {
                dismissed.set(true);
            })
            attr:data-testid="test-alert"
        >
            {alert_text}
        </Alert>
        <br/>
        <Button on_click=move |_| {
            alert_text.set("🐈");
        } attr:data-testid="btn-change-content">
            "Change Content"
        </Button>
    }
}
