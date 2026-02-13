use attr_docgen::generate_docs;
use leptos::html::Input;
use leptos::logging::debug_log;
use leptos::oco::Oco;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::For;
use leptos::prelude::Get;
use leptos::prelude::GetUntracked;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Update;
use leptos::tachys::html::node_ref::NodeRefContainer;
use leptos::{IntoView, component, view};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

use crate::class_list;
use crate::class_list::reactive_class::MaybeReactiveClass;
use crate::form_input::Label;

// The selection-indicating orb's style
const RADIO_OPTION_CLASSES: &str = "w-4 h-4 text-neutral-primary border-default-medium rounded-full checked:border-oa-blue focus:ring-2 focus:outline-none focus:ring-brand-subtle border border-default dark:border-gray-700 appearance-none";
// Label right of orb
const RADIO_OPTION_LABEL_CLASSES: &str =
    "w-full py-3 select-none ms-2 text-sm font-medium text-gray-900 dark:text-gray-100";

const RADIO_LIST_GROUP_CLASSES: &str =
    "border border-default dark:border-gray-700 rounded-lg shadow-sm";

/// Methods for radio option
pub trait RadioOption: Display {
    /// Value sent with the surrounding Form.
    fn value(&self) -> Oco<'static, str>;
}

#[generate_docs]
/// A list of options where the user can choose at most one.
#[component]
pub fn Radio<T>(
    /// Html id
    #[prop(optional, into)] id: Oco<'static, str>,
    /// Name identifier for form submission
    #[prop(into)] name: Oco<'static, str>,
    /// Extra style classes
    #[prop(optional, into)] class: MaybeReactiveClass,
    /// Label for the radio-button menu.
    #[prop(optional, into)] label: String,
    /// Whether selecting an option is required for form submission.
    #[prop(optional, into)] required: bool,
    /// Value of the selected element
    #[prop(optional, into)] selected: RwSignal<Option<T>>,
    /// Available radio options
    #[prop(optional, into)] options: RwSignal<Vec<T>>,
) -> impl IntoView
where
    T: RadioOption + Clone + Eq + Hash + Send + Sync + 'static,
{
    let checked_input = NodeRef::<Input>::new();
    let fields: RwSignal<HashMap<T, NodeRef<Input>>> = RwSignal::new(HashMap::new());
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
    Effect::watch(
        move || selected.get(),
        move |new, old, _| {
            if let Some(prev_input) = checked_input.get_untracked()
                && Some(new) != old
            {
                prev_input.set_checked(false);
                if let Some(new) = new {
                    if let Some(input_ref) = fields.get_untracked().get(new)
                        && let Some(input) = input_ref.get_untracked()
                    {
                        input.set_checked(true);
                        checked_input.load(&input)
                    }
                } else {
                    // unset
                }
            }
        },
        false,
    );
    view! {
        <div class=class_list!(class)>
            <Label label=label required>
                <ul class=class_list!(
                    "w-48 bg-oa-gray dark:bg-gray-700 flex flex-col gap-px overflow-hidden",
                    RADIO_LIST_GROUP_CLASSES  
                )>
                    <For
                        each=move || options.get()
                        key=|option: &T| {
                            option.clone()
                        }
                        children=move |option| {
                            let id = format!("{}-{}", id, option.value());
                            let node_ref = NodeRef::<Input>::new();
                            fields.update(|fields| {
                                fields.insert(option.clone(), node_ref);
                            });
                            view! {
                                <li class="w-full bg-white dark:bg-gray-900"
                                    on:click=move |_| {
                                        if let Some(input) = node_ref.get() {
                                            // Uncheck prev checked input
                                            if let Some(checked_input) = checked_input.get() {
                                                checked_input.set_checked(false);
                                            }
                                            // Check current input
                                            checked_input.load(&input);
                                            // Store current input as current-checked
                                            input.set_checked(true);
                                            // Update selected option for outside observation.
                                            selected.set(Some(option.clone()));
                                        }
                                    }
                                >
                                    <label class="flex items-center ps-3">
                                        <input
                                            id=id.clone()
                                            class=RADIO_OPTION_CLASSES
                                            value=option.value()
                                            name=name.clone()
                                            type="radio"
                                            node_ref=node_ref
                                            required=required
                                            on:change={
                                                let option = option.clone();
                                                move |_| {
                                                    debug_log!("change handler");

                                                    if let Some(input) = node_ref.get() && input.checked() {
                                                        let select = option.clone();

                                                        debug_log!("selecting radio opt {select}");
                                                        // Store current input as current-checked
                                                        checked_input.load(&input);
                                                        // Update selected option for outside observation.
                                                        selected.set(Some(select));
                                                    }
                                                }
                                            }
                                            // Initial checked, becomes unused after programatically checking elements.
                                            checked={
                                                let option = option.clone();
                                                move || selected.get() == Some(option.clone())
                                            }
                                        />
                                        <span class=RADIO_OPTION_LABEL_CLASSES>{option.to_string()}</span>
                                    </label>
                                </li>
                            }
                        }
                    />
                </ul>
            </Label>
        </div>
    }
}

impl RadioOption for u8 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for u16 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for u32 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for u64 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for u128 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}

impl RadioOption for i8 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for i16 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for i32 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for i64 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for i128 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}

impl RadioOption for f32 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
impl RadioOption for f64 {
    fn value(&self) -> Oco<'static, str> {
        Oco::Owned(self.to_string())
    }
}
