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
use leptodon_proc_macros::generate_docs;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::{IntoAny, RwSignal};
use leptos::view;
use leptos::{
    IntoView, component,
    prelude::{Children, MaybeProp},
};

use crate::button::{Button, ButtonAppearance};
use crate::modal::{Modal, ModalFooterChildren};
use crate::util::callback::BoxCallback;

#[generate_docs]
#[component]
pub fn Dialog(
    /// Dialog id
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra dialog classes
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Title shown in the dialog heading
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// True shows the dialog, false hides it.
    #[prop(optional, into)]
    visible: RwSignal<bool>,
    /// Dialog primary-button
    #[prop(default = "Ok".into(), into)]
    primary_text: String,
    /// Click handler primary-button
    #[prop(default = BoxCallback::new(|| ()), into)]
    on_click_primary: BoxCallback,
    /// Dialog secondary-button
    #[prop(default = "Cancel".into(), into)]
    secondary_text: String,
    /// Click handler secondary-button
    #[prop(default = BoxCallback::new(|| ()), into)]
    on_click_secondary: BoxCallback,
    /// Dialog content
    children: Children,
) -> impl IntoView {
    let footer = ModalFooterChildren::builder()
        .children(Box::new(|| {
            view! {
                <div class="w-full flex justify-end">
                    <Button on_click=move |_| on_click_secondary()>{secondary_text}</Button>
                    <Button on_click=move |_| on_click_primary() appearance=ButtonAppearance::Primary>{primary_text}</Button>
                </div>
            }
            .into_any()
        }))
        .build();
    view! {
        <Modal id class title visible children footer />
    }
}
