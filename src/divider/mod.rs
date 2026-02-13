use attr_docgen::generate_docs;
use leptos::IntoView;
use leptos::component;
use leptos::prelude::ClassAttribute;
use leptos::view;

const HEADER_CLASS: &str = "font-bold relative color-gray-900";

#[generate_docs]
#[component]
pub fn HorizontalLine() -> impl IntoView {
    view! {
        <hr class="my-4" />
    }
}
