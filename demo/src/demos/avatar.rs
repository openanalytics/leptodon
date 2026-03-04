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
use leptodon::avatar::Avatar;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(AvatarPlaceholderExample)]
#[component]
pub fn AvatarPlaceholderDemo() -> impl IntoView {
    view! {
        <Avatar/>
    }
}

#[generate_codeblock(AvatarLinkExample)]
#[component]
pub fn AvatarLinkDemo() -> impl IntoView {
    view! {
        <Avatar src="https://avatars.githubusercontent.com/u/274806"/>
    }
}

#[component]
pub fn AvatarDemoPage() -> impl IntoView {
    view! {
        <Title text="Avatar Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="avatar-placeholder">"Avatar Placeholder"</Heading4>
            <AvatarPlaceholderExample />

            <Heading4 anchor="avatar">"Avatar Link"</Heading4>
            <AvatarLinkExample />

            <leptodon::avatar::AvatarDocs />
        </FixedCenterColumn>
    }
}
