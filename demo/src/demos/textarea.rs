use attr_docgen::generate_codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::textarea::TextArea;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::RwSignal;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(TextAreaExample)]
#[component]
pub fn TextAreaDemo() -> impl IntoView {
    let value = RwSignal::new("".to_string());
    view! {
        <p>Synced textareas!</p>
        <TextArea
            label="Magically linked"
            required=true
            placeholder="Enter your magic words."
            value
        />
        <TextArea value />
    }
}

#[component]
pub fn TextAreaDemoPage() -> impl IntoView {
    view! {
        <Title text="TextArea"/>

        <FixedCenterColumn>
            <Heading4 anchor="textarea">"TextArea"</Heading4>
            <TextAreaExample />

            <leptodon::textarea::TextAreaDocs />
        </FixedCenterColumn>
    }
}
