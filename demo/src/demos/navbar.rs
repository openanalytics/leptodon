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
use leptodon::codeblock::Codeblock;
use leptodon::divider::HorizontalLine;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::paragraph::Paragraph;
use leptos::html::ElementChild;
use leptos::prelude::ClassAttribute;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn SideNavbarDemoPage() -> impl IntoView {
    view! {
        <Title text="Navbar"/>

        <FixedCenterColumn>
            <Heading4 anchor="side-navbar">"SideNavbar"</Heading4>
            <div class="flex mb-3 flex-col border-1 border rounded-lg shadow w-fit p-4 min-w-[50vw] dark:bg-[#1F2937] dark:border-gray-600">
                <div class="p-3">
                    <Paragraph>"No demo here since it conflicts with the existing navigation 🥹."</Paragraph>
                </div>
                <HorizontalLine />

                <Codeblock code=r##"
<SideNavbar>
    <NavbarEntries slot:entries>
        <li><SideBarLink href="/">Home</SideBarLink></li>
    </NavbarEntries>
    <NavbarEndChildren slot:end>
        "End section"
    </NavbarEndChildren>
    "Page content"
</SideNavbar>
                "##/>
            </div>

            <Heading4 anchor="route-shell">"Route shell"</Heading4>
            <div class="flex mb-3 flex-col border-1 border rounded-lg shadow w-fit p-4 min-w-[50vw] dark:bg-[#1F2937] dark:border-gray-600">
                <div class="p-3">
            "
            Leptos supports having a component wrap content on a set of routes.
            This is useful when splitting your website into pages but wanting to reuse the same navbar.

            For this website the route-shell looks somewhat as follows:"
                </div>
                <HorizontalLine />

                <Codeblock code=r##"
#[component]
pub fn RouteShell() -> impl IntoView {
    let demo_links = ...;
    view! {
        <main>
            <SideNavbar>
                <NavbarEntries slot:entries>
                    <li><SideBarLink href="#">Home</SideBarLink></li>
                    {demo_links}
                </NavbarEntries>
                <NavbarEndChildren slot:end>
                    <ThemeSelector />
                </NavbarEndChildren>

                <Outlet />
            </SideNavbar>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Router>
            <Routes fallback=|| "Page not found.">
                <ParentRoute path=StaticSegment("/") view=RouteShell>
                    <Route path=StaticSegment("/") view=Home/>
                    <DemoRoutes/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}
                "##
                />
            </div>

            <leptodon::navbar::SideNavbarDocs />
        </FixedCenterColumn>
    }
}
