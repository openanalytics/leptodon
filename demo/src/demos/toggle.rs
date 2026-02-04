use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Set;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::button::Button;
use leptos_components::form_input::Label;
use leptos_components::layout::CenteringColumn;
use leptos_components::toggle::Toggle;
use leptos_meta::Title;

#[generate_codeblock(ToggleExample)]
#[component]
pub fn ToggleDemo() -> impl IntoView {
    let checked = RwSignal::new(true);

    view! {
        <p>
            "Current toggle checked state: "
            {move || checked.get().to_string()}
        </p>
        <Toggle
            class="my-3"
            checked=checked
        >
            Test Label
        </Toggle>
        <br/>
        <Button on_click=move |_| {
            checked.set(true);
        }>
            On
        </Button>
        <Button on_click=move |_| {
            checked.set(false);
        }>
            Off
        </Button>
    }
}

#[component]
pub fn ToggleDemoPage() -> impl IntoView {
    view! {
        <Title text="Test Toggle"/>

        <CenteringColumn>
            <Label required=false label="Example Toggle usage">
                <div class="flex flex-col border-1 border border-black rounded-lg shadow-sm w-fit p-4 min-w-[50vw]">
                    <div class="p-3">
                        <ToggleDemo />
                    </div>
                    <hr class="mb-4"></hr>
                    <ToggleDemoCodeblock />
                </div>
            </Label>
            <leptos_components::toggle::ToggleDocs />
        </CenteringColumn>
    }
}
