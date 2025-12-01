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
pub fn Toggle(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    
    #[prop(into)]
    value: Signal<bool>,
    #[prop(optional, into)]
    label: String,

    #[prop(optional, into)] checked: RwSignal<bool>
) -> impl IntoView {
    
    let input_ref = NodeRef::<html::element::Input>::new();
    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        checked.set(input.checked());
    };
    
    view! {
        <label class=class_list!("relative inline-flex items-center mb-4 cursor-pointer", class)>
            <input
                name=name.get()
                type="checkbox"
                value=move || (!value.get()).to_string()
                checked=value
                node_ref=input_ref
                class="sr-only peer"
                on:change=on_change
            />

            <div
                id=id.get()
                class="w-11 h-6 bg-gray-200 rounded-full peer peer-focus:ring-4 peer-focus:ring-oa-blue dark:peer-focus:ring-oa-blue dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-0.5 after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-oa-blue"
            ></div>
            <span class="ml-3 text-sm font-medium text-gray-900 dark:text-gray-300">{label}</span>
        </label>
    }
}
