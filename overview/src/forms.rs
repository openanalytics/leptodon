use derive_more::Display;
use leptos::prelude::{ClassAttribute, ElementChild};
use leptos::{IntoView, component, oco::Oco, prelude::RwSignal, view};
use leptos_components::button::ButtonType;
use leptos_components::{
    button::Button,
    radio::{Radio, RadioOption},
};
use leptos_router::components::Form;

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

#[component]
pub fn Forms() -> impl IntoView {
    let radio_options = RwSignal::new(vec![
        RadioStation::Radio1,
        RadioStation::Radio2,
        RadioStation::Klara,
    ]);

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
                <Button button_type=ButtonType::Submit>"Submit"</Button>
            </div>
        </Form>
    }
}
