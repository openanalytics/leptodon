use leptos::prelude::Children;
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
use leptos::prelude::use_context;
use leptos::tachys::html;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
use leptos_use::math::use_or;

use crate::class_list;
use crate::form_input::FormInputContext;
use crate::form_input::PostfixLabelStyle;

#[component]
pub fn Toggle(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: Signal<bool>,
    /// Whether this toggle needs to be ON for a form to be submitted.
    #[prop(optional)] required: bool,
    /// The on/off state of the toggle
    #[prop(optional, into)] checked: RwSignal<bool>,
    /// Labels the toggle
    children: Children
) -> impl IntoView {
    let input_ref = NodeRef::<html::element::Input>::new();
    let on_change = move |_| {
        let input = input_ref.get_untracked().unwrap();
        checked.set(input.checked());
    };
    
    // Form context
    let form_context = use_context::<FormInputContext<String>>();
    let form_required = Signal::from(
        form_context
            .clone()
            .map(|ctx| ctx.required)
            .unwrap_or_default(),
    );
    let required = use_or(required, form_required);

    view! {
        <label class=class_list!("relative inline-flex items-center mb-4 cursor-pointer", class)>
            <input
                name=name.get()
                type="checkbox"
                required=required
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
            <PostfixLabelStyle required=required.get() children />
        </label>
    }
}
