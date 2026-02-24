use attr_docgen::generate_codeblock;
use leptodon::accordion::Accordion;
use leptodon::accordion::AccordionEntry;
use leptodon::heading::Heading4;
use leptodon::layout::FixedCenterColumn;
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
                <p>Est eveniet aut necessitatibus sunt accusantium mollitia accusantium. Nihil aperiam est dolor numquam. Incidunt qui cum sapiente distinctio deleniti quisquam. Asperiores ea sint voluptas et eum reiciendis. Et quos quasi aspernatur voluptatum eos id. Rerum quaerat suscipit cupiditate.</p>
                <Accordion>
                    <AccordionEntry title="Sub accordion">
                        <p>Est eveniet aut necessitatibus sunt accusantium mollitia accusantium. Nihil aperiam est dolor numquam. Incidunt qui cum sapiente distinctio deleniti quisquam. Asperiores ea sint voluptas et eum reiciendis. Et quos quasi aspernatur voluptatum eos id. Rerum quaerat suscipit cupiditate.</p>
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
