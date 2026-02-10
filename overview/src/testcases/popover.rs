use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
#[allow(unused)]
use leptos::prelude::IntoAnyAttribute;
use leptos::{IntoView, component, view};
use leptos_components::button::Button;
use leptos_components::popover::Popover;
use leptos_components::popover::PopoverAnchor;
use leptos_components::popover::PopoverTrigger;
use leptos_meta::Title;

#[component]
fn PopoverRow() -> impl IntoView {
    view! {
        <Popover preferred_pos=PopoverAnchor::Left>
            <PopoverTrigger slot>
                <Button>"L"</Button>
            </PopoverTrigger>
            <p>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Top>
            <PopoverTrigger slot>
                <Button>"T"</Button>
            </PopoverTrigger>
            <p>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Bottom>
            <PopoverTrigger slot>
                <Button>"B"</Button>
            </PopoverTrigger>
            <p>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
        <Popover preferred_pos=PopoverAnchor::Right>
            <PopoverTrigger slot>
                <Button>"R"</Button>
            </PopoverTrigger>
            <p>
                Lorem ipsum dolor sit amet consectetur adipiscing elit. Quisque faucibus ex sapien vitae pellentesque sem placerat. In id cursus mi pretium tellus duis convallis. Tempus leo eu aenean sed diam urna tempor. Pulvinar vivamus fringilla lacus nec metus bibendum egestas. Iaculis massa nisl malesuada lacinia integer nunc posuere. Ut hendrerit semper vel class aptent taciti sociosqu. Ad litora torquent per conubia nostra inceptos himenaeos.
            </p>
        </Popover>
    }
}

#[component]
pub fn TestPopover() -> impl IntoView {
    view! {
        <Title text="Test Popover"/>

        <div class="h-[300vh] w-[300vw] flex justify-center items-center">
            <div class="h-[100vh] w-[100vw] flex flex-col justify-between">
                <div class="flex flex-row justify-between">
                    <PopoverRow/>
                </div>
                <div class="flex flex-row justify-between">
                    <PopoverRow/>
                </div>
                <div class="flex flex-row justify-between">
                    <PopoverRow/>
                </div>
            </div>
        </div>
    }
}
