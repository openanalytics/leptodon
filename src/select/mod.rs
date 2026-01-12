use leptos::children::Children;
use leptos::prelude::BindAttribute;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};

use crate::class_list;
pub const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";

#[component]
pub fn Select(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] default_value: Option<String>,
    /// Whether the select is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    #[prop(optional, into)]
    size: Signal<SelectSize>,
    children: Children,
) -> impl IntoView {
    view! {
        <span class=class_list![
            class
        ]>
            <select
                class=SELECT_CLASSES
                name=name.get()
                bind:value=value
                disabled=disabled
            >
                {children()}
            </select>
        </span>
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum SelectSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SelectSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}
