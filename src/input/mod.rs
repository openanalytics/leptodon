use crate::class_list;
use crate::input_group::GroupItemClassContext;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
use crate::util::optional_prop::OptionalProp;
use crate::util::signals::ComponentRef;
use leptos::either::Either;
use leptos::html;
use leptos::prelude::BindAttribute;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::IntoAny;
use leptos::prelude::MaybeProp;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::OnAttribute;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Signal;
use leptos::prelude::use_context;
use leptos::{IntoView, component, view};

const OA_READONLY_INPUT_CLASSES: &str = "border-0 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";
const OA_INPUT_CLASSES: &str = "shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";

#[component]
pub fn Input(
    /// Extra classes added to augment the default style.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional)] comp_ref: ComponentRef<InputRef>,
    /// Text above the input that informs the user what to type.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// An input can have different text-based types based on the type of value the user will enter.
    #[prop(optional, into)]
    input_type: Signal<InputType>,
    /// An input can have different modes, useful for mobile devices to bring up the correct virtual keyboard. More fine-grained than type.
    #[prop(optional, into)]
    input_mode: Signal<InputMode>,
    /// Binds to the value of the input, has to be a string.
    #[prop(optional, into)]
    value: RwSignal<String>,
    /// Whether the input is readonly.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
) -> impl IntoView {
    let input_ref = NodeRef::<html::Input>::new();
    comp_ref.load(InputRef { input_ref });
    let group_context = use_context::<GroupItemClassContext>();
    let group_classes = group_context.map(|item| item.class);

    let standalone_input = view! {
        <input type=input_type.get().as_str()
            inputmode=input_mode.get().as_str()
            name={name.get()}
            bind:value=value
            class=class_list![
                if let Some(group_classes) = group_classes { group_classes } else { String::new() },
                if readonly.get() {
                    OA_READONLY_INPUT_CLASSES
                } else {
                    OA_INPUT_CLASSES
                },
                class
            ]
            disabled={readonly.get()}
            readonly={readonly.get()}
            node_ref=input_ref
            placeholder={placeholder.get()} required=""/>
    };

    if let Some(label) = label.get() {
        Either::Left(view! {
            <div>
                <label class="block mb-2.5 text-sm font-medium text-heading">
                    {label}
                    {standalone_input}
                </label>
            </div>
        })
    } else {
        Either::Right(standalone_input)
    }
}

#[component]
pub fn GenericInput<T, E>(
    /// Id for the input.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes added to augment the default style.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional)] comp_ref: ComponentRef<InputRef>,
    /// Text above the input that informs the user what to type.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// An input can have different text-based types based on the type of value the user will enter.
    #[prop(optional, into)]
    input_type: Signal<InputType>,
    /// An input can have different modes, useful for mobile devices to bring up the correct virtual keyboard. More fine-grained than type.
    #[prop(optional, into)]
    input_mode: Signal<InputMode>,
    /// Binds to the value of the input, has to be a string.
    #[prop(optional, into)]
    value: RwSignal<T>,
    /// Maps the user input to [T], not ran on empty inputs unless [required] is true
    #[prop(optional, into)]
    parser: OptionalProp<ArcOneCallback<String, Result<T, E>>>,
    /// Formats the value to be shown to the user, only happens when the user indicates they are done inputting.
    /// E.g. via Enter, Escape or leaving the input
    #[prop(optional, into)]
    format: OptionalProp<BoxOneCallback<T, String>>,
    /// Whether the input is required.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Whether the input is readonly.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
) -> impl IntoView
where
    T: Clone + Default + Sync + Send + 'static,
    E: Clone + Send + Sync + std::fmt::Display + 'static,
{
    let input_ref = NodeRef::<html::Input>::new();
    comp_ref.load(InputRef { input_ref });
    let group_context = use_context::<GroupItemClassContext>();
    let group_classes = group_context.map(|item| item.class);

    // String value bound to <input>
    let internal_value_signal = RwSignal::new("".to_string());
    let invalid_reason = RwSignal::new(None);

    let try_parse = {
        let parser = parser.clone();
        move |should_format: bool| {
            let internal_value = internal_value_signal.get();
            if let Some(parser) = parser.as_ref()
                && (!internal_value.is_empty() || required.get())
            {
                let parsed_value = parser(internal_value);
                match parsed_value {
                    Ok(parsed_success) => {
                        // Changing the parsed value causes a format
                        // the blur handler will want to format while input handling does not.
                        if should_format {
                            value.set(parsed_success);
                        }
                        invalid_reason.set(None);
                    }
                    Err(err) => {
                        invalid_reason.set(Some(err));
                    }
                }
            } else if internal_value.is_empty() && !required.get() {
                invalid_reason.set(None);
            }
        }
    };

    // When the Input loses focus, try parsing the new value
    let on_blur = {
        let try_parse = try_parse.clone();
        move |_| {
            try_parse(true);
        }
    };

    // If there is an error, try parsing on each key to transition in real time to a good state.
    let on_input = move |_| {
        if invalid_reason.get().is_some() {
            // Formatting should only be done when the user indicates they are done, e.g. by leaving the field (on_blur).
            // Otherwise a format can disrupt the input
            try_parse(false);
        }
    };

    // On a successfull parsing or change of value this function formats the input field.
    Effect::watch(
        move || value.get(),
        move |value, _prev_value, _| {
            if let Some(format) = format.as_ref() {
                internal_value_signal.set(format(value.clone()));
            }
        },
        false,
    );

    let standalone_input = view! {
        <input id=id.get() type=input_type.get().as_str()
            inputmode=input_mode.get().as_str()
            name={name.get()}
            bind:value=internal_value_signal
            class=class_list![
                ("border-oa-red", move || invalid_reason.get().is_some()),
                if let Some(group_classes) = group_classes { group_classes } else { String::new() },
                (OA_READONLY_INPUT_CLASSES, move || readonly.get()),
                (OA_INPUT_CLASSES, move || !readonly.get()),
                class
            ]
            disabled={readonly.get()}
            readonly={readonly.get()}
            node_ref=input_ref
            placeholder={placeholder.get()}
            required=""
            on:blur=on_blur
            on:input=on_input
        />
        {
            move || {
                if let Some(invalid_reason) = invalid_reason.get() {
                    Either::Left(view!{
                        <div class="text-oa-red">{ invalid_reason.to_string() }</div>
                    })
                } else { Either::Right(()) }
            }
        }
    };

    if let Some(label) = label.get() {
        Either::Left(view! {
            <div>
                <label class="block mb-2.5 text-sm font-medium text-heading">
                    {label}{
                        if required.get() {
                            view!{ <span class="color-red-500">*</span> }.into_any()
                        } else {
                            view!{}.into_any()
                        }
                    }
                    {standalone_input}
                </label>
            </div>
        })
    } else {
        Either::Right(standalone_input)
    }
}

#[derive(Clone)]
pub struct InputRef {
    pub input_ref: NodeRef<html::Input>,
}

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Tel,
    Url,
    Email,
    Time,
    Date,
    DatetimeLocal,
    Month,
    Week,
    Number,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Time => "time",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
            Self::Number => "number",
        }
    }
}

#[derive(Default, Clone)]
pub enum InputMode {
    #[default]
    Text,
    /// Decimal numbers
    Decimal,
    /// Only digits
    Numeric,
    /// Telephone number keypad
    Tel,
    /// Submit button may become "search" button
    Search,
    /// Standard text with an @ key
    Email,
    /// Standard keyboard with perhaps a / key
    Url,
}

impl InputMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Decimal => "decimal",
            Self::Numeric => "numeric",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
        }
    }
}
