use attr_docgen::generate_codeblock;
use leptos::prelude::ClassAttribute;
use leptos::prelude::ElementChild;
use leptos::{IntoView, component, view};
use leptos_components::accordion::Accordion;
use leptos_components::accordion::AccordionEntry;
use leptos_components::heading::Heading4;
use leptos_components::layout::FixedCenterColumn;
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

            <leptos_components::accordion::AccordionDocs />
            <leptos_components::accordion::AccordionEntryDocs />
        </FixedCenterColumn>
    }
}
