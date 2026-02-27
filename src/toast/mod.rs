use std::sync::Arc;

use crate::button::Button;
use crate::button::ButtonAppearance;
use crate::class_list;
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
use crate::toast;
use crate::util::callback::ArcOneCallback;
use crate::util::shared_id::shared_id;
use attr_docgen::generate_docs;
use leptos::context::Provider;
use leptos::logging::error;
use leptos::prelude::AnyView;
use leptos::prelude::ArcRwSignal;
use leptos::prelude::AriaAttributes;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Show;
use leptos::prelude::Update;
use leptos::prelude::ViewFn;
use leptos::prelude::WriteSignal;
use leptos::prelude::use_context;
use leptos::server::SharedValue;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{IntoView, component, prelude::MaybeProp, view};
use web_sys::window;
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
    TopLeft,
    TopCenter,
    TopRight,
    #[default]
    BottomRight,
    BottomCenter,
    BottomLeft,
}

type ToastId = Arc<SharedValue<String, FromToStringCodec>>;

#[derive(Clone)]
pub struct ToastDetails {
    /// The toast
    pub view: ViewFn,
}

#[derive(Clone)]
pub struct ToasterContext {
    /// Will show the toast and return its id.
    pub show_toast: ArcOneCallback<ToastDetails, ToastId>,
    /// Returns whether the toast existed and was dissmissed.
    pub dissmiss_toast: ArcOneCallback<ToastId, bool>,
}

// pub struct Toasts;
// impl Toasts {
//     pub fn show(toastProps: ToastProps) {
//         let Some(context) = use_context::<ToasterContext>() else {
//             window().map(|w| w.alert_with_message("Error, see console for more info."));
//             error!(
//                 "Tried to show a toast outside of a ToasterContext, use <Toaster> to provide the context to Toaster's children."
//             );
//             return;
//         };
//         let toast_view = Toast(toastProps);
//         let id = Arc::new(shared_id());
//         let details = ToastDetails {
//             id,
//             view: (|| Toast(ToastPropsBuilder::message("hello", "ehlo").build())).into(),
//         };
//         Self::show_details(details)
//     }

//     pub fn show_details(details: ToastDetails) {
//         let Some(context) = use_context::<ToasterContext>() else {
//             window().map(|w| w.alert_with_message("Error, see console for more info."));
//             error!(
//                 "Tried to show a toast outside of a ToasterContext, use <Toaster> to provide the context to Toaster's children."
//             );
//             return;
//         };
//         context.toast_queue.update(|queue| queue.push(details));
//     }
// }

#[generate_docs]
#[component]
// TODO: Support other screen locations for displaying toasts at.
pub fn Toaster(#[prop(optional)] _location: ToastLocation, children: Children) -> impl IntoView {
    let toast_queue: RwSignal<Vec<(ToastId, ToastDetails)>> = RwSignal::new(vec![]);
    let show_toast = ArcOneCallback::new(move |toast: ToastDetails| {
        let toast_id = Arc::new(shared_id());
        toast_queue.update(|toast_queue| {
            toast_queue.push((toast_id.clone(), toast));
        });
        toast_id
    });
    let dissmiss_toast = ArcOneCallback::new(move |to_dismiss_toast_id: ToastId| {
        let mut toasts = toast_queue.get();

        if let Some(position) = toasts
            .iter()
            .position(|(toast_id, _)| toast_id == &to_dismiss_toast_id)
        {
            toasts.remove(position);
            return true;
        }
        toast_queue.set(toasts);

        false
    });
    let toast_ctx = ToasterContext {
        show_toast,
        dissmiss_toast,
    };
    view! {
        <Provider<ToasterContext, _> value=toast_ctx>
            {children()}
        </Provider<ToasterContext, _>>
        <div class="fixed outline outline-dashed right-0 bottom-0">
            <div class="flex flex-col p-4 gap-4">
                <Toast title="Example toast" message="Don't forget to drink water!"/>
                <Toast title="Example toast2" message="Don't forget to drink water!"/>
                {move || {
                    let toasts = toast_queue.get();
                    toasts.iter().map(|(toast_id, toast)| toast.view.run()).collect_view()
                }}
            </div>
        </div>
    }
}

#[generate_docs]
#[component]
pub fn Toast(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] title: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] message: MaybeProp<String>,
    #[prop(optional, into)] appearance: ToastAppearance,
    #[prop(default = true)] dissmissable: bool,
) -> impl IntoView {
    view! {
        <div
            id=id.get()
            class=class_list!("flex items-center w-full max-w-sm p-4 text-body bg-gray-100 dark:bg-gray-700 rounded-lg shadow border border-default", class)
            role="alert"
        >
            <Icon icon=icon::ApproveIcon()/>
            <div class="ms-2.5 text-sm border-s border-default ps-3.5">
                {message}
            </div>
            <Show when=move || dissmissable fallback=|| ().into_any()>
                <Button icon=icon::CloseIcon() appearance=ButtonAppearance::Minimal />
            </Show>
        </div>

    }
}
