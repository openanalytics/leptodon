use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::checkbox::Checkbox;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(CheckboxExample)]
#[component]
pub fn CheckboxDemo() -> impl IntoView {
    let checked = RwSignal::new(true);
    view! {
        <p>
            {move || checked.get().to_string()}
        </p>
        <Checkbox
            class="my-3"
            checked=checked
        >
            Test Label
        </Checkbox>
        <br/>
        <Button on_click=move |_| {
            checked.set(true);
        }>
            On
        </Button>
        <Button on_click=move |_| {
            checked.set(false);
        }>
        Off
        </Button>
    }
}

#[component]
pub fn CheckboxDemoPage() -> impl IntoView {
    view! {
        <Title text="Checkbox"/>

        <FixedCenterColumn>
            <Heading4 anchor="checkbox">"Checkbox"</Heading4>
            <CheckboxExample />

            <leptos_components::checkbox::CheckboxDocs />
        </FixedCenterColumn>
    }
}
