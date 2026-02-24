use attr_docgen::generate_codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::link::Link;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(LinkExample)]
#[component]
pub fn LinkDemo() -> impl IntoView {
    view! {
        "Explore more about OA on the "
        <Link href="https://openanalytics.eu" target="_blank">OA website</Link>
    }
}

#[component]
pub fn LinkDemoPage() -> impl IntoView {
    view! {
        <Title text="Link"/>

        <FixedCenterColumn>
            <Heading4 anchor="link">"Link"</Heading4>
            <LinkExample />

            <leptodon::link::LinkDocs />
        </FixedCenterColumn>
    }
}
