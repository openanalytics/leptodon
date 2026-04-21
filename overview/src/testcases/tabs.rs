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
use leptodon::tabs::Tab;
use leptodon::tabs::Tabs;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn TestTabs() -> impl IntoView {
    view! {
        <Title text="Test Tabs"/>
        <Tabs>
            <Tab title="Profile" default=true>
                <div id="profile-content">
                    "profile content"
                </div>
            </Tab>
            <Tab title="Settings">
                <div id="settings-content">
                    "settings content"
                </div>
            </Tab>
        </Tabs>
    }
}
