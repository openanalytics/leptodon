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
use leptos::prelude::MaybeProp;
use leptos::prelude::{AddAnyAttr, IntoAny};
use leptos::{IntoView, component, oco::Oco, prelude::Children, view};
use leptos_router::components::A;
use leptos_router::components::ToHref;

use crate::class_list;

const LINK_CLASSES: &str = "text-oa-blue hover:underline";

#[generate_docs]
/// See [[A](leptos_router::components::A)]
#[component]
pub fn Link<H>(
    /// Used to calculate the link's `href` attribute. Will be resolved relative
    /// to the current route.
    href: H,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Where to display the linked URL, as the name for a browsing context (a tab, window, or `<iframe>`).
    #[prop(default = Oco::Borrowed(""), into)]
    target: Oco<'static, str>,
    /// If `true`, the link is marked aria-active when the location matches exactly;
    /// if false, link is marked aria-active if the current route starts with it.
    #[prop(optional)]
    exact: bool,
    /// If `true`, and when `href` has a trailing slash, `aria-current` will only be set if `current_url` also has
    /// a trailing slash.
    #[prop(optional)]
    strict_trailing_slash: bool,
    /// If `true`, the router will scroll to the top of the window at the end of navigation. Defaults to `true`.
    #[prop(default = true)]
    scroll: bool,
    /// The nodes or elements to be shown inside the link.
    children: Children,
) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <A href target exact strict_trailing_slash scroll {..} class=class_list![class, LINK_CLASSES]>
            {children().into_any()}
        </A>
    }
}
