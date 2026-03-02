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
use crate::icon::ApproveIcon;
use crate::icon::ApprovedIcon;
use crate::icon::CloseIcon;
use crate::icon::Icon;
use crate::icon::InfoIcon;
use crate::icon::WarningIcon;
use crate::icon::icon_data::IconData;
use crate::icon::icon_data::IconRef;
use crate::util::callback::ArcOneCallback;
use crate::util::option_comp::OptionComp;
use crate::util::shared_id::shared_id;
use attr_docgen::generate_docs;
use leptos::context::Provider;
use leptos::logging::debug_log;
use leptos::prelude::Callable;
use leptos::prelude::Callback;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Show;
use leptos::prelude::Update;
use leptos::prelude::ViewFn;
use leptos::server::SharedValue;
use leptos::server::codee::string::FromToStringCodec;
use leptos::{IntoView, component, prelude::MaybeProp, view};

pub const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";

#[derive(Default)]
pub enum ToastAppearance {
    Success,
    Warning,
    Danger,
    #[default]
    Inform,
    Plain,
}

impl ToastAppearance {
    fn icon(&self) -> Option<IconRef> {
        match self {
            ToastAppearance::Success => Some(ApproveIcon()),
            ToastAppearance::Warning => Some(WarningIcon()),
            ToastAppearance::Danger => Some(CloseIcon()),
            ToastAppearance::Inform => Some(InfoIcon()),
            ToastAppearance::Plain => None,
        }
    }

    fn icon_color(&self) -> &'static str {
        match self {
            ToastAppearance::Success => "text-success bg-success-soft",
            ToastAppearance::Warning => "text-warning bg-warning-soft",
            ToastAppearance::Danger => "text-danger bg-danger-soft",
            ToastAppearance::Inform => "text-oa-blue bg-oa-soft",
            ToastAppearance::Plain => "",
        }
    }
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

pub type ToastId = Arc<SharedValue<String, FromToStringCodec>>;

#[derive(Clone)]
pub struct ToastDetails {
    /// The toast
    pub view: ViewFn,
    /// The toast_id
    pub toast_id: ToastId,
}

pub type ShowToastCallback = ArcOneCallback<ViewFn, ()>;
pub type DissmissToastCallback = Callback<(), bool>;

#[derive(Clone)]
pub struct ToasterContext {
    /// Will show the toast and return its id.
    pub show_toast: ArcOneCallback<ToastDetails, ToastId>,
    /// Returns whether the toast existed and was dissmissed.
    pub dissmiss_toast: ArcOneCallback<ToastId, bool>,
}

impl ToasterContext {
    pub fn use_toast(&self) -> (ShowToastCallback, DissmissToastCallback) {
        let toast_id = Arc::new(shared_id());
        let show_toast = ArcOneCallback::new({
            let toast_id = toast_id.clone();
            let show_toast = self.show_toast.clone();
            move |view_fn| {
                (show_toast)(ToastDetails {
                    view: view_fn,
                    toast_id: toast_id.clone(),
                });
            }
        });
        let dissmiss_toast = Callback::new({
            let toast_id = toast_id.clone();
            let dissmiss_toast = self.dissmiss_toast.clone();
            move |_| (dissmiss_toast)(toast_id.clone())
        });
        (show_toast, dissmiss_toast)
    }
}

#[generate_docs]
#[component]
// TODO: Support other screen locations for displaying toasts at.
pub fn Toaster(#[prop(optional)] _location: ToastLocation, children: Children) -> impl IntoView {
    let toast_queue: RwSignal<Vec<(ToastId, ToastDetails)>> = RwSignal::new(vec![]);
    let show_toast = ArcOneCallback::new(move |toast: ToastDetails| {
        let toast_id = toast.toast_id.clone();
        toast_queue.update(|toast_queue| {
            toast_queue.push((toast_id.clone(), toast));
        });
        debug_log!("Added toast with {}", *toast_id);
        toast_id
    });

    let dissmiss_toast = ArcOneCallback::new(move |to_dismiss_toast_id: ToastId| {
        let mut toasts = toast_queue.get();

        if let Some(position) = toasts
            .iter()
            .position(|(toast_id, _)| toast_id == &to_dismiss_toast_id)
        {
            debug_log!("Found toast at {position}, removing");
            toasts.remove(position);
            toast_queue.set(toasts);
            return true;
        }
        debug_log!("Did not find {to_dismiss_toast_id}");
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
                {move || {
                    let toasts = toast_queue.get();
                    toasts.iter().map(|(_, toast)| view! {
                        {toast.view.run()}
                    }).collect_view()
                }}
            </div>
        </div>
    }
}

/// A small floating popup to show feedback in response to an action without disrupting the DOM layout.
/// See [Toaster] and [ToastContext#use_toast].
#[generate_docs]
#[component]
pub fn Toast(
    /// Html id
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Toast title, shown in bold
    #[prop(optional, into)]
    title: MaybeProp<String>,
    /// Extra toast-style classes
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Toast message content
    #[prop(optional, into)]
    message: MaybeProp<String>,
    /// Appearance style of the toast
    #[prop(optional, into)]
    appearance: ToastAppearance,
    /// Whether this toast should have a close button
    #[prop(default = true)]
    dismissable: bool,
    /// Callback to dismiss this toast. See ToastContext#use_toast();
    dismiss: DissmissToastCallback,
    /// Extra toast content provided as children
    #[prop(optional)]
    children: Option<Children>,
) -> impl IntoView {
    view! {
        <div
            id=id.get()
            class=class_list!(
                "flex items-center w-full max-w-sm p-4 text-body bg-gray-100 dark:bg-gray-700 rounded-lg shadow border border-gray-300 dark:border-gray-600",
                class
            )
            role="alert"
        >
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <OptionComp value=appearance.icon() let:icon>
                        <Icon
                            class=class_list!(
                                appearance.icon_color(),
                                "rounded pe-2.5 me-3.5 border-e border-gray-300 dark:border-gray-600"
                            )
                            icon=icon
                        />
                    </OptionComp>
                    {move || {
                        if let Some(title) = title.get() {
                            view! { <b>{title}</b> }.into_any()
                        } else {
                            view! {
                                <div class="text-sm">
                                    {message}
                                </div>
                            }.into_any()
                        }
                    }}
                </div>

                <Show when=move || dismissable fallback=|| ().into_any()>
                    <Button class="ms-auto" icon=icon::CloseIcon() appearance=ButtonAppearance::Minimal on_click=move |_| {
                        dismiss.run(());
                    } />
                </Show>
            </div>
            {move || {
                if title.get().is_some() && let Some(message) = message.get() {
                    view! {
                        <div class="text-sm">
                            {message}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </div>
    }
}
