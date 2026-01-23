use derive_more::Display;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{IntoView, component, oco::Oco, prelude::RwSignal, view};
use leptos_components::button::ButtonType;
use leptos_components::form_input::FormInput;
use leptos_components::input::{InputType, PasswordInput, TextInput};
use leptos_components::select::{MaybeSelect, Select};
use leptos_components::{
    button::Button,
    radio::{Radio, RadioOption},
};
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
        <Form action="/forms">
            <div class="p-4">
                <Radio
                    class="my-3"
                    name="radio_station"
                    label="Radio Stations"
                    options=radio_options
                    required=true
                />
                <MaybeSelect
                    class="my-3"
                    name="favorite_element"
                    label="Favorite Element"
                    options=elements
                    selected=RwSignal::new(None)
                    required=false
                />
                <Select
                    class="my-3"
                    name="an_element"
                    label="An Element"
                    options=elements
                    selected=RwSignal::new(Element::Carbon)
                    required=true
                />
                <TextInput label="Full name" name="fullname" placeholder="Preferred full name" required=true />
                <TextInput label="Email address" name="email" placeholder="localpart@domain" input_type=InputType::Email required=true />
                <FormInput<String> label="Password" required=true>
                    <PasswordInput name="password" placeholder="*******************" hazards=vec!["Merlijn".to_string()] show_eye=true required=true />
                </FormInput<String>>
                // <TextInput label="Passcode" placeholder="*******************" input_type=InputType::Password required=true />
                <Button button_type=ButtonType::Submit>"Submit"</Button>
            </div>
        </Form>
    }
}
