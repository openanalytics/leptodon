use attr_docgen::generate_codeblock;
use leptodon::button::Button;
use leptodon::button::DropdownButton;
use leptodon::button::DropdownButtonChildren;
use leptodon::dropdown::AlignmentAnchor;
use leptodon::dropdown::Dropdown;
use leptodon::dropdown::DropdownItem;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::signal;
use leptos::{IntoView, component, view};
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

            <leptodon::dropdown::DropdownDocs />
            <leptodon::dropdown::DropdownItemDocs />
            <leptodon::button::DropdownButtonDocs />
        </FixedCenterColumn>
    }
}
