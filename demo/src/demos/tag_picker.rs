use attr_docgen::generate_codeblock;
use derive_more::Display;
use leptodon::button::Button;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::tag_picker::TagPicker;
use leptos::prelude::ClassAttribute;
use leptos::prelude::CollectView;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_meta::Title;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[generate_codeblock(TagPickerExample)]
#[component]
pub fn TagPickerDemo() -> impl IntoView {
    #[derive(EnumIter, Display, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
    pub enum Element {
        Hydrogen,
        Helium,
        Lithium,
    }
    impl AsRef<str> for Element {
        fn as_ref(&self) -> &'static str {
            match self {
                Element::Hydrogen => "hydrogen",
                Element::Helium => "helium",
                Element::Lithium => "lithium",
            }
        }
    }

    let elements = RwSignal::new(Element::iter().collect::<Vec<_>>());
    let selected = RwSignal::new(vec![]);
    view! {
        <p>
            "Selected: "
            {move || selected.get()
                .iter()
                .map(|tag: &Element| {
                    view! {
                        {tag.to_string()},
                    }
                })
                .collect_view()}
        </p>
        <TagPicker
            selected=selected
            tags=elements
        />
        <Button on_click=move |_e| {
            selected.set(vec![Element::Lithium]);
        }>"Set Lithium as Selected"</Button>
    }
}

#[component]
pub fn TagPickerDemoPage() -> impl IntoView {
    view! {
        <Title text="Tag Picker"/>

        <FixedCenterColumn>
            <Heading4 anchor="tag-picker">"Tag Picker"</Heading4>
            <TagPickerExample />

            <leptodon::tag_picker::TagPickerDocs />
        </FixedCenterColumn>
    }
}
