use crate::button::{ControlButton, OA_TRANSPARENT_BUTTON_CLASSES};
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use crate::link::Link;
use crate::{class_list, icon};
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::MaybeProp;
use leptos::prelude::{AddAnyAttr, IntoAny, OnAttribute, RwSignal, Set, Update};
use leptos::prelude::{AriaAttributes, Children};
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::slot;
use leptos::view;
use leptos::{IntoView, component};
use leptos_router::components::ToHref;

const SIDEBAR_CLASSES: &str = "fixed top-0 left-0 z-40 w-64 h-screen pt-20 transition-transform bg-white border-r border-gray-200 sm:translate-x-0 dark:bg-gray-800 dark:border-gray-700";

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
        <Link href {..}  class=class_list!(OA_TRANSPARENT_BUTTON_CLASSES, "w-full rounded-lg aria-current-page:bg-oa-gray dark:aria-current-page:bg-gray-700")>
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
fn OALogoLink() -> impl IntoView {
    view! {
        <a href="https://www.openanalytics.eu/" class="flex ms-2 md:me-24">
            <img src="https://www.openanalytics.eu/img/logo.png" class="h-8 me-3" alt="OA Logo"/>
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

#[component]
pub fn SideNavbar(
    children: Children,
    #[prop(optional, default=NavbarEndChildren { children: Box::new(|| ().into_any()) })]
    end: NavbarEndChildren,
    #[prop(optional, default=NavbarLogo { children: Box::new(|| OALogoLink().into_any()) })]
    logo: NavbarLogo,
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
                            } class="sm:hidden" {..}
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
                "sm:hidden fixed top-0 left-0 w-full h-full bg-gray-900/50 z-10",
                ("hidden", move || !visible.get())
            ]
            on:click=move |_| visible.set(false)
        />
        <div class="p-4 mt-16 sm:ml-64 min-h-full">
            {children().into_any()}
        </div>
    }
}
