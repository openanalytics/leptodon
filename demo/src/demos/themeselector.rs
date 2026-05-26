use leptodon::alert::Alert;
use leptodon::alert::AlertTheme;
use leptodon::codeblock::Codeblock;
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
use leptodon::darkmode::ThemeSelector;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::paragraph::Paragraph;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(ThemeSelectorExample)]
#[component]
pub fn ThemeSelectorDemo() -> impl IntoView {
    view! {
        <Paragraph>"If you need mutliple theme-selectors on a single page do the following at the top of your App:"</Paragraph>
        <Codeblock code=r#"
    let color_scheme = use_color_scheme();

    view! {
        <MetaColorScheme color_scheme />
        // ...
    }
"#/>
        <Alert theme=AlertTheme::Danger>
            "Make sure to NOT put this in an SSR route-shell, the color_scheme context needs to be present both on hydrated client and server rendered side."
        </Alert>

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
