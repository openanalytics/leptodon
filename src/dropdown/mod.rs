use crate::class_list;
use crate::icon::Icon;
use crate::icon::icon_data::IconRef;
use leptos::either::Either;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::{IntoView, component, view};
use web_sys::MouseEvent;

const DROPDOWN_STYLE: &str = "absolute translate-y-1 z-10 bg-white divide-y divide-gray-100 rounded-lg shadow w-44 dark:bg-gray-700";
const DROPDOWN_LIST_STYLE: &str = "py-2 text-sm text-gray-700 dark:text-gray-200";
const DROPDOWN_ITEM_STYLE: &str =
    "block px-4 py-2 hover:bg-gray-100 dark:hover:bg-gray-600 dark:hover:text-white";

#[component]
pub fn Dropdown(
    /// Write true to display the dropdown, false to hide.
    #[prop(into)]
    is_visible: ReadSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list![
            DROPDOWN_STYLE, ("hidden", move || !*is_visible.read())
        ]>
            <ul class=DROPDOWN_LIST_STYLE aria-labelledby="dropdownDefaultButton">
                {children()}
            </ul>
        </div>
    }
}

pub type AutoClose = bool;
pub type SetVisibleCallback = WriteSignal<bool>;

/// An item in a dropdown menu.
/// This item can take contexts via [leptus::context::provide_context]:
///   context 1: [AutoClose] to indicate wether clicking this item should close the dropdown.
///   context 2: [SetVisibleCallback] to communicate back to the context-providing component (e.g. [crate::button::DropdownButton]) that it should hide its dropdown.
#[component]
pub fn DropdownItem(
    /// Extra classes appened to the item's default style
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Icon shown next to the label.
    #[prop(optional, into)]
    icon: MaybeProp<IconRef>,
    /// Label for the dropdown item
    #[prop(into)]
    label: String,
    /// Items in the dropdown must have an action:
    mut on_click: impl FnMut(MouseEvent) + 'static,
) -> impl IntoView {
    let auto_close = use_context::<AutoClose>().unwrap_or_default();
    let set_visible = use_context::<SetVisibleCallback>();

    view! {
        <li on:click=move |e| {
            // Call the provided click handler
            on_click(e);

            log!("{auto_close:?}");
            log!("{set_visible:?}");
            // Handle the closing of the dropdown
            if let Some(set_visible) = set_visible && auto_close {
                // close -> not visible
                *set_visible.write() = false;
            }
        }>
            <a href="#" class=class_list![DROPDOWN_ITEM_STYLE, class]>
                {if let Some(icon) = icon.get() {
                    Either::Left(view! { <Icon icon=icon/> })
                } else {
                    Either::Right(())
                }}
                { label }
            </a>
        </li>
    }
}
