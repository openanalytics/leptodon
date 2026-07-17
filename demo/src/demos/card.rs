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
use leptodon::card::Card;
use leptodon::card::CardField;
use leptodon::card::CardSection;
use leptodon::divider::HorizontalLine;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(CardExample)]
#[component]
pub fn CardDemo() -> impl IntoView {
    view! {
        <Card>
            <CardSection>
                <Heading4>"Profile"</Heading4>
                <CardField name="ID">
                    <p>"6205c4ef-8c15-4977-bfeb-5d69cf9cb156"</p>
                </CardField>
                <CardField name="Name">
                    <p>"Someone"</p>
                </CardField>
                <CardField name="Email address">
                    <p>"someone@example.com"</p>
                </CardField>
                <CardField name="Location">
                    <p>"Somewhere"</p>
                </CardField>
            </CardSection>
            <HorizontalLine default_spacing=false />
            <CardSection>
                <CardField name="Created at: ">
                    "2026-07-16"
                </CardField>
            </CardSection>
        </Card>
    }
}

#[component]
pub fn CardDemoPage() -> impl IntoView {
    view! {
        <Title text="Card Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="card">"Card"</Heading4>
            <CardExample />

            <leptodon::card::CardDocs />
            <leptodon::card::CardSectionDocs />
            <leptodon::card::CardFieldDocs />
        </FixedCenterColumn>
    }
}
