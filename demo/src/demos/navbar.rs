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
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::html::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn SideNavbarDemoPage() -> impl IntoView {
    view! {
        <Title text="Navbar"/>

        <FixedCenterColumn>
            <Heading4 anchor="side-navbar">"SideNavbar"</Heading4>
            <p>"No demo here since it conflicts with the existing navigation 🥹."</p>
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

            <Heading4 anchor="route-shell">"Route shell"</Heading4>
            "
            Leptos supports having a component wrap content on a set of routes.
            This is useful when splitting your website into pages but wanting to reuse the same navbar.

            For this website the route-shell looks somewhat as follows:"
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
        <Stylesheet href="/pkg/overview.css"/>
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

            <leptodon::navbar::SideNavbarDocs />
        </FixedCenterColumn>
    }
}
