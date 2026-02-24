use attr_docgen::generate_codeblock;
use leptodon::heading::Heading1;
use leptodon::heading::Heading2;
use leptodon::heading::Heading3;
use leptodon::heading::Heading4;
use leptodon::heading::Heading5;
use leptodon::heading::Heading6;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
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

            <leptodon::heading::Heading1Docs />
            <p>The other heading variants have the same docs.</p>
        </FixedCenterColumn>
    }
}
