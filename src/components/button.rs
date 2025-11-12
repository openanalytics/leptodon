use leptos::ev::MouseEvent;
use leptos::prelude::{ElementChild, GlobalAttributes, IntoRender, OnAttribute};
use leptos::{IntoView, component, view};

#[component]
fn Button<'a>(
    id: String,
    label: &'a str,
    on_click: impl Fn(MouseEvent) + 'static,
) -> impl IntoView {
    view! {
        <button type="button" id=id on:click=on_click>
            {label}
        </button>
    }
}
