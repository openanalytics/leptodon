use leptos::prelude::BindAttribute;
use crate::class_list;
use crate::input_group::GroupItemClassContext;
use crate::util::signals::ComponentRef;
use leptos::either::Either;
use leptos::html;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::MaybeProp;
use leptos::prelude::NodeRef;
use leptos::prelude::NodeRefAttribute;
use leptos::prelude::RwSignal;
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
    Url
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
            Self::Email => "email"
        }
    }
}