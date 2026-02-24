use attr_docgen::generate_codeblock;
use leptodon::button::Button;
use leptodon::checkbox::Checkbox;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
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

            <leptodon::checkbox::CheckboxDocs />
        </FixedCenterColumn>
    }
}
