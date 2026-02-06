use leptos::IntoView;
use leptos::prelude::ClassAttribute;
use leptos::view;
use leptos::component;

const HEADER_CLASS: &str = "font-bold relative color-gray-900";

#[component]
fn HorizontalLine() -> impl IntoView {
    view! {
        <hr class="my-4" />
    }
}