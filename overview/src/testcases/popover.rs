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
use leptodon::popover::Popover;
use leptodon::popover::PopoverAnchor;
use leptodon::popover::PopoverTrigger;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
fn PopoverRow(#[prop(into)] id: String) -> impl IntoView {
    let id_left = format!("{id}-left");
    let id_top = format!("{id}-top");
    let id_bot = format!("{id}-bottom");
    let id_right = format!("{id}-right");
    let id_left_popover = format!("{id_left}-popover");
    let id_top_popover = format!("{id_top}-popover");
    let id_bot_popover = format!("{id_bot}-popover");
    let id_right_popover = format!("{id_right}-popover");
    view! {
        <Popover preferred_pos=PopoverAnchor::Left>
            <PopoverTrigger slot>
                <Button id=id_left>"L"</Button>
            </PopoverTrigger>
            <p id=id_left_popover>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Top>
            <PopoverTrigger slot>
                <Button id=id_top>"T"</Button>
            </PopoverTrigger>
            <p id=id_top_popover>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Bottom>
            <PopoverTrigger slot>
                <Button id=id_bot>"B"</Button>
            </PopoverTrigger>
            <p id=id_bot_popover>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Right>
            <PopoverTrigger slot>
                <Button id=id_right>"R"</Button>
            </PopoverTrigger>
            <p id=id_right_popover>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
    }
}

#[component]
pub fn TestPopover() -> impl IntoView {
    view! {
        <Title text="Test Popover"/>

        <div class="h-[300vh] w-[300vw] flex justify-center items-center">
            <div class="h-[100vh] w-[100vw] flex flex-col justify-between">
                <div class="flex flex-row justify-between">
                    <PopoverRow id="row1" />
                </div>
                <div class="flex flex-row justify-between">
                    <PopoverRow id="row2"/>
                </div>
                <div class="flex flex-row justify-between">
                    <PopoverRow id="row3"/>
                </div>
            </div>
        </div>
    }
}
