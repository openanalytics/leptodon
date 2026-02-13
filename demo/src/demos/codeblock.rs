use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::codeblock::Codeblock;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(CodeblockExample)]
#[component]
pub fn CodeblockDemo() -> impl IntoView {
    view! {
        <Codeblock code=r#"fn main() {
    println!("Hello world!");
}"#>
        </Codeblock>
    }
}

#[component]
pub fn CodeblockDemoPage() -> impl IntoView {
    view! {
        <Title text="Codeblock"/>

        <FixedCenterColumn>
            <Heading4 anchor="codeblock">"Codeblock"</Heading4>
            <CodeblockExample />

            <leptos_components::codeblock::CodeblockDocs />
        </FixedCenterColumn>
    }
}
