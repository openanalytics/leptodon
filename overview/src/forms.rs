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
use derive_more::Display;
use leptodon::button::ButtonType;
use leptodon::checkbox::Checkbox;
use leptodon::date_picker::DatePicker;
use leptodon::date_picker::range_picker::DateRangePicker;
use leptodon::form_input::FormInput;
use leptodon::input::{InputType, PasswordInput, TextInput};
use leptodon::link::Link;
use leptodon::select::{MaybeSelect, Select};
use leptodon::textarea::TextArea;
use leptodon::toggle::Toggle;
use leptodon::{
    button::Button,
    radio::{Radio, RadioOption},
};
use leptos::prelude::AddAnyAttr;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{IntoView, component, oco::Oco, prelude::RwSignal, view};
use leptos_meta::Title;
use leptos_router::components::Form;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
enum RadioStation {
    Radio1,
    Radio2,
    Klara,
}
impl RadioOption for RadioStation {
    fn value(&self) -> Oco<'static, str> {
        match self {
            RadioStation::Radio1 => "radio_1",
            RadioStation::Radio2 => "radio_2",
            RadioStation::Klara => "klara",
        }
        .into()
    }
}

#[derive(EnumIter, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum Element {
    Hydrogen,
    Helium,
    Lithium,
    Beryllium,
    Boron,
    Carbon,
    Nitrogen,
    Oxygen,
    Fluorine,
    Neon,
    Natrium,
    Magnesium,
    Aluminium,
}

impl RadioOption for Element {
    fn value(&self) -> Oco<'static, str> {
        AsRef::<str>::as_ref(&self).to_string().into()
    }
}

impl AsRef<str> for Element {
    fn as_ref(&self) -> &'static str {
        match self {
            Element::Hydrogen => "hydrogen",
            Element::Helium => "helium",
            Element::Lithium => "lithium",
            Element::Beryllium => "beryllium",
            Element::Boron => "boron",
            Element::Carbon => "carbon",
            Element::Nitrogen => "nitrogen",
            Element::Oxygen => "oxygen",
            Element::Fluorine => "fluorine",
            Element::Neon => "neon",
            Element::Natrium => "natrium",
            Element::Magnesium => "magnesium",
            Element::Aluminium => "aluminium",
        }
    }
}

#[component]
pub fn Forms() -> impl IntoView {
    let radio_options = RwSignal::new(vec![
        RadioStation::Radio1,
        RadioStation::Radio2,
        RadioStation::Klara,
    ]);
    let elements = RwSignal::new(Element::iter().collect());

    view! {
        <Title text="Forms" />

        <Form action="/forms">
            <div class="p-4">
                <Radio
                    class="my-3"
                    name="radio_station"
                    label="Radio Stations"
                    options=radio_options
                    required=true
                    {..}
                    attr:data-testid="radio-input"
                />
                <FormInput<String> label="Favorite Element" required=false>
                    <MaybeSelect
                        class="my-3"
                        name="favorite_element"
                        options=elements
                        selected=RwSignal::new(None)
                    />
                </FormInput<String>>
                <FormInput<String> label="An Element" required=true>
                    <Select
                        class="my-3"
                        name="an_element"
                        options=elements
                        selected=RwSignal::new(Element::Carbon)
                    />
                </FormInput<String>>
                <FormInput<String> label="Full name" required=true>
                    <TextInput name="fullname" placeholder="Preferred full name" />
                </FormInput<String>>
                <FormInput<String> label="Email address" required=true>
                    <TextInput name="email" placeholder="localpart@domain" input_type=InputType::Email />
                </FormInput<String>>
                <FormInput<String> label="Password" required=true>
                    <PasswordInput name="password" placeholder="*******************" hazards=vec!["Merlijn".to_string()] show_eye=true />
                </FormInput<String>>
                <Checkbox required=true checked=RwSignal::new(false)>
                    "Accept "<Link class="inline-block" href="/terms">"terms"</Link>" and conditions"
                </Checkbox>
                <br/>
                <Toggle required=true checked=RwSignal::new(false)>
                    "Advertising cookies"
                </Toggle>
                <br/>
                <FormInput<String> label="End date" required=true>
                    <DatePicker name="end-date" value=RwSignal::new(None) />
                </FormInput<String>>
                <FormInput<String> label="Period" required=true>
                    <DateRangePicker name="period" id="date_range_picker" />
                </FormInput<String>>
                <FormInput<String> label="Notes" required=true>
                    <TextArea value=RwSignal::new("".to_string()) name="notes" />
                </FormInput<String>>
                <Button button_type=ButtonType::Submit>"Submit"</Button>
            </div>
        </Form>
    }
}
