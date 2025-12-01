use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

use crate::class_list;
use crate::icon;
use crate::input::InputMode;
use crate::input::InputType;
use crate::input_group::GroupItemContextProvider;
use leptos::logging::debug_log;
use leptos::prelude::ClassAttribute;
use leptos::prelude::Effect;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::MaybeProp;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Signal;
use leptos::{IntoView, component, view};
use num_traits::Bounded;
use num_traits::ConstOne;
use num_traits::SaturatingAdd;
use num_traits::SaturatingSub;

use crate::{button::Button, input::Input};

fn clamp<T>(some: T, min: T, max: T) -> T
where
    T: PartialOrd,
{
    if some < min {
        min
    } else if some > max {
        max
    } else {
        some
    }
}

#[component]
pub fn ControlledNumberInput<T>(
    /// Name of the input.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// Extra classes to size the component
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// The minimum number that the input value can take.
    #[prop(default = T::min_value().into(), into)]
    min: Signal<T>,
    /// The maximum number that the input value can take.
    #[prop(default = T::max_value().into(), into)]
    max: Signal<T>,
    /// Placeholder of input number.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Step size for the increment and decrement, defaults to the multiplicative identity of T (e.g. 1 for i32).
    #[prop(default = T::one())]
    step: T,
) -> impl IntoView
where
    T: Send + Sync,
    T: ConstOne
        + Add<Output = T>
        + Sub<Output = T>
        + SaturatingAdd<Output = T>
        + SaturatingSub<Output = T>
        + PartialOrd
        + Bounded,
    T: Default + Clone + Copy + FromStr + ToString + Display + 'static,
{
    // let value: RwSignal<Option<i32>> = RwSignal::new(None);
    let value_binder: RwSignal<String> = RwSignal::new("".to_string());
    Effect::watch(
        move || value_binder.get(),
        move |new_value, prev_value, _| {
            if Some(new_value) == prev_value {
                return;
            }
            let Ok(new_value) = new_value.parse::<T>() else {
                debug_log!(
                    "User inputted a non-number {new_value:?} resetting their input to {prev_value:?}"
                );
                let text = String::new();
                value_binder.set(format!("{}", prev_value.unwrap_or(&text)));
                return;
            };
            if new_value != clamp::<T>(new_value, min.get(), max.get()) {
                debug_log!(
                    "User inputted a number {new_value} outside of ({}, {}) ",
                    min.get(),
                    max.get()
                );
                let text = String::new();
                value_binder.set(format!("{}", prev_value.unwrap_or(&text)));
            };
        },
        false,
    );
    let inc_step = step.clone();
    let inc_handler = move |_| {
        let unparsed = value_binder.get();
        let old_value = unparsed.parse::<T>();
        if let Ok(old_value) = old_value {
            value_binder.set(format!(
                "{}",
                clamp(old_value.saturating_add(&inc_step), min.get(), max.get())
            ));
        } else if unparsed.is_empty() {
            value_binder.set(format!("1"));
        }
    };
    let dec_handler = move |_| {
        let unparsed = value_binder.get();
        let old_value = unparsed.parse::<T>();
        if let Ok(old_value) = old_value {
            value_binder.set(format!(
                "{}",
                clamp(old_value.saturating_sub(&step), min.get(), max.get())
            ));
        } else if unparsed.is_empty() {
            value_binder.set(format!("1"));
        }
    };
    view! {
        <div class=class_list!(class, "relative flex items-center mb-2")>
            <GroupItemContextProvider class="rounded-none rounded-l-lg">
                <Input name placeholder input_type=InputType::Text input_mode=InputMode::Numeric value=value_binder />
            </GroupItemContextProvider>
            <GroupItemContextProvider class="rounded-none border-x-0 !mr-0">
                <Button icon=icon::DecrementIcon() on_click=dec_handler />
            </GroupItemContextProvider>
            <GroupItemContextProvider class="rounded-none rounded-r-lg">
                <Button icon=icon::IncrementIcon() on_click=inc_handler />
            </GroupItemContextProvider>
            // Block context leakage
            <GroupItemContextProvider class="">
                <div/>
            </GroupItemContextProvider>
        </div>
    }
}
