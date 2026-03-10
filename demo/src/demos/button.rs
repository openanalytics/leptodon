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
use leptodon::button::AddButton;
use leptodon::button::Button;
use leptodon::button::ButtonAppearance;
use leptodon::button::ButtonShape;
use leptodon::button::CopyButton;
use leptodon::button::DeleteButton;
use leptodon::button::DownloadButton;
use leptodon::button::EditButton;
use leptodon::button_group::ButtonGroup;
use leptodon::button_group::First;
use leptodon::button_group::Last;
use leptodon::heading::Heading4;
use leptodon::icon;
use leptodon::layout::FixedCenterColumn;
use leptodon::paragraph::Paragraph;
use leptodon::textarea::TextArea;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::Update;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[generate_codeblock(ButtonExample)]
#[component]
pub fn ButtonDemo() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <Paragraph>
            {move ||
                format!("Button was pressed {} times!", count.get())
            }
        </Paragraph>

        <Button
            appearance=ButtonAppearance::Primary
            shape=ButtonShape::Rounded
            icon=icon::AddIcon()
            on_click=move |_| {
                count.update(|old| *old += 1);
            }
        >
            1
        </Button>
    }
}

#[generate_codeblock(ButtonGroupExample)]
#[component]
pub fn ButtonGroupDemo() -> impl IntoView {
    view! {
        <ButtonGroup>
            <First slot:first>
                <Button class="mr-0" on_click=move |_| {}>Profile</Button>
            </First>
            <Button on_click=move |_| {}>Settings</Button>
            <Button on_click=move |_| {}>Settings2</Button>
            <Last slot:last>
                <Button on_click=move |_| {}>Messages</Button>
            </Last>
        </ButtonGroup>
    }
}

#[generate_codeblock(StyledButtonExample)]
#[component]
pub fn PremadeButtonDemo() -> impl IntoView {
    let to_copy = RwSignal::new("📋".to_string());
    view! {
        <AddButton/>
        <EditButton/>
        <DeleteButton/>
        <DownloadButton/>
        <CopyButton class="m-2" to_copy=to_copy />
        <br/>
        <TextArea
            placeholder="Paste testing area.."
            value=RwSignal::new(String::default())
        />
    }
}

#[component]
pub fn ButtonDemoPage() -> impl IntoView {
    view! {
        <Title text="Button Components"/>

        <FixedCenterColumn>
            <Heading4 anchor="button">"Button"</Heading4>
            <ButtonExample />

            <Heading4 anchor="button-group">"Button Group"</Heading4>
            <ButtonGroupExample />

            <Heading4 anchor="premade-buttons">"Premade Buttons"</Heading4>
            <StyledButtonExample />

            <leptodon::button::ButtonDocs />
            <leptodon::button_group::ButtonGroupDocs />
        </FixedCenterColumn>
    }
}
