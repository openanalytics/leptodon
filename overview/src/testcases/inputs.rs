use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::input::NumberInput;
use leptos_components::input::NumberInputConfigProps;
use leptos_components::input::TextInput;
use leptos_components::input::TextInputConfigProps;
use leptos_meta::Title;

#[component]
pub fn TestInputs() -> impl IntoView {
    let text_value = RwSignal::new(String::new());
    let u32value = RwSignal::new(0u32);
    let i128value = RwSignal::new(-1i128);
    let f64value = RwSignal::new(3.1415);

    view! {
        <Title text="Test Inputs"/>
        <p id="text-input-display">
            {move || text_value.get()}
        </p>
        <TextInput
            id="text-input"
            class="my-3"
            value=text_value
            text_config=TextInputConfigProps::builder()
                .max_len(10)
                .trim(true)
                .build()
        />

        <p id="u32-input-display">
            {move || u32value.get()}
        </p>
        <NumberInput<u32>
            id="u32-input"
            class="my-3"
            value=u32value
            number_config={
                NumberInputConfigProps::<u32>::builder()
                    .max(10)
                    .trim(true)
                    .build()
            }
        />

        <p id="i128-input-display">
            {move || i128value.get()}
        </p>
        <NumberInput<i128>
            id="i128-input"
            class="my-3"
            value=i128value
            number_config={
                NumberInputConfigProps::<i128>::builder()
                    .max(10)
                    .min(-100)
                    .trim(true)
                    .build()
            }
        />

        <p id="f64-input-display">
            {move || f64value.get()}
        </p>
        <NumberInput<f64>
            id="f64-input"
            class="my-3"
            value=f64value
            number_config={
                NumberInputConfigProps::<f64>::builder()
                    .max(10.0005)
                    .min(-2.00001)
                    .trim(true)
                    .build()
            }
        />
    }
}
