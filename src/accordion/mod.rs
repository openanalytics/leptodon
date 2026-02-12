use crate::class_list;
use crate::util::shared_id::shared_id;
use attr_docgen::generate_docs;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::{IntoView, component, view};

const ACCORDION_HEADING_CLASSES: &str = "flex items-center justify-between w-full p-5 font-medium rtl:text-right text-body border border-t-0 border-x-0 border-b-default dark:border-gray-700 hover:text-heading gap-3 hover:bg-gray-50 hover:dark:bg-gray-900";
const ACCORDION_BODY_CLASSES: &str =
    "border dark:border-gray-700 border-s-0 border-e-0 border-t-0 border-b-default";

#[generate_docs]
/// Single collapseable section for the [Accordion]
#[component]
pub fn AccordionEntry(
    /// Header title that opens/closes the accordion entry upon a click.
    #[prop(into)]
    title: String,
    /// Normally a set of paragraphs with the text-body class.
    /// Or more accordions.
    children: Children,
) -> impl IntoView {
    let id = shared_id();
    let head_id = format!("accordion-collapse-heading-{id}");
    let body_id = format!("accordion-collapse-body-{id}");
    let (is_hidden, set_hidden) = signal(true);
    view! {
        <h2 id=head_id.clone()>
          <button type="button" class=class_list![
              ACCORDION_HEADING_CLASSES,
              ("bg-gray-50 dark:bg-gray-900", move || !is_hidden.get())
          ] aria-expanded="true" aria-controls=body_id.clone()
            on:click=move |_| {
                console_log(format!("Is_hidden: {}", is_hidden.get()).as_str());
                set_hidden.set(!is_hidden.get());
            }>
            <span>{ title }</span>
            <svg class=class_list![
                "w-5 h-5 shrink-0",
                ("rotate-180", move || is_hidden.get())
            ] aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24"><path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m5 15 7-7 7 7"/></svg>
          </button>
        </h2>
        <div id=body_id class=class_list![
            ACCORDION_BODY_CLASSES,
            ("hidden", move || is_hidden.get())
        ] aria-labelledby=head_id>
          <div class="p-4 md:p-5">
            { children() }
          </div>
        </div>
    }
}

#[generate_docs]
/// A stack of expandable/collapseable sections.
#[component]
pub fn Accordion<T>(
    /// A set accordion entries, see [AccordionEntry]
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    view! {
        <div id="accordion-collapse" data-accordion="collapse" class="rounded-lg border border-default dark:border-gray-700 overflow-hidden shadow-sm">
            { children.into_inner()() }
        </div>
    }
}
