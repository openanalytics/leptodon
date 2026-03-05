use leptodon::accordion::Accordion;
use leptodon::accordion::AccordionEntry;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
use leptodon::util::lorem::Lorem;
use leptodon_proc_macros::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_meta::Title;

#[generate_codeblock(AccordionExample)]
#[component]
pub fn AccordionDemo() -> impl IntoView {
    view! {
        <Accordion>
            <AccordionEntry title="An outer accordion">
                <Lorem sentences=4/>
                <Accordion class="mt-2">
                    <AccordionEntry title="Sub accordion">
                        <Lorem sentences=4/>
                    </AccordionEntry>
                </Accordion>
            </AccordionEntry>
        </Accordion>
    }
}

#[component]
pub fn AccordionDemoPage() -> impl IntoView {
    view! {
        <Title text="Accordion Component"/>

        <FixedCenterColumn>
            <Heading4 anchor="accordion">"Accordion"</Heading4>
            <AccordionExample />

            <leptodon::accordion::AccordionDocs />
            <leptodon::accordion::AccordionEntryDocs />
        </FixedCenterColumn>
    }
}
