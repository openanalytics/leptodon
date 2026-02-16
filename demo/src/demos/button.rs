use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Update;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_components::button::AddButton;
use leptos_components::button::Button;
use leptos_components::button::ButtonAppearance;
use leptos_components::button::ButtonShape;
use leptos_components::button::DeleteButton;
use leptos_components::button::DownloadButton;
use leptos_components::button::EditButton;
use leptos_components::button_group::ButtonGroup;
use leptos_components::button_group::First;
use leptos_components::button_group::Last;
use leptos_components::heading::Heading4;
use leptos_components::icon;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(ButtonExample)]
#[component]
pub fn ButtonDemo() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <p>
            {move ||
                format!("Button was pressed {} times!", count.get().to_string())
            }
        </p>

        <Button
            appearance=ButtonAppearance::Primary
            shape=ButtonShape::Rounded
            icon=icon::AddIcon()
            on_click=move |_| {
                count.update(|old| *old = *old + 1);
            }
        >
            1
        </Button>
    }
}

#[generate_codeblock(ButtonGroupExample)]
#[component]
pub fn ButtonGroupDemo() -> impl IntoView {
    view! {
        <ButtonGroup>
            <First slot:first>
                <Button class="mr-0" on_click=move |_| {}>Profile</Button>
            </First>
            <Button on_click=move |_| {}>Settings</Button>
            <Button on_click=move |_| {}>Settings2</Button>
            <Last slot:last>
                <Button on_click=move |_| {}>Messages</Button>
            </Last>
        </ButtonGroup>
    }
}

#[generate_codeblock(StyledButtonExample)]
#[component]
pub fn PremadeButtonDemo() -> impl IntoView {
    view! {
        <AddButton/>
        <EditButton/>
        <DeleteButton/>
        <DownloadButton/>
    }
}

#[component]
pub fn ButtonDemoPage() -> impl IntoView {
    view! {
        <Title text="Button Components"/>

        <FixedCenterColumn>
            <Heading4 anchor="button">"Button"</Heading4>
            <ButtonExample />

            <Heading4 anchor="button-group">"Button Group"</Heading4>
            <ButtonGroupExample />

            <Heading4 anchor="premade-buttons">"Premade Buttons"</Heading4>
            <StyledButtonExample />

            <leptos_components::button::ButtonDocs />
            <leptos_components::button_group::ButtonGroupDocs />
        </FixedCenterColumn>
    }
}
