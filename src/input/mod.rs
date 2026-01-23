use crate::button::Button;
use crate::button::ButtonAppearance;
use crate::button_group::ButtonGroup;
use crate::button_group::First;
use crate::button_group::InGroupContext;
use crate::button_group::Last;
use crate::class_list;
use crate::form_input::FormInputContext;
use crate::icon::HideIcon;
use crate::icon::ShowIcon;
use crate::input_group::GroupItemClassContext;
use crate::util::callback::ArcOneCallback;
use crate::util::callback::BoxOneCallback;
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
use leptos::prelude::Show;
use leptos::prelude::Signal;
use leptos::prelude::Update;
use leptos::prelude::use_context;
use leptos::{IntoView, component, view};
use leptos_use::math::use_or;
use std::u32;
use web_sys::KeyboardEvent;
use zxcvbn::Score;
use zxcvbn::zxcvbn;

pub const OA_READONLY_INPUT_CLASSES: &str = "border-0 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";
const OA_INPUT_CLASSES: &str = "shadow-sm bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-primary-600 focus:border-primary-600 block w-full p-2.5 dark:bg-gray-600 dark:border-gray-500 dark:placeholder-gray-400 dark:text-white dark:focus:ring-primary-500 dark:focus:border-primary-500";

#[component]
#[allow(unused)] // Generated propsbuilder is used.
pub fn TextInputConfig(
    #[prop(optional, into)] max_len: MaybeProp<u32>,
    // A min-len of 1 should be provided via the "required" property instead.
    #[prop(optional, into)] min_len: MaybeProp<u32>,
    // Whether or not to trim surrounding whitespace "  My name " -> "My name"
    #[prop(default = true)] trim: bool,
) -> impl IntoView {
    ()
}

#[component]
pub fn TextInput(
    /// Extra classes added to augment the default style.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Will be initialised with a DOM reference to the backing <input> element.
    #[prop(optional)]
    input_ref: NodeRef<html::Input>,
    /// Text above the input that informs the user what to type.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// An input can have different text-based types based on the type of value the user will enter.
    #[prop(optional, into)]
    input_type: Signal<InputType>,
    /// An input can have different modes, useful for mobile devices to bring up the correct virtual keyboard. More fine-grained than type.
    #[prop(optional, into)]
    input_mode: Signal<InputMode>,
    #[prop(default = TextInputConfigProps::builder().build())] text_config: TextInputConfigProps,
    /// Binds to the value of the input, has to be a string.
    #[prop(optional, into)]
    value: RwSignal<String>,
    /// Whether the input is readonly.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Whether the input is required.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
) -> impl IntoView {
    let parser = move |input: String| {
        // Trim first if configured, so we do not count whitespace characters.
        let input = if text_config.trim {
            input.trim()
        } else {
            input.as_str()
        };

        // != input.len()
        // emojis and other special characters are counted as only 1 extra length using the String::chars(&self) iterator.
        let input_len = input.chars().count() as u32;

        if let Some(max_len) = text_config.max_len.get()
            && let Some(min_len) = text_config.min_len.get()
        {
            if input_len > max_len || input_len < min_len {
                return Err(format!("Input Length must be >{min_len} and <{max_len}"));
            }
        } else if let Some(max_len) = text_config.max_len.get() {
            if input_len > max_len {
                return Err(format!("Input Length must be <{max_len}"));
            }
        } else if let Some(min_len) = text_config.min_len.get() {
            if input_len < min_len {
                return Err(format!("Input Length must be >{min_len}"));
            }
        }

        Ok(String::from(input))
    };

    return view! {
        <GenericInput<String, String>
            class
            input_ref
            label
            name
            input_type
            input_mode
            value
            readonly
            required
            placeholder
            parser
        />
    };
}

/// Integrates with dropbox's zxcvbn to create non annoying and actually strong passwords.
#[component]
pub fn PasswordInput(
    /// Extra classes added to augment the default style.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Will be initialised with a DOM reference to the backing <input> element.
    #[prop(optional)]
    input_ref: NodeRef<html::Input>,
    /// Text above the input that informs the user what to type.
    #[prop(optional, into)]
    label: MaybeProp<String>,
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// List of information to disallow in the password, should be all available info on the user.
    /// Attackers frequently try combinations of public/leaked information they have on people.
    #[prop(into)]
    hazards: Vec<String>,
    /// Show strength bar
    #[prop(default = false)]
    show_strength: bool,
    /// Shows an eye-icon-button next to the password input.
    /// Clicking the button toggles the input between plaintext and password mode.
    #[prop(default = false)]
    show_eye: bool,
    /// Binds to the value of the input.
    /// Passwords support all characters.
    #[prop(optional, into)]
    value: RwSignal<String>,
    /// Whether the input is readonly.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Whether the input is required.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
) -> impl IntoView {
    let parser = ArcOneCallback::new(move |input: String| {
        let hazard_strs: Vec<&str> = hazards.iter().map(|s| s.as_ref()).collect();
        let entropy = zxcvbn(input.as_str(), hazard_strs.as_slice());
        if let Some(feedback) = entropy.feedback() {
            return Err(format!("{feedback}"));
        }
        if entropy.score() < Score::Four {
            return Err(format!(
                "Almost strong enough, add another word or a couple symbols."
            ));
        }

        Ok(String::from(input))
    });

    let password_vis = RwSignal::new(false);

    if show_eye {
        view !{
            <ButtonGroup>
                <First slot:first>
                    <GenericInput<String, String>
                        class
                        input_ref
                        label
                        name
                        value
                        readonly
                        required
                        placeholder
                        input_type=Signal::derive(move || { if password_vis.get() { InputType::Text } else { InputType::Password } })
                        parser=parser.clone()
                    />
                </First>
                <Last slot:last>
                    <Button
                        on_click=move |_| {
                            password_vis.update(|mut_vis| *mut_vis = !*mut_vis)
                        }
                        appearance=ButtonAppearance::Secondary
                        icon=Signal::derive(move || {
                            if password_vis.get() { HideIcon() } else { ShowIcon() }
                        })
                    ></Button>
                </Last>
            </ButtonGroup>
        }.into_any()
    } else {
        view! {
            <GenericInput<String, String>
                class
                input_ref
                label
                name
                value
                readonly
                required
                placeholder
                input_type=move || { if password_vis.get() { InputType::Text } else { InputType::Password } }
                parser=parser.clone()
            />
        }.into_any()
    }
}

/// If the input is empty but you supplied **value** then check if you supplied a **format** handler
///
/// The normal layout is as follows:
/// ---------
/// | <Required *><Label>
/// | <Input required>
/// | <ParserFeedback<E>>
/// ---------
///
/// The layout can be changed by providing FormInputContext, generally done via `<FormInput label=... required=true><GenericInput<T,E> ... /></FormInput>`
/// The result is as follows:
/// ----GenericInput<T,E>----
/// | <Input required>
/// ---------
/// The label and parser feedback are then to be rendered by the FormInputContext-providing parent (e.g. [FormInput]).
/// Most useful when you need a ButtonGroup around the input. An example can be seen in the overview under forms, specifically the password input.
#[component]
pub fn GenericInput<T, E>(
    /// Id for the input.
    #[prop(optional, into)]
    id: MaybeProp<String>,
    /// Extra classes added to augment the default style.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// Will be initialised with a DOM reference to the backing <input> element.
    #[prop(optional)]
    input_ref: NodeRef<html::Input>,
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
    /// An inputform_context.required can have different modes, useful for mobile devices to bring up the correct virtual keyboard. More fine-grained than type.
    #[prop(optional, into)]
    input_mode: Signal<InputMode>,
    /// Binds to the value of the input.
    #[prop(optional, into)]
    value: RwSignal<T>,
    /// Maps the user input to [T], not ran on empty inputs unless [required] is true
    #[prop(optional, into)]
    parser: Option<ArcOneCallback<String, Result<T, E>>>,
    /// Formats the value to be shown to the user, only happens when the user indicates they are done inputting.
    /// E.g. via Enter, Escape or leaving the input
    #[prop(optional, into)]
    format: Option<BoxOneCallback<T, String>>,
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
    // let input_ref = NodeRef::<html::Input>::new();
    // comp_ref.load(InputRef { input_ref });
    let group_context = use_context::<GroupItemClassContext>();
    let group_classes = group_context.map(|item| item.class);
    let in_group = use_context::<InGroupContext>().unwrap_or(InGroupContext { in_group: false });

    // Form context
    let form_context = use_context::<FormInputContext<E>>();
    let form_required = Signal::from(
        form_context
            .clone()
            .map(|ctx| ctx.required)
            .unwrap_or_default(),
    );
    let required = use_or(required, form_required);
    let in_form = form_context.is_some();

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

    // Notify form feedback of this input's invalid reason.
    if let Some(form_context) = form_context {
        Effect::new(move || {
            form_context.feedback.set(invalid_reason.get());
        });
    }

    // When the Input loses focus, try parsing the new value
    let on_blur = {
        let try_parse = try_parse.clone();
        move |_| {
            try_parse(true);
        }
    };

    // If there is an error, try parsing on each key to transition in real time to a good state.
    let on_input = {
        let try_parse = try_parse.clone();
        move |_| {
            if invalid_reason.get().is_some() {
                // Formatting should only be done when the user indicates they are done, e.g. by leaving the field (on_blur).
                // Otherwise a format can disrupt the input
                try_parse(false);
            }
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
        true,
    );

    let standalone_input = view! {
        <input id=id.get()
            type=move || input_type.get().as_str()
            inputmode=move || input_mode.get().as_str()
            name={name.get()}
            bind:value=internal_value_signal
            class=class_list![
                ("border-oa-red", move || invalid_reason.get().is_some()),
                if let Some(group_classes) = group_classes { group_classes } else { String::new() },
                if in_group.in_group { "rounded-none border-r-0 !mr-0" } else { "" },
                (OA_READONLY_INPUT_CLASSES, move || readonly.get()),
                (OA_INPUT_CLASSES, move || !readonly.get()),
                class
            ]
            disabled={readonly.get()}
            readonly={readonly.get()}
            node_ref=input_ref
            placeholder={placeholder.get()}
            required={required.get()}
            on:blur=on_blur
            on:input=on_input
            on:keydown={
                let try_parse = try_parse.clone();
                move |key: KeyboardEvent| {
                    if key.code() == "Enter" {
                        try_parse(true);
                    }
                }
            }
        />
        {
            move || {
                if let Some(invalid_reason) = invalid_reason.get() && !in_form {
                    Either::Left(view!{
                        <div class="text-oa-red">{ invalid_reason.to_string() }</div>
                    })
                } else { Either::Right(()) }
            }
        }
    };

    if let Some(label) = label.get()
        && !in_form
    {
        Either::Left(view! {
            <div>
                <label class="block mb-2.5 text-sm font-medium text-heading">
                    <Show
                        when=move || required.get()
                        fallback=|| ()><span class="text-red-500">*</span>
                    </Show> {label}
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
