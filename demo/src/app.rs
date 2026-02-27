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
use leptodon::button::{Button, ButtonAppearance};
use leptodon::darkmode::ThemeSelector;
use leptodon::icon;
use leptodon::navbar::NavbarEndChildren;
use leptodon::navbar::NavbarEntries;
use leptodon::navbar::SideBarLink;
use leptodon::navbar::SideNavbar;
use leptos::prelude::*;
use leptos_meta::HashedStylesheet;
use leptos_meta::MetaTags;
use leptos_meta::Title;
use leptos_meta::provide_meta_context;
use leptos_router::components::Outlet;
use leptos_router::components::ParentRoute;
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
};

use crate::generated_demolist::DemoRoutes;
use crate::generated_demolist::page_infos;

const NAME: &str = "Leptodon";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="min-h-full">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <link rel="icon" type="image/svg+xml" href="/logo.svg"/>
                <HashedStylesheet options=options.clone() id="leptos"/>

                // Metadata injection is not allowed here, only use them in components down the chain
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body class="min-h-full">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn RouteShell() -> impl IntoView {
    let demo_links = page_infos()
        .into_iter()
        .map(|page_info| {
            view! {
                <li><SideBarLink href=page_info.location>{page_info.name}</SideBarLink></li>
            }
        })
        .collect_view();
    view! {
        <main>
            <SideNavbar>
                <NavbarEntries slot:entries>
                    <li><SideBarLink href="#" icon=icon::HomeIcon()>Home</SideBarLink></li>
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

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text=NAME />
        <main class="flex justify-center align-center min-h-full">
            <div>
                <h1 class="font-bold text-4xl">{NAME}</h1>
                <h2 class="text-xl mb-2">"Your Leptos UI toolkit for data science."</h2>
                <a href="https://docs.rs/leptodon/latest">
                    <Button appearance=ButtonAppearance::Primary>Docs</Button>
                </a>
                <a href="https://crates.io/crates/leptodon"><Button>Crate</Button></a>
                <a href="https://github.com/openanalytics/leptodon"><Button>Github</Button></a>
            </div>
        </main>
    }
}
