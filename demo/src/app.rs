use leptos::prelude::*;
use leptos_components::accordion::Accordion;
use leptos_components::accordion::AccordionEntry;
use leptos_components::button::{Button, ButtonAppearance};
use leptos_components::darkmode::ThemeSelector;
use leptos_components::icon;
use leptos_components::navbar::NavbarEndChildren;
use leptos_components::navbar::NavbarEntries;
use leptos_components::navbar::SideBarLink;
use leptos_components::navbar::SideNavbar;
use leptos_meta::MetaTags;
use leptos_meta::Stylesheet;
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

const NAME: &str = "OA Leptos-Components";

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en" class="min-h-full">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
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
                    <li><SideBarLink href="#" icon=icon::BillingIcon()>Home</SideBarLink></li>
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

// #[component]
// fn Home() -> impl IntoView {
//     view! {
//         <p>Hello World!</p>
//     }
// }

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text=NAME />
        <main class="flex justify-center align-center min-h-full">
            <div>
                <h1 class="font-bold text-4xl">{NAME}</h1>
                <h2 class="text-xl">A ThawUI inspired component library for the leptos web-framework.</h2>
                <Button appearance=ButtonAppearance::Primary>Docs</Button>
                <Button>Crate</Button>
                <Button>Github</Button>
            </div>
        </main>
    }
}

#[component]
pub fn AnyAccordion() -> AnyView {
    view!{
        <Accordion>
            <AccordionEntry title="What is Flowbite?">
                <p class="mb-2 text-body">Flowbite is an open - source library of interactive components built on top of
                Tailwind CSS including buttons, dropdowns, modals, navbars, and more.</p>
                <p class="mb-2 text-body">Check out this guide to learn how to<a href="/docs/getting-started/introduction/" class="text-fg-brand hover:underline">get started</a>and start developing websites even faster with components on top of Tailwind
                CSS.</p>

                <Accordion>
                    <AccordionEntry title="What about version 2.7.2?">
                        <p class="mb-2 text-body">Version 2.7.2 is available at <a href="https://web.archive.org/web/20240328025144/https://flowbite.com/docs/components/dropdowns/">this location</a></p>
                    </AccordionEntry>
                </Accordion>
            </AccordionEntry>
        </Accordion>
    }.into_any()
}

#[component]
pub fn Settings(children: Children) -> AnyView {
    view! {
        <section class="bg-white dark:bg-gray-900">
            <div class="grid max-w-screen-xl px-4 py-4 mx-auto lg:gap-8 xl:gap-0">
                <div class="mr-auto">
                    <h2
                        id="settings-title"
                        class="max-w-2xl mb-4 text-xl font-medium tracking-tight leading-none md:text-2xl xl:text-3xl dark:text-white"
                    >
                        "Settings"
                    </h2>
                    <div class="max-w-2xl mb-6 lg:mb-8">{children()}</div>
                </div>
            </div>
        </section>
    }.into_any()
}
