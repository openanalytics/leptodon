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
use leptodon::toast::Toast;
use leptodon::toast::ToastAppearance;
use leptodon::toast::Toaster;
use leptodon::toast::ToasterContext;
use leptos::context::use_context;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(ToastExample)]
#[component]
pub fn ToastDemo() -> impl IntoView {
    view! {
        <Toaster> // <- Context provider, use once in your page root somewhere.
            <Button on_click={move |_| {
                let Some(toast_ctx) = use_context::<ToasterContext>() else {
                    return;
                };

                let (show_toast, dismiss_toast) = toast_ctx.use_toast();

                // Use show_toast no more than once, use_toast's returned callbacks have a set-id.
                show_toast((move || view! {
                    <Toast
                        title="Save your changes!"
                        message="You have not saved your changes yet, scroll to the bottom of the page to save."
                        dismiss=dismiss_toast
                        appearance=ToastAppearance::Warning
                    >
                        // Optional children.
                        <Button>"Got to Save button!"</Button>
                    </Toast>
                }).into());
            }}>
            </Button>
        </Toaster>
    }
}

#[component]
pub fn ToastDemoPage() -> impl IntoView {
    view! {
        <Title text="Toast"/>

        <FixedCenterColumn>
            <Heading4 anchor="toast">"Toast"</Heading4>
            <ToastExample />

            <leptodon::toast::ToastDocs />
            <leptodon::toast::ToasterDocs />
        </FixedCenterColumn>
    }
}
