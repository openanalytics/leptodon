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
use leptodon::darkmode::ThemeSelector;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(ThemeSelectorExample)]
#[component]
pub fn ThemeSelectorDemo() -> impl IntoView {
    view! {
        <p>"Using multiple theme-selectors on the same page (as done here) does not work correctly.\nIf you need this behaviour, please open an issue."</p>
        <ThemeSelector/>
    }
}

#[component]
pub fn ThemeSelectorDemoPage() -> impl IntoView {
    view! {
        <Title text="ThemeSelector"/>

        <FixedCenterColumn>
            <Heading4 anchor="themeselector">"ThemeSelector"</Heading4>
            <ThemeSelectorExample />
        </FixedCenterColumn>
    }
}
