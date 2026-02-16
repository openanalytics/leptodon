use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::button::Button;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_components::toggle::Toggle;
use leptos_meta::Title;

#[generate_codeblock(ToggleExample)]
#[component]
pub fn ToggleDemo() -> impl IntoView {
    let checked = RwSignal::new(true);

    view! {
        <p>
            "Current toggle checked state: "
            {move || checked.get().to_string()}
        </p>
        <Toggle
            class="my-3"
            checked=checked
        >
            Test Label
        </Toggle>
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
pub fn ToggleDemoPage() -> impl IntoView {
    view! {
        <Title text="Toggle Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="toggle">"Toggle"</Heading4>
            <ToggleExample />

            <leptos_components::toggle::ToggleDocs />
        </FixedCenterColumn>
    }
}
