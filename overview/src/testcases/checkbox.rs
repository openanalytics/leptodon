use leptodon::button::Button;
use leptodon::checkbox::Checkbox;
use leptos::prelude::AddAnyAttr;
use leptos::prelude::CustomAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[component]
pub fn TestCheckbox() -> impl IntoView {
    let checked = RwSignal::new(true);
    view! {
        <Title text="Test Checkbox"/>
        <p data-testid="checkbox-disp">
            {move || checked.get().to_string()}
        </p>
        <Checkbox
            class="my-3"
            checked=checked
            attr:data-testid="checkbox"
        >
            Test Label
        </Checkbox>
        <br/>
        <Button on_click=move |_| {
            checked.set(true);
        } attr:data-testid="btn-on">
            On
        </Button>
        <Button on_click=move |_| {
            checked.set(false);
        } attr:data-testid="btn-off">
        Off
        </Button>
    }
}
