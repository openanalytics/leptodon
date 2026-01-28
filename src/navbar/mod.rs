use leptos::logging::debug_log;
use leptos::prelude::Get;
use leptos::prelude::{AddAnyAttr, IntoAny, OnAttribute, RwSignal, Set, Update};
use leptos::prelude::{AriaAttributes, Children};
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::view;
use leptos::{IntoView, component};
use leptos_router::components::ToHref;
// Do not remove until leptos is upgraded above 0.8.14
use crate::avatar::Avatar;
use crate::button::{
    ButtonAppearance, ButtonShape, ControlButton, DropdownButton, DropdownButtonChildren,
    OA_TRANSPARENT_BUTTON_CLASSES,
};
use crate::dropdown::{AlignmentAnchor, DropdownItem};
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::link::Link;
use crate::{class_list, icon};
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;

const SIDEBAR_CLASSES: &str = "fixed top-0 left-0 z-40 w-64 h-screen pt-20 transition-transform bg-white border-r border-gray-200 sm:translate-x-0 dark:bg-gray-800 dark:border-gray-700";

#[component]
pub fn SideBarLink<H>(icon: IconRef, href: H, children: Children) -> impl IntoView
where
    H: ToHref + Send + Sync + 'static,
{
    view! {
        <Link href {..}  class=class_list!(OA_TRANSPARENT_BUTTON_CLASSES, "w-full rounded-lg aria-current-page:bg-oa-gray dark:aria-current-page:bg-gray-700")>
           <Icon icon=icon />
           {children()}
        </Link>
    }
}

#[component]
pub fn SideNavbar(children: Children) -> impl IntoView {
    let visible = RwSignal::new(false);

    view! {
        <nav class="fixed top-0 z-[500] w-full bg-white border-b border-gray-200 dark:bg-gray-800 dark:border-gray-700">
            <div class="px-3 py-3 lg:px-5 lg:pl-3">
                <div class="flex items-center justify-between">
                    <div class="flex items-center justify-start rtl:justify-end">
                        <ControlButton icon=icon::HamburgerIcon() on_click=move |_| {
                                visible.update(|is_visible| *is_visible = !*is_visible);
                            } class="sm:hidden" {..}
                            data-drawer-target="logo-sidebar" data-drawer-toggle="logo-sidebar" aria-controls="logo-sidebar"
                        />
                        <a href="https://www.openanalytics.eu/" class="flex ms-2 md:me-24">
                            <img src="https://www.openanalytics.eu/img/logo.png" class="h-8 me-3" alt="FlowBite Logo"/>
                        </a>
                    </div>
                    <div class="flex items-center">
                        <div class="flex items-center ms-3">
                            <DropdownButton alignment=AlignmentAnchor::BottomRight appearance=ButtonAppearance::Minimal shape=ButtonShape::Circular>
                                <DropdownButtonChildren slot:button_children>
                                    <Avatar src="https://flowbite.com/docs/images/people/profile-picture-5.jpg" />
                                </DropdownButtonChildren>
                                <DropdownItem label="Dashboard" on_click=move |e| { debug_log!("{e:?}"); } />
                                <DropdownItem label="Settings" on_click=move |e| { debug_log!("{e:?}"); } />
                                <DropdownItem label="Logout" on_click=move |e| { debug_log!("{e:?}"); } />
                            </DropdownButton>
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
                    <li><SideBarLink href="#" icon=icon::CalendarIcon()>Calendar</SideBarLink></li>
                    <li><SideBarLink href="users" icon=icon::UserIcon()>Users</SideBarLink></li>
                    <li><SideBarLink href="forms" icon=icon::PendingApprovalIcon()>Users</SideBarLink></li>
                    <li><SideBarLink href="security" icon=icon::LockOpenIcon()>Security</SideBarLink></li>
                </ul>
            </div>
        </aside>
        <div class=class_list![
                "sm:hidden fixed top-0 left-0 w-full h-full bg-gray-900/50 z-10",
                ("hidden", move || !visible.get())
            ]
            on:click=move |_| visible.set(false)
        />
        <div class="p-4 sm:ml-64 h-full">
            <div class="p-4 border-2 border-gray-200 border-dashed rounded-lg dark:border-gray-700 mt-14">
                {children().into_any()}
            </div>
        </div>
    }
}
