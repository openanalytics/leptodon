use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::prelude::Get;
use leptos::prelude::RwSignal;
use leptos::prelude::Set;
use leptos::prelude::Update;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::button::DialogButton;
use leptos_components::button::DialogButtonChildren;
use leptos_components::dialog::Dialog;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_components::util::callback::BoxCallback;
use leptos_components::util::lorem::Lorem;
use leptos_meta::Title;

#[generate_codeblock(DialogButtonExample)]
#[component]
pub fn DialogButtonDemo() -> impl IntoView {
    let dialog_visible = RwSignal::new(false);
    let yes_clickcount = RwSignal::new(0);
    let no_clickcount = RwSignal::new(0);
    view! {
        <p>
            "Shown: " {move || format!("{:?}", dialog_visible.get())}
            <br/>
            "Yes vs No: " { move || yes_clickcount.get()} " - " { move || no_clickcount.get()}
        </p>
        <DialogButton dialog_title="Example dialog?" dialog_visible
            primary_text="Yes"
            on_click_primary=BoxCallback::new(move || {
                yes_clickcount.update(|old| *old = *old + 1);
                dialog_visible.set(false);
            })
            secondary_text="No"
            on_click_secondary=BoxCallback::new(move || {
                no_clickcount.update(|old| *old = *old + 1);
                dialog_visible.set(false);
            })
        >
            <DialogButtonChildren slot:button_children>Toggle Dialog</DialogButtonChildren>

            // Content
            <p class="leading-relaxed text-body">
                <Lorem sentences=1/>
            </p>
            <p class="leading-relaxed text-body">
                <Lorem offset=1 sentences=1/>
            </p>
        </DialogButton>
    }
}


#[generate_codeblock(DialogExample)]
#[component]
pub fn DialogDemo() -> impl IntoView {
    let visible = RwSignal::new(false);
    view! {
        <p>
            "Shown: " {move || format!("{:?}", visible.get())}
        </p>
        <Dialog title="Example dialog?" visible
            on_click_primary=BoxCallback::new(move || {
                visible.set(false);
            })
            on_click_secondary=BoxCallback::new(move || {
                visible.set(false);
            })
        >
            // Content
            <Lorem sentences=2/>
        </Dialog>
        <Button on_click=move |_| {
            visible.set(true);
        }>Open dialog</Button>
    }
}

#[component]
pub fn DialogDemoPage() -> impl IntoView {
    view! {
        <Title text="Dialog"/>

        <FixedCenterColumn>
            <Heading4 anchor="dialog">"Dialog"</Heading4>
            <DialogExample />
            
            <Heading4 anchor="dialog-button">"Dialog Button"</Heading4>
            <DialogButtonExample />

            <leptos_components::dialog::DialogDocs />
            <leptos_components::button::DialogButtonDocs />
        </FixedCenterColumn>
    }
}
