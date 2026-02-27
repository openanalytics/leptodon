// Leptodon
//
// Copyright (C) 2025-2026 Open Analytics NV
//
// ===========================================================================
//
// This program is free software: you can redistribute it and/or modify it
// under the terms of the Apache License as published by The Apache Software
// Foundation, either version 2 of the License, or (at your option) any later
// version.
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
// FOR A PARTICULAR PURPOSE. See the Apache License for more details.
//
// You should have received a copy of the Apache License along with this program.
// If not, see <http://www.apache.org/licenses/>
use attr_docgen::generate_docs;
use leptos::logging::debug_log;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
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
use crate::form_input::Label;
use crate::radio::RadioOption;
pub const SELECT_CLASSES: &str = "bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500";

#[generate_docs]
#[component]
pub fn Select<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] label: MaybeProp<String>,
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
            debug_log!(
                "Some_selected, {:?} {:?}",
                new.as_ref().map(|t| t.value()),
                old.map(|t| t.as_ref().map(|tt| tt.value()))
            );
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
            disabled
            options
        />
    }
}

#[generate_docs]
#[component]
pub fn MaybeSelect<T>(
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Label obove the select.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// Shown as default option, this option cannot be submitted while this select is required.
    #[prop(default = " -- select an option -- ".to_string(), into)]
    placeholder: String,
    /// Shown as extra option when the select is not required.
    #[prop(default = " -- none -- ".to_string(), into)]
    none_option: String,
    /// Whether a value needs to be selected before a form surrounding this select can be submitted.
    #[prop(optional, into)]
    required: bool,
    /// Selected element, should be an element of [options]
    #[prop(into)]
    selected: RwSignal<Option<T>>,
    /// Whether the select is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Possible options of this select.
    #[prop(into)]
    options: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: RadioOption + Clone + Eq + Hash + Send + Sync + 'static,
{
    // Unset the selected elem when it is not part of the new [options]
    Effect::watch(
        move || options.get(),
        move |new, _, _| {
            if let Some(selected_value) = selected.get_untracked()
                && !new.contains(&selected_value)
            {
                selected.set(None);
            }
        },
        false,
    );
    let node_ref = NodeRef::<leptos::html::Select>::new();

    // Unset the selected elem when it is not part of the new [options]
    Effect::watch(
        move || selected.get(),
        move |new, old, _| {
            if Some(new) != old
                && let Some(select) = node_ref.get_untracked()
            {
                // Firefox does not want to select disabled values with the selected attribute.
                // Workaround..
                if let Some(new) = new {
                    select.set_value(new.value().as_str());
                } else {
                    select.set_value("");
                }
            }
        },
        false,
    );

    view! {
        <span class=class_list![
            class
        ]>
            <Label required label>
                <select
                    id=id.get()
                    class=SELECT_CLASSES
                    name=name.get()
                    node_ref=node_ref
                    on:input=move |_| {
                        if let Some(input) = node_ref.get() && !input.value().is_empty() {
                            let selected_value = input.value();
                            if let Some(matched_option) = options.get().iter().find(|opt| opt.value() == selected_value) {
                                debug_log!("selecting opt {matched_option}");
                                selected.set(Some(matched_option.clone()));
                            } else {
                                debug_log!("Could not match {} to any option", selected_value);
                            }
                        } else if !required {
                            selected.set(None);
                            debug_log!("Nothing was selected for {:?}", node_ref.get());
                        }
                    }
                    required=required
                    disabled=disabled
                >
                    <Show
                        when=move || { !options.get().is_empty() }
                        fallback=|| view! { <option disabled=true selected=true>No options</option> }
                    >
                       {
                           let placeholder = placeholder.clone();
                           view! {
                               <Show
                                    when=move || { required }
                                    fallback=|| ().into_any()
                                >
                                    // Placeholder option
                                    <option
                                        value=""
                                        disabled=true
                                        selected=move || { selected.get().is_none() && required }
                                    >{ placeholder.clone() }</option>
                                </Show>
                           }
                       }
                    </Show>
                    <Show
                        when=move || { !required }
                        fallback=|| ().into_any()
                    >
                        // None option, shown when the select is not required.
                        <option
                            value=""
                            selected=move || { selected.get().is_none() && !required }
                        >{ none_option.clone() }</option>
                    </Show>
                    <For
                        each=move || options.get()
                        key=|option| { option.clone() }
                        children=move |option| {
                            view! {
                                <option
                                    value=option.value()
                                    selected=move || { selected.get() == Some(option.clone()) }
                                >{ option.to_string() }</option>
                            }
                        }
                    >
                    </For>
                </select>
            </Label>
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
