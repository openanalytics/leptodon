use crate::web_calendar::PopulatedCalendar;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[component]
pub fn TestCalendar() -> impl IntoView {
    view! {
        <Title text="Test Calendar"/>
        <PopulatedCalendar />
    }
}
