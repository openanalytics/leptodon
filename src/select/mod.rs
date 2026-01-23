use leptos::logging::debug_log;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalOnAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::Memo;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Show;
use leptos::{
    IntoView, component,
    prelude::{MaybeProp, Signal},
    view,
};
use std::hash::Hash;

use crate::class_list;
use crate::radio::RadioOption;
pub const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";

#[component]
pub fn Select<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: bool,
    #[prop(into)] selected: RwSignal<T>,
    // TODO:
    // #[prop(optional, into)] default_value: Option<String>,
    /// Whether the select is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    // TODO: Implement size
    #[prop(into)] options: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: RadioOption + Clone + Eq + Hash + Send + Sync + 'static,
{
    let some_selected = RwSignal::new(Some(selected.get_untracked()));

    // Downflow
    Effect::watch(
        move || selected.get(),
        move |new, old, _| {
            if Some(new) != old {
                some_selected.set(Some(new.clone()));
            }
        },
        false,
    );

    // Upflow
    Effect::watch(
        move || some_selected.get(),
        move |new, old, _| {
            if let Some(new_some_selected) = new
                && Some(new) != old
            {
                selected.set(new_some_selected.clone());
            }
        },
        false,
    );

    view! {
        <MaybeSelect
            class
            name
            label
            required
            selected=some_selected
            options
        />
    }
}

#[component]
pub fn MaybeSelect<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] label: String,
    #[prop(default = " -- select an option -- ".to_string(), into)] placeholder: String,
    #[prop(optional, into)] required: bool,
    #[prop(into)] selected: RwSignal<Option<T>>,
    // TODO:
    // #[prop(optional, into)] default_value: Option<String>,
    /// Whether the select is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    // TODO: Implement size
    #[prop(into)] options: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: RadioOption + Clone + Eq + Hash + Send + Sync + 'static,
{
    let node_ref = NodeRef::<leptos::html::Select>::new();
    view! {
        <span class=class_list![
            class
        ]>
            {if !label.is_empty() {
                view!{
                    <h3 class="mb-1 mt-3 font-semibold text-heading">
                    <Show
                        when=move || required
                        fallback=|| ()><span class="text-red-500">*</span>
                    </Show> {label}</h3>
                }.into_any()
            } else {
                ().into_any()
            }}

            <select
                class=SELECT_CLASSES
                name=name.get()
                node_ref=node_ref
                onchange=move || {
                    if let Some(input) = node_ref.get() && !input.value().is_empty() {
                        let selected_value = input.value();
                        if let Some(matched_option) = options.get().iter().find(|opt| opt.value() == selected_value) {
                            debug_log!("selecting radio opt {matched_option}");
                            selected.set(Some(matched_option.clone()));
                        }
                    }
                }
                required=required
                disabled=disabled
            >
                <Show
                    when=move || { options.get().len() > 0 }
                    fallback=|| view! { <option disabled=true selected=true>No options</option> }
                >
                    <option
                        value=""
                        disabled=true
                        selected=move || { selected.get().is_none() }
                    >{ placeholder.clone() }</option>
                </Show>
                <For
                    each=move || options.get()
                    key=|option| { option.clone() }
                    children=move |option| {
                        view! {
                            <option
                                value=option.value()
                                selected={ selected.get() == Some(option) }
                            >{ option.to_string() }</option>
                        }
                    }
                >

                </For>

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
