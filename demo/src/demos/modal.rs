use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::button::ButtonAppearance;
use leptos_components::button::ModalButton;
use leptos_components::button::ModalButtonChildren;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_components::modal::Modal;
use leptos_components::modal::ModalFooterChildren;
use leptos_components::util::lorem::Lorem;
use leptos_meta::Title;

#[generate_codeblock(ModalButtonExample)]
#[component]
pub fn ModalButtonDemo() -> impl IntoView {
    let last_action = RwSignal::new("");
    let modal_visible = RwSignal::new(false);
    view! {
        <p>
            "Shown: " {move || format!("{:?}", modal_visible.get())}
            <br/>
            "Last modal action: " { move || last_action.get()}
        </p>
        <ModalButton modal_title="Example modal?" modal_visible>
            <ModalButtonChildren slot:button_children>Toggle Modal</ModalButtonChildren>
            <ModalFooterChildren slot:modal_footer>
                <Button>Modal action 3</Button>
                <Button>Modal action 2</Button>
                <Button
                    appearance=ButtonAppearance::Primary
                    on_click=move |_| {
                        last_action.set("Action 1");
                        modal_visible.set(false);
                    }
                >Modal action 1</Button>
            </ModalFooterChildren>

            // Content
            <p class="leading-relaxed text-body">
                <Lorem sentences=1/>
            </p>
            <p class="leading-relaxed text-body">
                <Lorem offset=1 sentences=1/>
            </p>
        </ModalButton>
    }
}

#[generate_codeblock(ModalExample)]
#[component]
pub fn ModalDemo() -> impl IntoView {
    let visible = RwSignal::new(false);
    view! {
        <p>
            "Shown: " {move || format!("{:?}", visible.get())}
        </p>
        <Modal title="Example modal?" visible>
            <ModalFooterChildren slot:footer>
                <Button>Modal action 3</Button>
                <Button>Modal action 2</Button>
                <Button
                    appearance=ButtonAppearance::Primary
                    on_click=move |_| {
                        visible.set(false);
                    }
                >Modal action 1</Button>
            </ModalFooterChildren>

            // Content
            <Lorem sentences=2/>
        </Modal>
        <Button on_click=move |_| {
            visible.set(true);
        }>Open modal</Button>
    }
}

#[component]
pub fn ModalDemoPage() -> impl IntoView {
    view! {
        <Title text="Modal"/>

        <FixedCenterColumn>
            <Heading4 anchor="modal">"Modal"</Heading4>
            <ModalExample />

            <Heading4 anchor="modal-button">"Modal Button"</Heading4>
            <ModalButtonExample />

            <leptos_components::modal::ModalDocs />
            <leptos_components::button::ModalButtonDocs />
        </FixedCenterColumn>
    }
}
