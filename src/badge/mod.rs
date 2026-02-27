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
use crate::avatar::Avatar;
use crate::class_list;
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::spinner::Spinner;
use crate::util::callback::ArcOneCallback;
use attr_docgen::generate_docs;
use leptos::either::Either;
use leptos::prelude::AriaAttributes;
use leptos::prelude::{AnyView, Children, ClassAttribute, ElementChild, IntoAny, MaybeProp};
use leptos::prelude::{Get, OnAttribute, RwSignal, Set};
use leptos::{IntoView, component, view};
use web_sys::MouseEvent;

const BADGE_BASE_CLASSES: &str = "flex font-medium rounded-lg h-fit w-fit";

#[generate_docs]
/// A badge
#[component]
pub fn Badge(
    #[prop(optional)] size: BadgeSize,
    /// Extra styling
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional)] theme: BadgeTheme,
    /// Shown inside the badge, before the children
    #[prop(optional, into)]
    prefix: MaybeProp<BadgePrefix>,
    /// Shown inside the badge, after the children
    #[prop(optional, into)]
    postfix: MaybeProp<BadgePostfix>,
    /// Whether the badge should have a border
    #[prop(optional)]
    border: bool,
    /// Whether the badge should have a cross button to dismiss it.
    #[prop(optional)]
    dismissable: bool,
    /// Ran on dismissal request
    #[prop(optional)]
    on_dismiss: Option<ArcOneCallback<MouseEvent>>,
    /// Usually text content.
    children: Children,
) -> impl IntoView {
    let prefix_classes = move |prefix: MaybeProp<BadgePrefix>| {
        prefix.get().map(|prefix| prefix.class()).unwrap_or("")
    };
    let dismissed = RwSignal::new(false);
    view! {
        <span
            class=class_list!(
                BADGE_BASE_CLASSES, class, theme.base_class(),
                (theme.border_class(), border),
                ("gap-1", dismissable),
                ("hidden", move || dismissed.get()),
                prefix_classes(prefix), prefix_classes(postfix), size.class()
            )
        >
            {move || if let Some(prefix) = prefix.get() {
                prefix.view()
            } else {
                ().into_any()
            }}
            <span>{children()}</span>
            {move || if let Some(postfix) = postfix.get() {
                postfix.view()
            } else {
                ().into_any()
            }}
            {if dismissable {
                Either::Left(view!{
                    <button type="button" class="inline-flex items-center p-0.5 text-sm bg-transparent rounded-xs hover:bg-neutral-tertiary" aria_label="Remove" on:click=move |e| {
                        if let Some(on_dismiss) = on_dismiss.clone() {
                            on_dismiss(e);
                        }
                        dismissed.set(true);
                    }>
                        <svg class="w-3 h-3" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/></svg>
                        <span class="sr-only">Remove badge</span>
                    </button>
                })
            } else {
                Either::Right(())
            }}
        </span>
    }
}

#[derive(Default)]
pub enum BadgeSize {
    #[default]
    Normal,
    Large,
}

impl BadgeSize {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeSize::Normal => "text-xs px-1 py-0.5",
            BadgeSize::Large => "text-sm px-2 py-1",
        }
    }
}

#[derive(Default)]
pub enum BadgeTheme {
    #[default]
    Brand,
    Secondary,
    Transparent,
    Danger,
    Success,
    Warning,
}

impl BadgeTheme {
    // Color theme
    pub fn base_class(&self) -> &'static str {
        match self {
            BadgeTheme::Brand => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-300",
            BadgeTheme::Secondary => {
                "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-300"
            }
            BadgeTheme::Transparent => "dark:text-white",
            BadgeTheme::Danger => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-300",
            BadgeTheme::Success => {
                "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-300"
            }
            BadgeTheme::Warning => {
                "bg-yellow-100 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-300"
            }
        }
    }

    // Border and their colors
    pub fn border_class(&self) -> &'static str {
        match self {
            BadgeTheme::Brand => "border border-blue-400",
            BadgeTheme::Secondary => "border border-gray-400",
            BadgeTheme::Transparent => "border border-gray-400",
            BadgeTheme::Danger => "border border-red-400",
            BadgeTheme::Success => "border border-green-400",
            BadgeTheme::Warning => "border border-yellow-400",
        }
    }
}

pub type BadgePostfix = BadgePrefix;
#[derive(Clone)]
pub enum BadgePrefix {
    Icon(IconRef),
    Dot,
    SvgLoader,
    Avatar {
        /// Url or data of the avatar image
        src: String,
    },
}
impl BadgePrefix {
    pub fn class(self) -> &'static str {
        "inline-flex items-center"
    }

    pub fn view(&self) -> AnyView {
        let size = "h-[1lh] w-[1lh] me-1";
        match self {
            BadgePrefix::Icon(icon_data) => view! {
                <Icon class=class_list!(size) icon=*icon_data/>
            }
            .into_any(),
            BadgePrefix::Dot => view! {
                <span class=class_list!("w-1.5 h-1.5 me-1 bg-oa-blue rounded-full")></span>
            }
            .into_any(),
            BadgePrefix::SvgLoader => view! {
                <Spinner class=size />
            }
            .into_any(),
            BadgePrefix::Avatar { src } => view! {
                <Avatar src=src.to_string() class=class_list!(size) />
            }
            .into_any(),
        }
    }
}
