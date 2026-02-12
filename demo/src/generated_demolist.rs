
use leptos::{component, view};
use leptos_router::components::{Outlet, ParentRoute, Route};
use leptos_router::path;
use leptos_router::any_nested_route::IntoAnyNestedRoute;

pub struct PageInfo {
    pub location: &'static str,
    pub name: &'static str
}

pub fn page_infos() -> Vec<PageInfo> {
    vec![

        PageInfo {
            location: "/demo/badge",
            name: "Badge"
        },
        PageInfo {
            location: "/demo/avatar",
            name: "Avatar"
        },
        PageInfo {
            location: "/demo/toggle",
            name: "Toggle"
        },
        PageInfo {
            location: "/demo/calendar",
            name: "Calendar"
        },
        PageInfo {
            location: "/demo/accordion",
            name: "Accordion"
        },
        PageInfo {
            location: "/demo/input",
            name: "Inputs"
        },
        PageInfo {
            location: "/demo/button",
            name: "Button"
        }
    ]
}

#[component(transparent)]
pub fn DemoRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("demo") view=Outlet>
            <Route path=path!("/badge") view=crate::demos::badge::BadgeDemoPage/>
            <Route path=path!("/avatar") view=crate::demos::avatar::AvatarDemoPage/>
            <Route path=path!("/toggle") view=crate::demos::toggle::ToggleDemoPage/>
            <Route path=path!("/calendar") view=crate::demos::calendar::CalendarDemoPage/>
            <Route path=path!("/accordion") view=crate::demos::accordion::AccordionDemoPage/>
            <Route path=path!("/input") view=crate::demos::input::InputsDemoPage/>
            <Route path=path!("/button") view=crate::demos::button::ButtonDemoPage/>
        </ParentRoute>
    }
    .into_inner()
    .into_any_nested_route()
}
