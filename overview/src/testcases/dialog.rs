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
use leptodon::dialog::Dialog;
use leptos::logging::log;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::ElementChild;
use leptos::reactive::signal::RwSignal;
use leptos::reactive::traits::Get;
use leptos::reactive::traits::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn TestDialog() -> impl IntoView {
    let show_dialog = RwSignal::new(false);
    view! {
        <Title text="Test Dialog"/>
        <p>"dialog shown: "{move || { show_dialog.get().to_string() }}</p>
        <Dialog
            attr:data-testid="dialog-backdrop"
            title="Dialog testing title"
            primary_text="PrimaryText"
            secondary_text="SecondaryText"
            visible=show_dialog
            on_click_primary=|| {
                log!("on_click_primary ran");
            }
            on_click_secondary=|| {
                log!("on_click_secondary ran");
            }
            on_close=|_| {
                log!("on_close ran");
            }
            on_close_or_secondary=|_| {
                log!("on_close_or_secondary ran");
            }
        >
            "test-content-in-dialog"
        </Dialog>

        <Button on_click=move |_| {
            show_dialog.set(true);
        }>"Show Dialog"</Button>
    }
}
