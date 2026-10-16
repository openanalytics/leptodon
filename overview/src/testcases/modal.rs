use leptodon::button::ButtonAppearance;
use leptodon::modal::Modal;
use leptodon::modal::ModalFooterChildren;
use leptodon::paragraph::Paragraph;
use leptodon::util::lorem::Lorem;
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
use leptos::logging::log;
use leptos::prelude::AddAnyAttr;
use leptos::reactive::signal::RwSignal;
use leptos::reactive::traits::Get;
use leptos::reactive::traits::Set;
use leptos::tachys::view::any_view::IntoAny;
use leptos::{IntoView, component, view};

#[component]
pub fn TestModal() -> impl IntoView {
    let visible = RwSignal::new(false);
    let footer = ModalFooterChildren::builder()
        .children({
            Box::new(move || {
                view! {
                    <Button>"Modal action 3"</Button>
                    <Button>"Modal action 2"</Button>
                    <Button
                        appearance=ButtonAppearance::Primary
                        on_click=move |_| {
                            visible.set(false);
                        }
                    >"Modal action 1"</Button>
                }
                .into_any()
            })
        })
        .build();
    view! {
        <Paragraph>
            "Shown: " {move || format!("{:?}", visible.get())}
        </Paragraph>
        <Modal
            attr:data-testid="dialog-backdrop"
            id="example=modal"
            title="Example modal?"
            visible
            footer
            on_close=|_| {
                log!("hello there");
            }
        >

            // Content
            <Lorem sentences=2/>
        </Modal>
        <Button on_click=move |_| {
            visible.set(true);
        }>Open modal</Button>
    }
}
