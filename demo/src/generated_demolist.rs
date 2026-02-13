
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
            location: "/demo/navbar",
            name: "Side Navbar"
        },
        PageInfo {
            location: "/demo/badge",
            name: "Badge"
        },
        PageInfo {
            location: "/demo/select",
            name: "Select"
        },
        PageInfo {
            location: "/demo/avatar",
            name: "Avatar"
        },
        PageInfo {
            location: "/demo/divider",
            name: "Divider"
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
            location: "/demo/dropdown",
            name: "Dropdown"
        },
        PageInfo {
            location: "/demo/dialog",
            name: "Dialog"
        },
        PageInfo {
            location: "/demo/codeblock",
            name: "Codeblock"
        },
        PageInfo {
            location: "/demo/form_input",
            name: "Form Input"
        },
        PageInfo {
            location: "/demo/modal",
            name: "Modal"
        },
        PageInfo {
            location: "/demo/checkbox",
            name: "Checkbox"
        },
        PageInfo {
            location: "/demo/heading",
            name: "Heading"
        },
        PageInfo {
            location: "/demo/link",
            name: "Link"
        },
        PageInfo {
            location: "/demo/popover",
            name: "Popover"
        },
        PageInfo {
            location: "/demo/accordion",
            name: "Accordion"
        },
        PageInfo {
            location: "/demo/themeselector",
            name: "Theme Selector"
        },
        PageInfo {
            location: "/demo/input",
            name: "Inputs"
        },
        PageInfo {
            location: "/demo/button",
            name: "Button"
        },
        PageInfo {
            location: "/demo/date_picker",
            name: "Date Picker"
        },
        PageInfo {
            location: "/demo/spinner",
            name: "Spinner"
        },
        PageInfo {
            location: "/demo/radio",
            name: "Radio"
        }
    ]
}

#[component(transparent)]
pub fn DemoRoutes() -> impl leptos_router::MatchNestedRoutes + Clone {
    view! {
        <ParentRoute path=path!("demo") view=Outlet>
            <Route path=path!("/navbar") view=crate::demos::navbar::SideNavbarDemoPage/>
            <Route path=path!("/badge") view=crate::demos::badge::BadgeDemoPage/>
            <Route path=path!("/select") view=crate::demos::select::SelectDemoPage/>
            <Route path=path!("/avatar") view=crate::demos::avatar::AvatarDemoPage/>
            <Route path=path!("/divider") view=crate::demos::divider::DividerDemoPage/>
            <Route path=path!("/toggle") view=crate::demos::toggle::ToggleDemoPage/>
            <Route path=path!("/calendar") view=crate::demos::calendar::CalendarDemoPage/>
            <Route path=path!("/dropdown") view=crate::demos::dropdown::DropdownDemoPage/>
            <Route path=path!("/dialog") view=crate::demos::dialog::DialogDemoPage/>
            <Route path=path!("/codeblock") view=crate::demos::codeblock::CodeblockDemoPage/>
            <Route path=path!("/form_input") view=crate::demos::form_input::FormInputDemoPage/>
            <Route path=path!("/modal") view=crate::demos::modal::ModalDemoPage/>
            <Route path=path!("/checkbox") view=crate::demos::checkbox::CheckboxDemoPage/>
            <Route path=path!("/heading") view=crate::demos::heading::HeadingDemoPage/>
            <Route path=path!("/link") view=crate::demos::link::LinkDemoPage/>
            <Route path=path!("/popover") view=crate::demos::popover::PopoverDemoPage/>
            <Route path=path!("/accordion") view=crate::demos::accordion::AccordionDemoPage/>
            <Route path=path!("/themeselector") view=crate::demos::themeselector::ThemeSelectorDemoPage/>
            <Route path=path!("/input") view=crate::demos::input::InputsDemoPage/>
            <Route path=path!("/button") view=crate::demos::button::ButtonDemoPage/>
            <Route path=path!("/date_picker") view=crate::demos::date_picker::DatePickerDemoPage/>
            <Route path=path!("/spinner") view=crate::demos::spinner::SpinnerDemoPage/>
            <Route path=path!("/radio") view=crate::demos::radio::RadioDemoPage/>
        </ParentRoute>
    }
    .into_inner()
    .into_any_nested_route()
}
