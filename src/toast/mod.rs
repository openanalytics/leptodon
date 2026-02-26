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
use crate::icon;
use crate::icon::Icon;
use attr_docgen::generate_docs;
use leptos::context::Provider;
use leptos::logging::debug_log;
use leptos::prelude::AnyView;
use leptos::prelude::ArcMultiAction;
use leptos::prelude::ArcRwSignal;
use leptos::prelude::ArcTrigger;
use leptos::prelude::AriaAttributes;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CustomAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Show;
use leptos::prelude::Trigger;
use leptos::prelude::ViewFn;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
pub const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";

#[derive(Default)]
pub enum ToastAppearance {
    Success,
    Warning,
    Failure,
    #[default]
    Inform,
    Plain,
}

#[derive(Default)]
pub enum ToastLocation {
    Top,
    TopRight,
    Right,
    #[default]
    BottomRight,
    Bottom,
    BottomLeft,
    Left,
    TopLeft,
}

#[derive(Clone)]
struct ToastContext {
    bread_channel: ArcRwSignal<Vec<ViewFn>>,
}

#[generate_docs]
#[component]
pub fn Toaster(#[prop(optional)] location: ToastLocation, children: Children) -> impl IntoView {
    let toast_ctx = ToastContext {
        bread_channel: ArcRwSignal::new(vec![]),
    };
    view! {
        <Provider<ToastContext, _> value=toast_ctx>
            {children()}
        </Provider<ToastContext, _>>
        <div class="fixed outline outline-dashed">
            <Toast title="Example toast" message="Don't forget to drink water!"/>

        </div>
    }
}

#[generate_docs]
#[component]
pub fn Toast(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] title: MaybeProp<String>,
    #[prop(optional, into)] message: MaybeProp<String>,
    #[prop(optional, into)] appearance: ToastAppearance,
    #[prop(default = true)] dissmissable: bool,
) -> impl IntoView {
    view! {
        <div
            id=id.get()
            class="flex items-center w-full max-w-sm p-4 text-body bg-neutral-primary-soft rounded-base shadow-xs border border-default"
            role="alert"
        >
            <Icon icon=icon::ApproveIcon()/>
            <div class="ms-2.5 text-sm border-s border-default ps-3.5">
                {message}
            </div>
            <button type="button" class="ms-auto flex items-center justify-center text-body hover:text-heading bg-transparent box-border border border-transparent hover:bg-neutral-secondary-medium focus:ring-4 focus:ring-neutral-tertiary font-medium leading-5 rounded text-sm h-8 w-8 focus:outline-none" data-dismiss-target="#toast-simple" aria-label="Close">
                <span class="sr-only">Close</span>
                <Icon icon=icon::CloseIcon() />
            </button>
        </div>

    }
}
