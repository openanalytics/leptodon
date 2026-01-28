use derive_more::Display;
use leptos::prelude::AddAnyAttr;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{IntoView, component, oco::Oco, prelude::RwSignal, view};
use leptos_components::button::ButtonType;
use leptos_components::checkbox::Checkbox;
use leptos_components::date_picker::DatePicker;
use leptos_components::date_picker::range_picker::DateRangePicker;
use leptos_components::form_input::FormInput;
use leptos_components::input::{InputType, PasswordInput, TextInput};
use leptos_components::link::Link;
use leptos_components::select::{MaybeSelect, Select};
use leptos_components::textarea::TextArea;
use leptos_components::toggle::Toggle;
use leptos_components::{
    button::Button,
    radio::{Radio, RadioOption},
};
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
enum Element {
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
        .into()
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
                <Checkbox required=true value=false>
                    <span class="whitespace-pre-wrap">
                        Accept <Link class="inline-block" href="/terms"> terms </Link> and conditions
                    </span>
                </Checkbox>
                <br/>
                <Toggle required=true value=false>
                    "Advertising cookies"
                </Toggle>
                <br/>
                <FormInput<String> label="End date" required=true>
                    <DatePicker name="end-date" value=RwSignal::new(None) />
                </FormInput<String>>
                <FormInput<String> label="Period" required=true>
                    <DateRangePicker name="period" />
                </FormInput<String>>
                <FormInput<String> label="Notes" required=true>
                    <TextArea value=RwSignal::new("".to_string()) name="notes" />
                </FormInput<String>>
                <Button button_type=ButtonType::Submit>"Submit"</Button>
            </div>
        </Form>
    }
}
