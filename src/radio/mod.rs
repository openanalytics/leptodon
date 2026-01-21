use leptos::html::Input;
use leptos::logging::debug_log;
use leptos::oco::Oco;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::GlobalOnAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use std::fmt::Display;
use std::hash::Hash;
use std::sync::Arc;

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::util::shared_id::shared_id;

// The selection-indicating orb's style
const RADIO_OPTION_CLASSES: &'static str = "w-4 h-4 text-neutral-primary border-default-medium bg-neutral-secondary-medium rounded-full checked:border-brand focus:ring-2 focus:outline-none focus:ring-brand-subtle border border-default appearance-none";
// Label right of orb
const RADIO_OPTION_LABEL_CLASSES: &'static str =
    "w-full py-3 select-none ms-2 text-sm font-medium text-heading";

const RADIO_LIST_GROUP_CLASSES: &'static str = "border border-default rounded-lg shadow-sm";

/// Methods for radio option
pub trait RadioOption: Display {
    /// Value sent with the surrounding Form.
    fn value(&self) -> Oco<'static, str>;
}

/// A list of options where the user can choose at most one.
#[component]
pub fn Radio<T>(
    #[prop(into)] name: Oco<'static, str>,
    #[prop(optional, into)] class: MaybeReactiveClass,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] required: bool,
    #[prop(default = RadioAppearance::default())] appearance: RadioAppearance,
    #[prop(optional, into)] selected: RwSignal<Option<T>>,
    #[prop(optional, into)] options: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: RadioOption + Clone + Eq + Hash + Send + Sync + 'static,
{
    let id = Arc::new(shared_id());
    view! {
        <div class=class_list!(class)> 
            {if !label.is_empty() {
                view!{
                    <h3 class="mb-4 font-semibold text-heading">{label}</h3>
                }.into_any()
            } else {
                ().into_any()
            }}
    
            <ul class=class_list!(
                "w-48 bg-neutral-primary-soft",
                match appearance {
                    RadioAppearance::ListGroup => RADIO_LIST_GROUP_CLASSES,
                    RadioAppearance::Minimal => "",
                }
            )>
                <For
                    each=move || options.get()
                    key=|option: &T| {
                        option.clone()
                    }
                    children=move |option| {
                        let id = format!("{}-{}", id, option.value());
                        let node_ref= NodeRef::<Input>::new();
                        view! {
                            <li class="w-full border-b border-default"
                                on:click=move |_| {
                                    if let Some(input) = node_ref.get() {
                                        // Simply changes the input checked state, triggering its onchange handler.
                                        input.set_checked(true);
                                    }
                                }
                            >
                                <div class="flex items-center ps-3">
                                    <input
                                        id=id.clone()
                                        value=option.value()
                                        name=name.clone()
                                        type="radio"
                                        node_ref=node_ref
                                        onchange={
                                            let option = option.clone();
                                            move || {
                                                if let Some(input) = node_ref.get() && input.checked() {
                                                    let select = option.clone();
    
                                                    debug_log!("selecting radio opt {select}");
                                                    // Updating selected is not handled via the click handler in order to support the standard radio-keyboard navigation.
                                                    selected.set(Some(select));
                                                }
                                            }
                                        }
                                        required=required
                                        class=RADIO_OPTION_CLASSES
                                    />
                                    <label for=id class=RADIO_OPTION_LABEL_CLASSES>{option.to_string()}</label>
                                </div>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}

#[derive(Default)]
pub enum RadioAppearance {
    #[default]
    ListGroup,
    Minimal,
}
