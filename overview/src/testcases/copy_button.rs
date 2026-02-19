use leptos::prelude::ElementChild;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::button::Button;
use leptos_components::button::CopyButton;
use leptos_components::textarea::TextArea;
use leptos_meta::Title;

#[component]
pub fn TestCopyButton() -> impl IntoView {
    let to_copy = RwSignal::new("test_string1".to_string());

    view! {
        <Title text="Test CopyButton"/>
        <p>
            {to_copy}
        </p>
        <CopyButton id="copy-button" to_copy=to_copy />
        <Button id="set-test-string1" on_click=move |_e| {
            to_copy.set("test_string1".to_string());
        }>"test_string1"</Button>
        <Button id="set-test-string2" on_click=move |_e| {
            to_copy.set("test_string2".to_string());
        }>"test_string2"</Button>
        <TextArea placeholder="Test paste here" value=RwSignal::new(String::new())/>
    }
}
