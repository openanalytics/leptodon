use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::input::TextInput;
use leptos_components::input::TextInputConfigProps;
use leptos_meta::Title;

#[component]
pub fn TestInputs() -> impl IntoView {
    let text_value = RwSignal::new(String::new());
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
    }
}
