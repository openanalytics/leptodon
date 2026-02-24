use attr_docgen::generate_codeblock;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::spinner::Spinner;
use leptodon::spinner::SpinnerAppearance;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(SpinnerExample)]
#[component]
pub fn SpinnerDemo() -> impl IntoView {
    view! {
        <div class="flex flex-row items-center gap-2">
        <Spinner class="w-5 h-5"/>
        <Spinner class="w-10 h-10" appearance=SpinnerAppearance::OA/>
        // Text themes the 75% portion and strok themes the 25% portion.
        <Spinner class="w-15 h-15" appearance=SpinnerAppearance::Custom(
            "text-red-500 stroke-oa-gray dark:text-red-500 dark:stroke-gray-700".into()
        )/>
        </div>
    }
}

#[component]
pub fn SpinnerDemoPage() -> impl IntoView {
    view! {
        <Title text="Spinner"/>

        <FixedCenterColumn>
            <Heading4 anchor="spinner">"Spinner"</Heading4>
            <SpinnerExample />

            <leptodon::spinner::SpinnerDocs />
        </FixedCenterColumn>
    }
}
