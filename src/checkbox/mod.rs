use leptos::logging::debug_log;
use leptos::prelude::BindAttribute;
use leptos::prelude::Children;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::use_context;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
use leptos_use::math::use_or;

use crate::class_list;
use crate::form_input::FormInputContext;
use crate::form_input::PostfixLabelStyle;

const CHECKBOX_CLASS: &str = "w-4 h-4 text-blue-600 bg-gray-100 border-gray-300 rounded focus:ring-blue-500 dark:focus:ring-blue-600 dark:ring-offset-gray-800 focus:ring-2 dark:bg-gray-700 dark:border-gray-600";

/// A simple checkmark-box with optional label on the right. 
#[component]
pub fn Checkbox(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] name: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Checked state
    #[prop(optional)]
    checked: RwSignal<bool>,
    /// Required to be checked on for form submission.
    #[prop(optional)]
    required: bool,
    /// Whether or not this element is unreachable by tabbing.
    #[prop(optional, into)]
    disable_tab: bool,
    /// Stops internal click handling
    #[prop(optional)]
    listen_only: bool,
    /// Label goes here.
    children: Children,
) -> impl IntoView {

    // Form context
    let form_context = use_context::<FormInputContext<String>>();
    let form_required = Signal::from(
        form_context
            .clone()
            .map(|ctx| ctx.required)
            .unwrap_or_default(),
    );
    let required = use_or(required, form_required);
    Effect::watch(move || checked.get(), |new, _, _|{
        debug_log!("Checkbox checked state changed to {new}");
    }, false);
    
    view! {
        <label class=class_list!["relative inline-flex items-center cursor-pointer", class] 
            on:click={
                move |ev| {
                    if listen_only {
                        ev.prevent_default();
                    }
                }
            }>
            <input
                id=id.get()
                name=name.get()
                type="checkbox"
                bind:checked=checked
                class=CHECKBOX_CLASS
                required=required
                // Non integer values should make tabbing reset to the default behaviour.
                tabindex=move || if disable_tab { "-1" } else { "auto" }
            />
            <PostfixLabelStyle required=required.get() children />
        </label>
    }
}
