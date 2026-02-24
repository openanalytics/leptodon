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
use leptodon::button::Button;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::popover::Popover;
use leptodon::popover::PopoverAnchor;
use leptodon::popover::PopoverTrigger;
use leptodon::util::lorem::Lorem;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(PopoverExample)]
#[component]
pub fn PopoverDemo() -> impl IntoView {
    view! {
        <Popover preferred_pos=PopoverAnchor::Left>
            <PopoverTrigger slot>
                <Button>"L"</Button>
            </PopoverTrigger>
            <p>
                <Lorem sentences=2/>
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Top>
            <PopoverTrigger slot>
                <Button>"T"</Button>
            </PopoverTrigger>
            <p>
                <Lorem sentences=4/>
            </p>
        </Popover>
    }
}

#[component]
pub fn PopoverDemoPage() -> impl IntoView {
    view! {
        <Title text="Popover"/>

        <FixedCenterColumn>
            <Heading4 anchor="popover">"Popover"</Heading4>
            <p>"Notice that the L popup likely does not appear on the left, since on most screens there is not enough space for the popover content.
            The popover component will automatically adjust its position to a more optimal side.
            Scrolling the popover into a wall is also supported."</p>
            <PopoverExample />

            <leptodon::popover::PopoverDocs />
        </FixedCenterColumn>
    }
}
