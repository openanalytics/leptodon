use attr_docgen::generate_codeblock;
use leptodon::darkmode::ThemeSelector;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(ThemeSelectorExample)]
#[component]
pub fn ThemeSelectorDemo() -> impl IntoView {
    view! {
        <p>Using multiple theme-selectors on the same page (as done here) does not work correctly.</p>
        <ThemeSelector/>
    }
}

#[component]
pub fn ThemeSelectorDemoPage() -> impl IntoView {
    view! {
        <Title text="ThemeSelector"/>

        <FixedCenterColumn>
            <Heading4 anchor="themeselector">"ThemeSelector"</Heading4>
            <ThemeSelectorExample />

            <leptodon::darkmode::ThemeSelectorDocs />
        </FixedCenterColumn>
    }
}
