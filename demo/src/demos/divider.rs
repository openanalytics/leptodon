use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::divider::HorizontalLine;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(DividerExample)]
#[component]
pub fn DividerDemo() -> impl IntoView {
    view! {
        <p>
            "Section1"
        </p>
        <HorizontalLine />
        <p>
            "Section2"
        </p>
    }
}

#[component]
pub fn DividerDemoPage() -> impl IntoView {
    view! {
        <Title text="Divider"/>

        <FixedCenterColumn>
            <Heading4 anchor="divider">"Divider"</Heading4>
            <DividerExample />

            <leptos_components::divider::HorizontalLineDocs />
        </FixedCenterColumn>
    }
}
