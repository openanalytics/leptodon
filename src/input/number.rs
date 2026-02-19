use leptos::prelude::AddAnyAttr;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use std::fmt::Debug;
use std::fmt::Display;

use crate::input::GenericInput;
use crate::input::InputMode;
use crate::input::InputType;
use attr_docgen::generate_docs;
use leptos::html;
use leptos::prelude::Get;
use leptos::prelude::MaybeProp;
use leptos::prelude::NodeRef;
use leptos::prelude::RwSignal;
use leptos::prelude::Signal;
use leptos::{IntoView, component, view};
use num_traits::Num;
use num_traits::NumCast;
use num_traits::NumOps;

#[generate_docs]
#[component]
#[allow(unused)] // Generated propsbuilder is used.
pub fn NumberInputConfig<NumberType>(
    #[prop(optional, into)] max: MaybeProp<NumberType>,
    #[prop(optional, into)] min: MaybeProp<NumberType>,
    /// Maximum number of digits after the decimal separator.
    /// Warning: the actual decimal places can get rounded sooner when the full precision on the floating type is consumed.
    #[prop(optional, into)]
    decimal_places: MaybeProp<u8>,
    /// Whether or not to trim surrounding whitespace "  My name " -> "My name"
    #[prop(default = true)]
    trim: bool,
) -> impl IntoView
where
    NumberType: Num + NumCast + Clone + NumOps + std::default::Default + Send + Sync + 'static,
{
}

#[generate_docs]
#[component]
pub fn NumberInput<NumberType>(
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
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// An input can have different text-based types based on the type of value the user will enter.
    #[prop(default = InputType::Number.into(), into)]
    input_type: Signal<InputType>,
    /// An input can have different modes, useful for mobile devices to bring up the correct virtual keyboard. More fine-grained than type.
    #[prop(default = InputMode::Numeric.into(), into)]
    input_mode: Signal<InputMode>,
    #[prop(default = NumberInputConfigProps::<NumberType>::builder().build())]
    number_config: NumberInputConfigProps<NumberType>,
    /// Binds to the value of the input, has to be a string.
    #[prop(optional)]
    value: RwSignal<NumberType>,
    /// Whether the input is readonly.
    #[prop(optional, into)]
    readonly: Signal<bool>,
    /// Whether the input is required.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
) -> impl IntoView
where
    NumberType: Num
        + NumCast
        + NumOps
        + Display
        + Debug
        + Intness
        + Signedness
        + PartialOrd
        + Clone
        + std::default::Default
        + Send
        + Sync
        + 'static,
{
    let max = if let Some(max) = number_config.max.get() {
        max.to_string()
    } else {
        String::default() // empty string to omit the property
    };
    let min = if let Some(min) = number_config.min.get() {
        min.to_string()
    } else {
        String::default() // empty string to omit the property
    };
    let parser = move |input: String| {
        // Trim first if configured, so we do not count whitespace characters.
        let input = if number_config.trim {
            input.trim()
        } else {
            input.as_str()
        };

        let parsed_value = match NumberType::from_str_radix(input, 10) {
            Ok(parsed_value) => parsed_value,
            Err(_) => {
                if NumberType::is_int() {
                    if NumberType::is_signed() {
                        return Err("Please input an integer.".to_string());
                    } else {
                        return Err("Please input a positive integer.".to_string());
                    }
                } else if !NumberType::is_int() {
                    return Err("Please input a decimal.".to_string());
                }
                return Err("Please input a number".to_string());
            }
        };

        if let Some(max) = number_config.max.get()
            && let Some(min) = number_config.min.get()
        {
            if parsed_value > max || parsed_value < min {
                return Err(format!("Value must be ≥{min} and ≤{max}"));
            }
        } else if let Some(max) = number_config.max.get() {
            if parsed_value > max {
                return Err(format!("Value must be ≤{max}"));
            }
        } else if let Some(min) = number_config.min.get()
            && parsed_value < min
        {
            return Err(format!("Value must be ≥{min}"));
        }

        Ok(parsed_value)
    };

    let format = move |input: NumberType| input.to_string();
    // Html input step also decides whether the user can enter decimal places and how many.
    let step = if !NumberType::is_int() {
        if let Some(places) = number_config.decimal_places.get() {
            if places == 0 {
                "1".to_string()
            } else {
                let zeroes = "0".repeat((places - 1) as usize);
                format!("0.{}1", zeroes)
            }
        } else {
            "any".to_string()
        }
    } else {
        "1".to_string()
    };

    view! {
        <GenericInput<NumberType, String>
            id
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
            format
            {..}
            step=step
            min=min
            max=max
        />
    }
}

macro_rules! impl_signedness {
    ($T:ty, $signed:expr) => {
        impl Signedness for $T {
            #[inline]
            fn is_signed() -> bool {
                $signed
            }
        }
    };
}

pub trait Signedness {
    fn is_signed() -> bool;
}

impl_signedness!(u8, false);
impl_signedness!(u16, false);
impl_signedness!(u32, false);
impl_signedness!(u64, false);
impl_signedness!(u128, false);

impl_signedness!(i8, true);
impl_signedness!(i16, true);
impl_signedness!(i32, true);
impl_signedness!(i64, true);
impl_signedness!(i128, true);
impl_signedness!(f32, true);
impl_signedness!(f64, true);

pub trait Intness {
    fn is_int() -> bool;
}

macro_rules! impl_intness {
    ($T:ty, $int:expr) => {
        impl Intness for $T {
            #[inline]
            fn is_int() -> bool {
                $int
            }
        }
    };
}

impl_intness!(u8, true);
impl_intness!(u16, true);
impl_intness!(u32, true);
impl_intness!(u64, true);
impl_intness!(u128, true);

impl_intness!(i8, true);
impl_intness!(i16, true);
impl_intness!(i32, true);
impl_intness!(i64, true);
impl_intness!(i128, true);
impl_intness!(f32, false);
impl_intness!(f64, false);
