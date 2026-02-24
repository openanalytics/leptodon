use std::ops::RangeInclusive;

use leptodon::button::Button;
use leptodon::tag_picker::TagPicker;
use leptos::prelude::CollectView;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;
use strum::IntoEnumIterator;

use crate::forms::Element;

#[component]
pub fn TestTagPicker() -> impl IntoView {
    let range_to_tags = |range: RangeInclusive<i32>| {
        Element::iter()
            .enumerate()
            .filter(|(i, _)| range.contains(&(*i as i32)))
            .map(|(_, e)| e)
            .collect::<Vec<Element>>()
    };
    let elements = RwSignal::new(Element::iter().collect::<Vec<_>>());
    let selected = RwSignal::new(vec![]);
    view! {
        <Title text="Test Tag Picker"/>
        <p>
            "Selected: "
            <span id="selected-display">
            {move || selected.get()
                .iter()
                .map(|tag: &Element| {
                    view! {
                        {tag.to_string()}
                    }
                })
                .collect_view()}
            </span>
        </p>
        <TagPicker
            id="tag_picker"
            class="mt-[150vh]"
            selected=selected
            tags=elements
        />
        <Button id="set-5" on_click=move |_e| {
            selected.set(range_to_tags(1..=5));
        }>"Set 1..=5 as Selected"</Button>
        <Button id="set-none" on_click=move |_e| {
            selected.set(vec![]);
        }>"Clear selection"</Button>
        <Button id="elems-1-7" on_click=move |_e| {
            elements.set(range_to_tags(1..=7));
        }>"Make selectable elements = (1..=7)"</Button>
        <Button id="elems-3-10" on_click=move |_e| {
            elements.set(range_to_tags(3..=10));
        }>"Make selectable elements = (3..=10)"</Button>
    }
}
