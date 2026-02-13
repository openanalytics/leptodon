use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::signal;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::button::DropdownButton;
use leptos_components::button::DropdownButtonChildren;
use leptos_components::dropdown::AlignmentAnchor;
use leptos_components::dropdown::Dropdown;
use leptos_components::dropdown::DropdownItem;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_meta::Title;

#[generate_codeblock(DropdownExample)]
#[component]
pub fn DropdownDemo() -> impl IntoView {
    let last_action = RwSignal::new("/");
    let (is_visible, set_visible) = signal(false);
    view! {
        "Last action: " {move || format!("{:?}", last_action.get())}
        <div class="relative">
            <Dropdown is_visible>
                <DropdownItem label="Item-1" on_click=move |_e| {
                    last_action.set("item-1");
                    set_visible.set(false);
                }/>
                <DropdownItem label="Item-2" on_click=move |_e| {
                    last_action.set("item-2");
                    set_visible.set(false);
                }/>
            </Dropdown>
        </div>
        <Button on_click=move |_| {
            set_visible.set(true);
        }>Show dropdown</Button>

    }
}

#[generate_codeblock(DropdownButtonExample)]
#[component]
pub fn DropdownButtonDemo() -> impl IntoView {
    let last_action = RwSignal::new("/");

    view! {
        "Last action: " {move || format!("{:?}", last_action.get())}
        <DropdownButton alignment=AlignmentAnchor::BottomRight>
            // The button label
           <DropdownButtonChildren slot:button_children>DropDownButton</DropdownButtonChildren>
           <DropdownItem label="Entry-1" on_click=move |_e| {
               last_action.set("item-1");
           } />
           <DropdownItem label="Entry-2" on_click=move |_e| {
               last_action.set("item-2");
           } />
        </DropdownButton>
    }
}

#[component]
pub fn DropdownDemoPage() -> impl IntoView {
    view! {
        <Title text="Dropdown"/>

        <FixedCenterColumn>
            <Heading4 anchor="dropdown">"Dropdown"</Heading4>
            <DropdownExample />

            <Heading4 anchor="dropdown-button">"Dropdown Button"</Heading4>
            <DropdownButtonExample />

            <leptos_components::dropdown::DropdownDocs />
            <leptos_components::dropdown::DropdownItemDocs />
            <leptos_components::button::DropdownButtonDocs />
        </FixedCenterColumn>
    }
}
