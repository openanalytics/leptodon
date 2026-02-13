use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::heading::Heading1;
use leptos_components::heading::Heading2;
use leptos_components::heading::Heading3;
use leptos_components::heading::Heading4;
use leptos_components::heading::Heading5;
use leptos_components::heading::Heading6;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(HeadingExample)]
#[component]
pub fn HeadingDemo() -> impl IntoView {
    view! {
        <Heading1 anchor="the-largest-heading">The Largest Heading</Heading1>
        <Heading2 class="text-red-500">The 2nd Largest Heading</Heading2>
        <Heading3>The Large Heading</Heading3>
        <Heading4>The Heading</Heading4>
        <Heading5>The Smaller Heading</Heading5>
        <Heading6>The Smallest Heading</Heading6>
    }
}

#[component]
pub fn HeadingDemoPage() -> impl IntoView {
    view! {
        <Title text="Heading"/>

        <FixedCenterColumn>
            <Heading4 anchor="heading">"Heading"</Heading4>
            <HeadingExample />

            <leptos_components::heading::Heading1Docs />
            <p>The other heading variants have the same docs.</p>
        </FixedCenterColumn>
    }
}
