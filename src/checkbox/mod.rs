use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::tachys::html;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};

use crate::class_list;

#[component]
pub fn Checkbox(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,

    /// Update this signal to update the state of the checkbox
    #[prop(into)]
    value: Signal<bool>,
    #[prop(optional, into)] label: String,

    /// Whether or not this element is unreachable by tabbing.
    #[prop(optional, into)]
    disable_tab: bool,
    /// Listen to the checked state.
    #[prop(optional, into)]
    checked: RwSignal<bool>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::element::Input>::new();
    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        checked.set(input.checked());
    };

    view! {
        <label class=class_list!["relative inline-flex items-center cursor-pointer", class]>
            <input
                id=id.get()
                name=name.get()
                type="checkbox"
                value=move || (!value.get()).to_string()
                checked=value
                node_ref=input_ref
                class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600"
                on:change=on_change
                // Non integer values should make tabbing reset to the default behaviour.
                tabindex=move || if disable_tab { "-1" } else { "auto" }
            />
            <span class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">{label}</span>
        </label>
    }
}
