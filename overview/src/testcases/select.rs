use leptos::prelude::ElementChild;
use leptos::prelude::GlobalAttributes;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::select::MaybeSelect;
use leptos_components::button::Button;
use leptos_meta::Title;

#[component]
pub fn TestSelect() -> impl IntoView {
    let elements = RwSignal::new((1..=10).collect::<Vec<u8>>());
    let selected = RwSignal::new(None);
    view! {
        <Title text="Test Select"/>
        <p id="selected-display">
            {selected}
        </p>
        <MaybeSelect
            class="my-3"
            name="favorite_number"
            options=elements
            selected=selected
        />
        <Button id="set-5" on_click=move |_e| {
            selected.set(Some(5));
        }>"Set 5 as Selected"</Button>
        <Button id="set-none" on_click=move |_e| {
            selected.set(None);
        }>"Clear selection"</Button>
        <Button id="elems-1-7" on_click=move |_e| {
            elements.set((1..=7).collect::<Vec<u8>>());
        }>"Make selectable elements = (1..=7)"</Button>
        <Button id="elems-3-10" on_click=move |_e| {
            elements.set((3..=10).collect::<Vec<u8>>());
        }>"Make selectable elements = (3..=10)"</Button>
    }
}
