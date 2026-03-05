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
use derive_more::Display;
use leptodon::button::Button;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::paragraph::Paragraph;
use leptodon::tag_picker::TagPicker;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[generate_codeblock(TagPickerExample)]
#[component]
pub fn TagPickerDemo() -> impl IntoView {
    #[derive(EnumIter, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    pub enum Element {
        Hydrogen,
        Helium,
        Lithium,
    }
    impl AsRef<str> for Element {
        fn as_ref(&self) -> &'static str {
            match self {
                Element::Hydrogen => "hydrogen",
                Element::Helium => "helium",
                Element::Lithium => "lithium",
            }
        }
    }

    let elements = RwSignal::new(Element::iter().collect::<Vec<_>>());
    let selected = RwSignal::new(vec![]);
    view! {
        <Paragraph>
            "Selected: "
            {move || selected.get()
                .iter()
                .map(|tag: &Element| {
                    view! {
                        {tag.to_string()},
                    }
                })
                .collect_view()}
        </Paragraph>
        <TagPicker
            selected=selected
            tags=elements
        />
        <Button on_click=move |_e| {
            selected.set(vec![Element::Lithium]);
        }>"Set Lithium as Selected"</Button>
    }
}

#[component]
pub fn TagPickerDemoPage() -> impl IntoView {
    view! {
        <Title text="Tag Picker"/>

        <FixedCenterColumn>
            <Heading4 anchor="tag-picker">"Tag Picker"</Heading4>
            <TagPickerExample />

            <leptodon::tag_picker::TagPickerDocs />
        </FixedCenterColumn>
    }
}
