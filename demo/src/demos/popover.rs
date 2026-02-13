use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
use leptos_components::popover::Popover;
use leptos_components::popover::PopoverAnchor;
use leptos_components::popover::PopoverTrigger;
use leptos_components::util::lorem::Lorem;
use leptos_meta::Title;

#[generate_codeblock(PopoverExample)]
#[component]
pub fn PopoverDemo() -> impl IntoView {
    view! {
        <Popover preferred_pos=PopoverAnchor::Left>
            <PopoverTrigger slot>
                <Button>"L"</Button>
            </PopoverTrigger>
            <p>
                <Lorem sentences=2/>
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Top>
            <PopoverTrigger slot>
                <Button>"T"</Button>
            </PopoverTrigger>
            <p>
                <Lorem sentences=4/>
            </p>
        </Popover>
    }
}

#[component]
pub fn PopoverDemoPage() -> impl IntoView {
    view! {
        <Title text="Popover"/>

        <FixedCenterColumn>
            <Heading4 anchor="popover">"Popover"</Heading4>
            <p>"Notice that the L popup likely does not appear on the left, since on most screens there is not enough space for the popover content.
            The popover component will automatically adjust its position to a more optimal side.
            Scrolling the popover into a wall is also supported."</p>
            <PopoverExample />

            <leptos_components::popover::PopoverDocs />
        </FixedCenterColumn>
    }
}
