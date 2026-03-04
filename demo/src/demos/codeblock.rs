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
use leptodon::codeblock::Codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(CodeblockExample)]
#[component]
pub fn CodeblockDemo() -> impl IntoView {
    view! {
        <Codeblock code=r#"fn main() {
    println!("Hello world!");
}"#>
        </Codeblock>
    }
}

#[component]
pub fn CodeblockDemoPage() -> impl IntoView {
    view! {
        <Title text="Codeblock"/>

        <FixedCenterColumn>
            <Heading4 anchor="codeblock">"Codeblock"</Heading4>
            <CodeblockExample />

            <leptodon::codeblock::CodeblockDocs />
        </FixedCenterColumn>
    }
}
