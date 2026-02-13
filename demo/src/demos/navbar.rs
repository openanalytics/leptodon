use leptos::html::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::codeblock::Codeblock;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
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

            <leptos_components::navbar::SideNavbarDocs />
        </FixedCenterColumn>
    }
}
