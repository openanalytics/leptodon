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
use crate::input::GenericInput;
use crate::input::InputMode;
use crate::input::InputType;
use leptodon_proc_macros::generate_docs;
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
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Rem;

#[generate_docs]
#[component]
#[allow(unused)] // Generated propsbuilder is used.
pub fn NumberInputConfig<NumberType>(
    #[prop(optional, into)] max: MaybeProp<NumberType>,
    #[prop(optional, into)] min: MaybeProp<NumberType>,
    /// Currently unchecked for floats as they're too imprecise.
    /// Stepsize of the number input, (used as increment/decrement when using up/down arrows).
    #[prop(optional, into)]
    step: MaybeProp<NumberType>,
    /// Whether or not to trim surrounding whitespace "  My name " -> "My name"
    #[prop(default = true)]
    trim: bool,
) -> impl IntoView
where
    NumberType: Num + NumCast + Clone + NumOps + std::default::Default + Send + Sync + 'static,
{
}

// TODO: Decimal number support with rust_decimal, e.g. floats cannot represent 0.01 and have many other edges to deal with when calculating.
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
        + Rem<NumberType>
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

        if let Some(step) = number_config.step.get()
            && NumberType::is_int()
        {
            let remainder = parsed_value.clone() % step.clone();
            if remainder != NumberType::zero() {
                return Err(format!(
                    "Value must be divisible by {step}, current remainder: {remainder}"
                ));
            }
        }

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
    let step = if let Some(step) = number_config.step.get() {
        step.to_string()
    } else {
        "any".to_string()
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
            step=step
            min=min
            max=max
        />
    }
}

#[generate_docs]
#[component]
pub fn OptNumberInput<OptNumberType, NumberType>(
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
    value: RwSignal<OptNumberType>,
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
    OptNumberType: IsOption<Inner = NumberType>
        + ConstructSelfFromInner<NumberType>
        + Display
        + Eq
        + Clone
        + Default
        + Debug
        + Send
        + Sync
        + 'static,
    NumberType: Num
        + NumCast
        + NumOps
        + Display
        + Intness
        + Signedness
        + PartialOrd
        + Clone
        + Default
        + Send
        + Sync
        + Rem<NumberType>
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

        if let Some(step) = number_config.step.get()
            && NumberType::is_int()
        {
            let remainder = parsed_value.clone() % step.clone();
            if remainder != NumberType::zero() {
                return Err(format!(
                    "Value must be divisible by {step}, current remainder: {remainder}"
                ));
            }
        }

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

        Ok(OptNumberType::construct_self(parsed_value))
    };

    let format = move |input: OptNumberType| input.to_string();
    let step = if let Some(step) = number_config.step.get() {
        step.to_string()
    } else {
        "any".to_string()
    };

    view! {
        <GenericInput<OptNumberType, String>
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

pub trait IsOption {
    type Inner;
    fn into_option(self) -> Option<Self::Inner>;
}

pub trait ConstructSelfFromInner<Inner> {
    fn construct_self(inner: Inner) -> Self;
}

impl<T> ConstructSelfFromInner<T> for Option<T> {
    fn construct_self(inner: T) -> Self {
        Some(inner)
    }
}

impl<T> IsOption for Option<T> {
    type Inner = T;
    fn into_option(self) -> Option<T> {
        self
    }
}

// #[cfg(feature = "decimal")]
// struct Decimal {
//     decimal: rust_decimal::Decimal
// }

// #[cfg(feature = "decimal")]
// impl Num for Decimal {

// }

macro_rules! define_optional {
    ($T:ty, $name:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, Default)]
        pub struct $name(Option<$T>);
        impl From<$T> for $name {
            fn from(value: $T) -> Self {
                Self(Some(value))
            }
        }
        impl IsOption for $name {
            type Inner = $T;

            fn into_option(self) -> Option<Self::Inner> {
                self.0
            }
        }
        impl ConstructSelfFromInner<$T> for $name {
            fn construct_self(inner: $T) -> Self {
                Self(Some(inner))
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self.0 {
                    Some(number) => write!(f, "{number}"),
                    None => write!(f, "none"),
                }
            }
        }
    };
}

define_optional!(u8, OptionalU8);
define_optional!(u16, OptionalU16);
define_optional!(u32, OptionalU32);
define_optional!(u64, OptionalU64);
define_optional!(u128, OptionalU128);

define_optional!(i8, OptionalI8);
define_optional!(i16, OptionalI16);
define_optional!(i32, OptionalI32);
define_optional!(i64, OptionalI64);
define_optional!(i128, OptionalI128);
