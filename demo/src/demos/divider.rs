use attr_docgen::generate_codeblock;
use leptodon::divider::HorizontalLine;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
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

            <leptodon::divider::HorizontalLineDocs />
        </FixedCenterColumn>
    }
}
