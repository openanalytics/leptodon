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
use crate::button::{ControlButton, OA_TRANSPARENT_BUTTON_CLASSES};
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::link::Link;
use crate::{class_list, icon};
use attr_docgen::generate_docs;
use leptos::prelude::Get;
use leptos::prelude::MaybeProp;
use leptos::prelude::{AddAnyAttr, IntoAny, OnAttribute, RwSignal, Set, Update};
use leptos::prelude::{AriaAttributes, Children};
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::slot;
use leptos::view;
use leptos::{IntoView, component};
use leptos_router::components::ToHref;

const SIDEBAR_CLASSES: &str = "fixed top-0 left-0 z-40 w-64 h-screen pt-20 transition-transform bg-white border-r border-gray-200 lg:translate-x-0 dark:bg-gray-800 dark:border-gray-700";
const SIDEBAR_LINK_TEXT: &str = "text-gray-800 dark:text-gray-100 aria-current-page:text-oa-blue";
#[generate_docs]
#[component]
pub fn SideBarLink<H>(
    #[prop(optional, into)] icon: MaybeProp<IconRef>,
    href: H,
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <Link href colorless=true class=class_list!(
            OA_TRANSPARENT_BUTTON_CLASSES,
            SIDEBAR_LINK_TEXT,
            "w-full rounded-lg aria-current-page:bg-oa-gray dark:aria-current-page:bg-gray-700"
        )>
            {if let Some(icon) = icon.get() {
                view! { <Icon icon=icon /> }.into_any()
            } else {
                ().into_any()
            }}
           {children()}
        </Link>
    }
}

#[component]
fn LeptodonLogoLink() -> impl IntoView {
    view! {
        <a href="https://leptodon.dev/" class="flex items-center ms-2 md:me-24">
            <img src="/logo.svg" class="h-8 me-3" alt="Leptodon Logo"/>
            Leptodon
        </a>
    }
}

#[slot]
pub struct NavbarLogo {
    children: Children,
}

/// What to place at the navbar end.
#[slot]
pub struct NavbarEndChildren {
    children: Children,
}

/// Navigation entries
#[slot]
pub struct NavbarEntries {
    /// Expecting: <li><SideBarLink href="#" icon=icon::CalendarIcon()>Calendar</SideBarLink></li>
    children: Children,
}

#[generate_docs]
#[component]
pub fn SideNavbar(
    /// Page content
    children: Children,
    /// Slot for items in the top navbar
    #[prop(optional, default=NavbarEndChildren { children: Box::new(|| ().into_any()) })]
    end: NavbarEndChildren,
    /// Slot for branding
    #[prop(optional, default=NavbarLogo { children: Box::new(|| LeptodonLogoLink().into_any()) })]
    logo: NavbarLogo,
    /// Slot for sidebar entries
    #[prop(optional, default=NavbarEntries { children: Box::new(|| ().into_any()) })]
    entries: NavbarEntries,
) -> impl IntoView {
    let visible = RwSignal::new(false);

    view! {
        <nav class="fixed top-0 z-[500] w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
            <div class="px-3 py-3 lg:px-5 lg:pl-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center justify-start rtl:justify-end">
                        <ControlButton icon=icon::HamburgerIcon() on_click=move |_| {
                                visible.update(|is_visible| *is_visible = !*is_visible);
                            } class="lg:hidden" {..}
                            data-drawer-target="logo-sidebar" data-drawer-toggle="logo-sidebar" aria-controls="logo-sidebar"
                        />
                        {(logo.children)()}
                    </div>
                    <div class="flex items-center">
                        <div class="flex items-center ms-3">
                            {(end.children)()}
                        </div>
                    </div>
                </div>
            </div>
        </nav>

        <aside
            class=class_list!(
                SIDEBAR_CLASSES,
                ("-translate-x-full", move || !visible.get())
            )
            aria-label="Sidebar"
        >
            <div class="h-full px-3 pb-4 overflow-y-auto bg-white dark:bg-gray-800">
                <ul class="space-y-2 font-medium mt-2">
                    {(entries.children)()}
                </ul>
            </div>
        </aside>
        <div class=class_list![
                "lg:hidden fixed top-0 left-0 w-full h-full bg-gray-900/50 z-10",
                ("hidden", move || !visible.get())
            ]
            on:click=move |_| visible.set(false)
        />
        <div class="p-4 mt-20 lg:ml-64 min-h-full">
            {children().into_any()}
        </div>
    }
}
