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
use attr_docgen::generate_docs;
use leptos::children::Children;
use leptos::logging::debug_log;
use leptos::logging::error;
use leptos::prelude::AriaAttributes;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
#[allow(unused_imports)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::slot;
use leptos::{IntoView, component, prelude::MaybeProp, view};
use leptos_use::UseTimeoutFnReturn;
use leptos_use::use_timeout_fn;
use web_sys::KeyboardEvent;

use crate::button::ButtonRef;
use crate::button::ControlButton;
use crate::class_list;
use crate::icon::CloseIcon;
use crate::util::signals::ComponentRef;

const MODAL_CLASSES: &str =
    "relative bg-white rounded-lg shadow dark:bg-gray-700 px-4 m-4 w-full max-w-2xl max-h-full";
const MODAL_BACKDROP_CLASSES: &str = "overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-[1000] justify-center items-center w-full md:inset-0 h-full max-h-full flex bg-black/50";

#[slot]
pub struct ModalFooterChildren {
    children: Children,
}

#[generate_docs]
#[component]
pub fn Modal(
    /// Modal id
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra modal classes
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Title shown in the modal heading
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// True shows the modal, false hides it.
    #[prop(optional, into)]
    visible: RwSignal<bool>,
    /// Modal content
    children: Children,
    /// Modal footer (e.g. Ok and Cancel buttons)
    footer: ModalFooterChildren,
) -> impl IntoView {
    let first_button: ComponentRef<ButtonRef> = ComponentRef::new();

    let warp_focus = move || {
        let Some(first_button): Option<ButtonRef> = first_button.get() else {
            error!("Internal modal first-div reference is not mounted!");
            return;
        };
        // Somehow this hack makes focus work..
        let UseTimeoutFnReturn { start, .. } = use_timeout_fn(
            move |_| {
                first_button.focus();
            },
            0.0,
        );
        start(());
        debug_log!("modal: successfully focused first-div");
    };

    Effect::watch(
        move || visible.get(),
        move |new, old, _| {
            if *new && Some(new) != old {
                warp_focus();
            }
        },
        false,
    );

    view! {
        <div tabindex="-1"
            class=class_list!(
                MODAL_BACKDROP_CLASSES,
                ("hidden", move || !visible.get())
            )
            on:click=move |_| visible.set(false)
            on:keydown=move |key: KeyboardEvent| {
                if key.code() == "Escape"{
                    visible.set(false);
                }
            }
        >
            // Modal content
            <div
                id=move || id.get()
                class=class_list!(MODAL_CLASSES, class)
                on:click=move |e| e.stop_propagation()
                role="dialog"
                aria-label=move || title.get()
                aria-modal=true
            >
                // Backward Focus Blocker, Preferably this would loop to the last_button but last_button is unknown.
                <span tabindex="0" aria-hidden="true" on:focus=move |_| warp_focus()></span>

                // Modal header
                <div class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-gray-600">
                    <h3 class="text-lg font-medium text-heading">
                        { title.get() }
                    </h3>
                    <ControlButton
                        icon=CloseIcon()
                        comp_ref=first_button
                        on_click=move |_| visible.set(false)
                    />
                </div>

                // Modal body
                <div class="p-4 md:p-5 space-y-4">
                    {children()}
                </div>

                // Modal footer
                <div
                    class="flex items-center p-4 md:p-5 border-t border-gray-200 rounded-b dark:border-gray-600">
                    {(footer.children)().into_any()}
                </div>

                // Forward Focus Redirector
                <span tabindex="0" on:focus=move |_| warp_focus() aria-hidden="true"></span>
            </div>
        </div>
    }
}
