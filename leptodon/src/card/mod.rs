use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::reactive::traits::Get;
use leptos::reactive::wrappers::read::Signal;
use leptos::{IntoView, children::Children, component, view};

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div class="max-w-md mx-auto py-2">
            <div class="bg-white dark:bg-gray-900 border border-gray-200 dark:border-gray-800 rounded-xl shadow-sm overflow-hidden transition-colors">
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn CardSection(
    #[prop(into, optional)] class: MaybeReactiveClass,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list![class, "px-5 py-3 space-y-3"]>
            {children()}
        </div>
    }
}

#[component]
pub fn CardField(#[prop(into)] name: Signal<String>, children: Children) -> impl IntoView {
    view! {
        <div>
            <span class="text-xs font-semibold uppercase tracking-wider text-gray-500 dark:text-gray-400">{move || name.get()}</span>
            {children()}
        </div>
    }
}
