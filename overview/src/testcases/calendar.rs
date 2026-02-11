use leptos::{IntoView, component, view};
use leptos_meta::Title;
use crate::web_calendar::PopulatedCalendar;

#[component]
pub fn TestCalendar() -> impl IntoView {
    view! {
        <Title text="Test Calendar"/>
        <PopulatedCalendar />
    }
}
