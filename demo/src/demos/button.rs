use attr_docgen::generate_codeblock;
use leptodon::button::AddButton;
use leptodon::button::Button;
use leptodon::button::ButtonAppearance;
use leptodon::button::ButtonShape;
use leptodon::button::CopyButton;
use leptodon::button::DeleteButton;
use leptodon::button::DownloadButton;
use leptodon::button::EditButton;
use leptodon::button_group::ButtonGroup;
use leptodon::button_group::First;
use leptodon::button_group::Last;
use leptodon::heading::Heading4;
use leptodon::icon;
use leptodon::layout::FixedCenterColumn;
use leptodon::textarea::TextArea;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::prelude::Update;
use leptos::{IntoView, component, prelude::RwSignal, view};
use leptos_meta::Title;

#[generate_codeblock(ButtonExample)]
#[component]
pub fn ButtonDemo() -> impl IntoView {
    let count = RwSignal::new(0);

    view! {
        <p>
            {move ||
                format!("Button was pressed {} times!", count.get())
            }
        </p>

        <Button
            appearance=ButtonAppearance::Primary
            shape=ButtonShape::Rounded
            icon=icon::AddIcon()
            on_click=move |_| {
                count.update(|old| *old += 1);
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
    let to_copy = RwSignal::new("📋".to_string());
    view! {
        <AddButton/>
        <EditButton/>
        <DeleteButton/>
        <DownloadButton/>
        <CopyButton class="m-2" to_copy=to_copy />
        <br/>
        <TextArea
            placeholder="Paste testing area.."
            value=RwSignal::new(String::default())
        />
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

            <leptodon::button::ButtonDocs />
            <leptodon::button_group::ButtonGroupDocs />
        </FixedCenterColumn>
    }
}
