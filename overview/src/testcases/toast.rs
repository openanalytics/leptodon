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
use leptodon::{
    button::Button,
    toast::{Toast, Toaster, ToasterContext},
};
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, prelude::use_context, view};
use leptos_meta::Title;

#[component]
pub fn TestToast() -> impl IntoView {
    view! {
        <Title text="Test Toast"/>
        <Toaster>
            <Button on_click=move |_| {
                if let Some(toast_ctx) = use_context::<ToasterContext>() {
                    let (show_toast, dismiss_toast) = toast_ctx.use_toast();
                    show_toast((move || view! {
                        <Toast id="permanent-toast" title="Permanent toast" dismiss=dismiss_toast dismissable=false />
                    }).into());

                    let (show_toast, dismiss_toast) = toast_ctx.use_toast();
                    show_toast((move || view! {
                        <Toast id="dismissable-toast" title="Dismissable toast" message="Don't forget to drink water!" dismiss=dismiss_toast />
                    }).into());

                    let (show_toast, dismiss_toast) = toast_ctx.use_toast();
                    show_toast((move || view! {
                        <Toast id="detailed-toast" title="Detailed toast" dismiss=dismiss_toast>
                            <ul>
                                <li>Details</li>
                            </ul>
                        </Toast>
                    }).into());
                }
            }>"Show Toasts"</Button>
        </Toaster>
    }
}
